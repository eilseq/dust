//! Defines NoteEvent and NoteValue parsing.

use crate::arc::Arc;

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Represents a parsed musical note value.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NoteValue {
    pub note_number: f64,
    pub scale_note: String,
    pub midi_pitch: u8,
}

impl NoteValue {
    /// Parses a note string like `"1.0n (cs5)"` into `NoteValue`.
    pub fn from_str(note_str: &str) -> Option<Self> {
        let re = Regex::new(r#"(?P<number>[\d.]+)n \((?P<note>[a-g][s]?\d)\)"#).ok()?;
        let captures = re.captures(note_str)?;

        let note_number = captures.name("number")?.as_str().parse::<f64>().ok()?;
        let scale_note = captures.name("note")?.as_str().to_string();
        let midi_pitch = (60.0 + note_number) as u8;

        Some(NoteValue {
            note_number,
            scale_note,
            midi_pitch,
        })
    }
}

/// Represents a note event.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NoteEvent {
    pub whole: Option<Arc>,
    pub part: Arc,
    pub note: NoteValue,
}
