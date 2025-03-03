use event_parser::{parse_json_pattern, Event};

#[test]
fn test_parse_mixed_events() {
    let json = r#"
        {
            "arcLen": 1,
            "events": [
                {
                    "whole": { "start": 0, "stop": 0.5 },
                    "part": { "start": 0, "stop": 0.5 },
                    "value": {
                        "s": "bd",
                        "n": 3,
                        "gain": 0.8,
                        "cutoff": 4000.0,
                        "pan": 0.2
                    }
                },
                {
                    "whole": { "start": 0.5, "stop": 1.0 },
                    "part": { "start": 0.5, "stop": 1.0 },
                    "value": {
                        "n": "-9.0n (ds4)",
                        "gain": 0.7,
                        "cutoff": 2000.0,
                        "pan": 0.5
                    }
                }
            ]
        }
    "#;

    let pattern = parse_json_pattern(json.to_string()).unwrap();
    assert_eq!(pattern.arc_len, 1.0);
    assert_eq!(pattern.events.len(), 2);

    for event in &pattern.events {
        match event {
            Event::Sound(sound_event) => {
                assert_eq!(sound_event.value.s, "bd");
                assert_eq!(sound_event.value.n, Some(3));

                // Check controls
                assert_eq!(sound_event.value.controls.gain, Some(0.8));
                assert_eq!(sound_event.value.controls.cutoff, Some(4000.0));
                assert_eq!(sound_event.value.controls.pan, Some(0.2));
            }
            Event::Note(note_event) => {
                assert_eq!(note_event.value.n, "-9.0n (ds4)");

                // Validate MIDI pitch
                let midi_pitch = note_event
                    .value
                    .get_midi_pitch()
                    .expect("Failed to get MIDI pitch");
                assert_eq!(midi_pitch, 51); // ds4 → MIDI 51

                // Check controls
                assert_eq!(note_event.value.controls.gain, Some(0.7));
                assert_eq!(note_event.value.controls.cutoff, Some(2000.0));
                assert_eq!(note_event.value.controls.pan, Some(0.5));
            }
        }
    }
}
