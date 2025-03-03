//! Handles parsing of TidalCycles pattern responses.

pub mod event;
pub mod value;

pub use crate::event::Event;
use serde::{Deserialize, Serialize};
use serde_json;

/// Represents a parsed pattern response from TidalCycles.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Pattern {
    #[serde(rename = "arcLen")]
    pub arc_len: f64,
    pub events: Vec<Event>,
}

pub fn parse_json_pattern(json: String) -> Result<Pattern, String> {
    match serde_json::from_str::<Pattern>(&json) {
        Ok(pattern) => Ok(pattern),
        Err(e) => Err(format!("failed to parse pattern json: {}", e)),
    }
}
