use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

use anyhow::Result;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    SerdeSerialize,
    SerdeDeserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
pub struct GrammarIssue {
    pub start: usize,
    pub end: usize,
    pub message: String,
    pub priority: u8,
    pub replacement: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarperGrammarChecker {
}

impl HarperGrammarChecker {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn analyze(&self, _text: &str) -> Result<Vec<GrammarIssue>> {
        Ok(Vec::new())
    }

    pub fn correct(&self, text: &str) -> Result<String> {
        Ok(text.to_string())
    }
}

impl Default for HarperGrammarChecker {
    fn default() -> Self {
        Self::new()
    }
}
