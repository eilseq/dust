//! Handles parsing of TidalCycles pattern responses.

use crate::arc::Arc;
use crate::note::{NoteEvent, NoteValue};
use crate::sound::SoundEvent;
use crate::Event;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a parsed pattern response from TidalCycles.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Pattern {
    Success {
        #[serde(rename = "arcLen")]
        arc_len: f64,
        events: Vec<JSONEvent>, // ✅ Correct field name
    },
    Failure {
        error: String,
    },
}

impl Pattern {
    /// Converts `Pattern` into structured events.
    pub fn to_events(self) -> Result<Vec<Event>, String> {
        match self {
            Pattern::Success { events, .. } => {
                Ok(events.into_iter().map(|event| event.to_event()).collect())
            }
            Pattern::Failure { error } => Err(error),
        }
    }
}

/// Represents an individual event inside `Pattern::Success`.
#[derive(Debug, Deserialize, Serialize)]
pub struct JSONEvent {
    pub part: Arc,
    pub whole: Option<Arc>,
    pub value: HashMap<String, serde_json::Value>,
}

impl JSONEvent {
    /// Converts `JSONEvent` into structured `Event`.
    pub fn to_event(self) -> Event {
        if let Some(note_value) = self.value.get("n") {
            let note_str = match note_value {
                serde_json::Value::Number(num) => num.to_string(),
                serde_json::Value::String(s) => s.clone(),
                _ => return Event::Sound(SoundEvent::from_value("unknown")),
            };

            if let Some(parsed_note) = NoteValue::from_str(&note_str) {
                return Event::Note(NoteEvent {
                    whole: self.whole,
                    part: self.part,
                    note: parsed_note,
                });
            }
        } else if let Some(sound_str) = self.value.get("s") {
            if let Some(sound) = sound_str.as_str() {
                return Event::Sound(SoundEvent::from_value(sound));
            }
        }
        Event::Sound(SoundEvent::from_value("unknown"))
    }
}
