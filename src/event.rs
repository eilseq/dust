//! Defines core event structures: SoundEvent and NoteEvent.
//! Implements parsing logic from TidalCycles JSON.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an event's active duration (start & stop time).
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Arc {
    pub start: f64,
    pub stop: f64,
}

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

/// Represents a sound event (sample trigger).
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundEvent {
    pub whole: Option<Arc>,
    pub part: Arc,
    pub sound: String,
    pub index: u32,
}

impl SoundEvent {
    /// Parses a sound value, extracting name and index (e.g., "hh:1" → `"hh", 1`).
    pub fn from_value(value: &str) -> Self {
        let cleaned_sound = value.trim().to_string();
        let (sound_name, index) = if let Some((name, idx)) = cleaned_sound.split_once(':') {
            (name.to_string(), idx.parse::<u32>().unwrap_or(0))
        } else {
            (cleaned_sound.clone(), 0)
        };

        SoundEvent {
            whole: None,
            part: Arc {
                start: 0.0,
                stop: 1.0,
            },
            sound: sound_name,
            index,
        }
    }
}

/// Represents a note event.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NoteEvent {
    pub whole: Option<Arc>,
    pub part: Arc,
    pub note: NoteValue,
}

/// Enum for different event types.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Event {
    Sound(SoundEvent),
    Note(NoteEvent),
}

/// Represents TidalCycles' JSON response format.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Pattern {
    Success {
        #[serde(rename = "arcLen")]
        arc_len: f64,
        events: Vec<RawEvent>,
    },
    Failure {
        error: String,
    },
}

/// Raw event struct before parsing into structured event.
#[derive(Debug, Deserialize, Serialize)]
pub struct RawEvent {
    pub whole: Option<Arc>,
    pub part: Arc,
    pub value: HashMap<String, String>,
}

impl RawEvent {
    /// Converts `RawEvent` into structured `Event`.
    pub fn to_event(self) -> Event {
        if let Some(note_str) = self.value.get("n") {
            if let Some(parsed_note) = NoteValue::from_str(note_str) {
                return Event::Note(NoteEvent {
                    whole: self.whole,
                    part: self.part,
                    note: parsed_note,
                });
            }
        } else if let Some(sound_str) = self.value.get("s") {
            return Event::Sound(SoundEvent::from_value(sound_str));
        }

        Event::Sound(SoundEvent::from_value("unknown"))
    }
}
