use event_parser::Pattern;
use serde_json;

/// ✅ **Test: Parsing a valid Sound Event with `:` (n in SoundValue)**
#[test]
fn test_parse_sound_event_with_colon() {
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
                        "cutoff": 4000.0
                    }
                }
            ]
        }
        "#;

    let pattern: Pattern = serde_json::from_str(json).unwrap();

    match pattern {
        Pattern::PatternSound { arc_len, events } => {
            assert_eq!(arc_len, 1.0);
            assert_eq!(events.len(), 1);

            let event = &events[0];

            assert_eq!(event.timing.whole.start, 0.0);
            assert_eq!(event.timing.whole.stop, 0.5);

            assert_eq!(event.timing.part.start, 0.0);
            assert_eq!(event.timing.part.stop, 0.5);

            assert_eq!(event.value.s, "bd");
            assert_eq!(event.value.n, Some(3));

            assert_eq!(event.value.controls.gain, Some(0.8));
            assert_eq!(event.value.controls.cutoff, Some(4000.0));
        }
        _ => panic!("Expected a Success pattern"),
    }
}
#[test]
fn test_parse_note_event_with_cutoff_and_gain() {
    let json = r#"
        {
            "arcLen": 1,
            "events": [
                {
                    "whole": { "start": 0, "stop": 0.5 },
                    "part": { "start": 0, "stop": 0.5 },
                    "value": {
                        "n": "-9.0n (ds4)",
                        "gain": 0.8,
                        "cutoff": 2000.0
                    }
                }
            ]
        }
        "#;

    let pattern: Pattern = serde_json::from_str(json).unwrap();

    match pattern {
        Pattern::PatternNote { arc_len, events } => {
            assert_eq!(arc_len, 1.0);
            assert_eq!(events.len(), 1);

            let event = &events[0];

            assert_eq!(event.timing.whole.start, 0.0);
            assert_eq!(event.timing.whole.stop, 0.5);

            assert_eq!(event.timing.part.start, 0.0);
            assert_eq!(event.timing.part.stop, 0.5);

            assert_eq!(event.value.n, "-9.0n (ds4)");
            assert_eq!(event.value.get_midi_pitch(), Some(51));

            assert_eq!(event.value.controls.gain, Some(0.8));
            assert_eq!(event.value.controls.cutoff, Some(2000.0));
        }
        _ => panic!("Expected a Note pattern"),
    }
}

#[test]
fn test_parse_failure_response() {
    let json = r#"
        {
            "error": "Pattern could not be parsed"
        }
        "#;

    let pattern: Pattern = serde_json::from_str(json).unwrap();

    match pattern {
        Pattern::Failure { error } => {
            assert_eq!(error, "Pattern could not be parsed");
        }
        _ => panic!("Expected a Failure pattern"),
    }
}

#[test]
fn test_pattern_serialization() {
    let json = r#"
        {
            "arcLen": 1,
            "events": [
                {
                    "whole": { "start": 0, "stop": 0.5 },
                    "part": { "start": 0, "stop": 0.5 },
                    "value": {
                        "s": "snare",
                        "n": 2,
                        "gain": 0.9,
                        "cutoff": 3500.0
                    }
                }
            ]
        }
        "#;

    let pattern: Pattern = serde_json::from_str(json).unwrap();
    let json_output = serde_json::to_string_pretty(&pattern).unwrap();

    let pattern_roundtrip: Pattern = serde_json::from_str(&json_output).unwrap();
    assert_eq!(pattern, pattern_roundtrip);
}
