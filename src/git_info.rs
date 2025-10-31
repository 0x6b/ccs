use anyhow::{Context, Result};
use git2::{Diff, Repository, StatusOptions};

#[derive(Debug)]
pub struct GitInfo {
    pub branch: String,
    pub files_staged: usize,
    pub files_modified: usize,
    pub files_untracked: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
}

impl Default for GitInfo {
    fn default() -> Self {
        Self {
            branch: "unknown".to_string(),
            files_staged: 0,
            files_modified: 0,
            files_untracked: 0,
            lines_added: 0,
            lines_removed: 0,
        }
    }
}

impl GitInfo {
    pub fn try_new(dir: &str) -> Result<Self> {
        let repo = Repository::discover(dir).context("Not a git repository")?;

        let branch = repo
            .head()
            .ok()
            .and_then(|h| h.shorthand().map(str::to_string))
            .or_else(|| {
                repo.find_reference("HEAD").ok().and_then(|r| {
                    r.symbolic_target()
                        .and_then(|t| t.strip_prefix("refs/heads/"))
                        .map(str::to_string)
                })
            })
            .unwrap_or_else(|| "unknown".to_string());

        let statuses = repo
            .statuses(Some(StatusOptions::new().include_untracked(true).include_ignored(false)))
            .context("Failed to get git status")?;

        let (files_staged, files_modified, files_untracked) =
            statuses.iter().fold((0, 0, 0), |(s, m, u), e| {
                let status = e.status();
                let has_staged = status.is_index_new()
                    || status.is_index_modified()
                    || status.is_index_deleted()
                    || status.is_index_renamed()
                    || status.is_index_typechange();
                let has_unstaged_changes = status.is_wt_modified() || status.is_wt_deleted();
                let is_untracked = status.is_wt_new();

                (
                    if has_staged { s + 1 } else { s },
                    if has_unstaged_changes { m + 1 } else { m },
                    if is_untracked { u + 1 } else { u },
                )
            });

        let get_diff_stats =
            |diff: Diff| diff.stats().ok().map(|s| (s.insertions(), s.deletions()));

        let (lines_added, lines_removed) = repo
            .head()
            .ok()
            .and_then(|h| h.peel_to_tree().ok())
            .and_then(|t| repo.diff_tree_to_index(Some(&t), None, None).ok())
            .and_then(get_diff_stats)
            .into_iter()
            .chain(repo.diff_index_to_workdir(None, None).ok().and_then(get_diff_stats))
            .fold((0, 0), |(a, r), (ia, ir)| (a + ia, r + ir));

        Ok(Self {
            branch,
            files_staged,
            files_modified,
            files_untracked,
            lines_added,
            lines_removed,
        })
    }
}
