//! Defines the base Event structure and specialized SoundEvent & NoteEvent.

use crate::value::{NoteValue, SoundValue};
use serde::{Deserialize, Serialize};

/// Represents a sound event, specializing BaseEvent.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundEvent {
    pub value: SoundValue,

    #[serde(flatten)]
    pub timing: Timing,
}

/// Represents a note event, specializing BaseEvent.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NoteEvent {
    pub value: NoteValue,

    #[serde(flatten)]
    pub timing: Timing,
}

/// Represents shared properties for all events.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Timing {
    pub whole: Arc,
    pub part: Arc,
}

/// Represents an event's active duration (start & stop time).
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Arc {
    pub start: f64,
    pub stop: f64,
}
