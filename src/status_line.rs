use std::{
    convert::TryFrom,
    fmt,
    fmt::{Display, Formatter},
    path::Path,
};

use anyhow::{Context, Error, Result};
use owo_colors::OwoColorize;
use serde_json::from_str;

use crate::{GitInfo, Input, TokenCount, styled, styled_by};

pub struct StatusLine {
    dir: String,
    git_info: GitInfo,
    total_tokens: TokenCount,
    model: String,
}

impl StatusLine {
    const COMPACTION_THRESHOLD: f64 = 200000.0 * 0.8;
}

impl TryFrom<&str> for StatusLine {
    type Error = Error;

    fn try_from(json: &str) -> Result<Self> {
        let input: Input = from_str(json).context("Failed to parse JSON input")?;
        input.try_into()
    }
}

impl TryFrom<Input> for StatusLine {
    type Error = Error;

    fn try_from(data: Input) -> Result<Self> {
        let current_dir_full = data
            .workspace
            .and_then(|w| w.current_dir)
            .or(data.cwd)
            .unwrap_or_else(|| ".".to_string());

        let dir = Path::new(&current_dir_full)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_string();

        let git_info = GitInfo::try_new(&current_dir_full).unwrap_or_default();

        let total_tokens = data
            .transcript_path
            .as_deref()
            .map(TokenCount::from)
            .unwrap_or_default();

        let model = data
            .model
            .and_then(|m| m.display_name)
            .unwrap_or_else(|| "Unknown".to_string());

        Ok(Self { dir, git_info, total_tokens, model })
    }
}

impl Display for StatusLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        styled!(f, &self.dir, white, dimmed)?;
        styled!(f, "@", white, dimmed)?;
        styled_by!(f, &self.git_info.branch;
            self.git_info.files_staged > 0 => yellow(dimmed),
            self.git_info.files_modified > 0 || self.git_info.files_untracked > 0 => red(dimmed),
            true => green(dimmed)
        )?;

        styled!(f, " · ", white, dimmed)?;

        match (self.git_info.files_modified, self.git_info.files_untracked) {
            (0, 0) => {}
            (m, 0) => styled!(f, format!("{m} modified, "), dimmed)?,
            (0, u) => styled!(f, format!("{u} untracked, "), dimmed)?,
            (m, u) => styled!(f, format!("{m} modified, {u} untracked, "), dimmed)?,
        }
        styled!(f, format!("+{}", self.git_info.lines_added), green, dimmed)?;
        write!(f, " ")?;
        styled!(f, format!("-{}", self.git_info.lines_removed), red, dimmed)?;
        styled!(f, " lines", white, dimmed)?;

        styled!(f, " · ", white, dimmed)?;

        let pct = ((*self.total_tokens as f64 / Self::COMPACTION_THRESHOLD) * 100.0)
            .min(100.0)
            .round() as u32;
        styled!(f, format!("{} ", self.total_tokens), white, dimmed)?;
        styled!(f, "(", white, dimmed)?;
        styled_by!(f, format!("{pct}%");
            pct >= 90 => red(dimmed),
            pct >= 70 => yellow(dimmed),
            true => green(dimmed)
        )?;
        styled!(f, ")", white, dimmed)?;
        styled!(f, " · ", white, dimmed)?;

        styled!(f, &self.model, white, dimmed)
    }
}
