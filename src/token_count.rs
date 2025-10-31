use std::{fmt::{Display, Formatter, Result}, fs::File, io::{BufRead, BufReader}, ops::Deref};
use crate::TranscriptEntry;

#[derive(Default)]
pub struct TokenCount(u64);

impl Deref for TokenCount {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for TokenCount {
    fn from(transcript_path: &str) -> Self {
        let usage = File::open(transcript_path).ok().and_then(|file|
            BufReader::new(file).lines()
                .filter_map(|line| line.ok()?.try_into().ok())
                .filter(TranscriptEntry::is_assistant)
                .filter_map(TranscriptEntry::usage)
                .last()
        );
        Self(usage.unwrap_or(0))
    }
}

impl Display for TokenCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            1_000_000.. => write!(f, "{:.1}M", self.0 as f64 / 1_000_000.0),
            1_000.. => write!(f, "{:.1}K", self.0 as f64 / 1_000.0),
            _ => write!(f, "{}", self.0),
        }
    }
}
