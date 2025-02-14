//! Defines the base Value structure and specialized SoundValue & NoteValue.

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundValue {
    pub s: String,
    pub n: Option<u32>,

    #[serde(flatten)]
    pub controls: Controls,
}

/// Represents a parsed musical note value.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NoteValue {
    pub n: String,

    #[serde(flatten)]
    pub controls: Controls,
}

impl NoteValue {
    pub fn get_midi_pitch(&self) -> Option<u8> {
        let re =
            Regex::new(r"(?P<number>[-+]?\d+(\.\d+)?)n?\s*\((?P<note>[a-gA-G][s#]?\d)\)").unwrap();
        let captures = re.captures(&self.n)?;

        let note_number = captures.name("number")?.as_str().parse::<f64>().ok()?;
        let midi_pitch = (60.0 + note_number) as u8;

        Some(midi_pitch)
    }
}

/// Represents shared properties for all values.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Controls {
    // Common TidalCycles parameters
    pub gain: Option<f64>,
    pub cutoff: Option<f64>,
    pub resonance: Option<f64>,
    pub pan: Option<f64>,

    // Pitch effects
    pub fshift: Option<f64>,
    pub fshiftnote: Option<f64>,
    pub fshiftphase: Option<f64>,

    // Ring modulation
    pub ring: Option<f64>,
    pub ringf: Option<f64>,
    pub ringdf: Option<f64>,

    // Tremolo
    pub tremolodepth: Option<f64>,
    pub tremolorate: Option<f64>,

    // Delay
    pub delay: Option<f64>,
    pub delaytime: Option<f64>,
    pub delayfeedback: Option<f64>,
    pub lock: Option<f64>,

    // Reverb
    pub room: Option<f64>,
    pub size: Option<f64>,

    // Leslie
    pub leslie: Option<f64>,
    pub lrate: Option<f64>,
    pub lsize: Option<f64>,

    // Phaser
    pub phaserrate: Option<f64>,
    pub phaserdepth: Option<f64>,

    // Spectral delay
    pub xsdelay: Option<f64>,
    pub tsdelay: Option<f64>,

    // Freeze
    pub freeze: Option<f64>,

    // ASR Envelope
    pub attack: Option<f64>,
    pub hold: Option<f64>,
    pub release: Option<f64>,

    // Legato
    pub legato: Option<f64>,

    // Filters
    pub djf: Option<f64>,
    pub hcutoff: Option<f64>,
    pub hresonance: Option<f64>,
    pub bandf: Option<f64>,
    pub bandq: Option<f64>,
    pub vowel: Option<String>,
    pub comb: Option<f64>,
    pub hbrick: Option<f64>,
    pub lbrick: Option<f64>,

    // Distortion
    pub distort: Option<f64>,
    pub triode: Option<f64>,
    pub shape: Option<f64>,
    pub squiz: Option<f64>,

    // Bit manipulation
    pub binshift: Option<f64>,
    pub scram: Option<f64>,
    pub crush: Option<f64>,
    pub coarse: Option<f64>,
    pub waveloss: Option<f64>,
    pub krush: Option<f64>,
    pub kcutoff: Option<f64>,

    // Spectral effects
    pub smear: Option<f64>,
    pub enhance: Option<f64>,
    pub real: Option<f64>,
    pub imag: Option<f64>,
}
