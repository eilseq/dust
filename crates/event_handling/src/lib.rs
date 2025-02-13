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

/// Enum for different event types.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Event {
    Sound(SoundEvent),
    Note(NoteEvent),
}
