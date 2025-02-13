//! Main module for event structures.

mod arc;
mod note;
mod pattern;
mod sound;

pub use arc::Arc;
pub use note::NoteEvent;
pub use note::NoteValue;
pub use pattern::Pattern;
pub use sound::SoundEvent;

use serde::{Deserialize, Serialize};
use serde_json;

/// Enum for different event types.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Event {
    Sound(SoundEvent),
    Note(NoteEvent),
}

/// Parses a JSON string into structured events or an error.
pub fn parse_tidal_json(result: &str) -> Result<Vec<Event>, String> {
    match serde_json::from_str::<Pattern>(result) {
        Ok(pattern) => pattern
            .to_events()
            .map_err(|e| format!("Tidal error: {}", e)),
        Err(e) => Err(format!("Evaluator error: {}", e)),
    }
}
