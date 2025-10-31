# CCS - Claude Code Statusline

A [Claude Code](https://docs.anthropic.com/en/docs/claude-code/) statusline tool that displays session information including model, directory, git status, token usage, and code changes.

No configuration is the feature. Use [sirmalloc/ccstatusline](https://github.com/sirmalloc/ccstatusline/) if you need flexibility. I wrote this one for fun and quick access. While I don't expect any issues or pull requests, you're welcome to fork it and modify it as you see fit.

## Installation

```console
$ cargo install --git https://github.com/0x6b/ccs
```

## Usage

Add a `statusLine` command to your `.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.cargo/bin/ccs"
  }
}
```

See [Status line configuration - Claude Docs](https://docs.claude.com/en/docs/claude-code/statusline) for details.

### Input Format

The tool expects JSON input with the following structure (all fields are optional):

```json
{
  "model": {
    "display_name": "Sonnet 4.5"
  },
  "workspace": {
    "current_dir": "/path/to/project"
  },
  "cwd": "/path/to/project",
  "transcript_path": "/path/to/transcript.jsonl"
}
```

Notes:

- `workspace.current_dir` takes precedence over `cwd` if both are provided
- `transcript_path` is used to read token usage from Claude Code transcript files
- All fields gracefully degrade to defaults if missing

## Output Format

```
dir@branch · X modified, Y untracked, +added -removed lines · tokens (percentage%) · Model
```

### Components

- Directory: Current directory name
- Branch: Git branch name with status-based coloring:
  - Green: Clean working tree
  - Yellow: Staged files present
  - Red: Modified or untracked files present
- Changes: File counts ("X modified", "Y untracked") with line changes (green for additions, red for deletions). Untracked files don't contribute to line counts
- Tokens: Token count with K/M suffix, with percentage in parentheses
  - Percentage color: green < 70% < yellow < 90% < red
  - Percentage calculated against compaction threshold (160K tokens = 200K × 0.8)
- Model: Model name at the end (white)

### Examples

With git changes:

```
ccs@main · 2 modified, 1 untracked, +47 -32 lines · 113.5K (71%) · Sonnet 4.5
```

Only modified files:

```
ccs@main · 2 modified, +47 -32 lines · 0 (0%) · Sonnet 4.5
```

Clean repository:

```
ccs@main · +0 -0 lines · 0 (0%) · Sonnet 4.5
```

Non-git directory:

```
tmp · +0 -0 lines · 0 (0%) · Sonnet 4.5
```

With staged changes (branch shown in yellow):

```
ccs@main · 3 modified, +120 -45 lines · 87.2K (54%) · Sonnet 4.5
```

## License

MIT. See [LICENSE](./LICENSE) for details.
