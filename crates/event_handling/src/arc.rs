//! Defines the Arc struct, representing time spans in TidalCycles.

use serde::{Deserialize, Serialize};

/// Represents an event's active duration (start & stop time).
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Arc {
    pub start: f64,
    pub stop: f64,
}
