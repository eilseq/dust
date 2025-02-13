//! Defines SoundEvent parsing.

use crate::arc::Arc;

use serde::{Deserialize, Serialize};

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
