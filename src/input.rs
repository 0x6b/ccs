use anyhow::{Context, Error, Result};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub model: Option<Model>,
    pub workspace: Option<Workspace>,
    pub cwd: Option<String>,
    pub transcript_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub current_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct TranscriptEntry {
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    pub message: Option<Message>,
}

impl TranscriptEntry {
    pub fn is_assistant(&self) -> bool {
        self.entry_type.as_deref() == Some("assistant")
    }

    pub fn usage(self) -> Option<u64> {
        self.message?.usage.map(|usage| {
            usage.input_tokens.unwrap_or(0)
                + usage.output_tokens.unwrap_or(0)
                + usage.cache_creation_input_tokens.unwrap_or(0)
                + usage.cache_read_input_tokens.unwrap_or(0)
        })
    }
}

impl TryFrom<String> for TranscriptEntry {
    type Error = Error;

    fn try_from(line: String) -> Result<Self> {
        from_str(&line).context("Failed to parse transcript entry")
    }
}
