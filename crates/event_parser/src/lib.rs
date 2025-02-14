//! Handles parsing of TidalCycles pattern responses.

pub mod event;
pub mod value;

use crate::event::{NoteEvent, SoundEvent};
use serde::{Deserialize, Serialize};
use serde_json;

/// Represents a parsed pattern response from TidalCycles.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Pattern {
    PatternSound {
        #[serde(rename = "arcLen")]
        arc_len: f64,
        events: Vec<SoundEvent>,
    },
    PatternNote {
        #[serde(rename = "arcLen")]
        arc_len: f64,
        events: Vec<NoteEvent>,
    },
    Failure {
        error: String,
    },
}

pub fn parse_json_pattern(json: String) -> Result<Pattern, String> {
    match serde_json::from_str::<Pattern>(&json) {
        Ok(pattern) => Ok(pattern),
        Err(e) => Err(format!("failed to parse pattern json: {}", e)),
    }
}
