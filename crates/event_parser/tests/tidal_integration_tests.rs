use embedded_tidal_core::{eval_pattern, setup, teardown};
use event_parser::{parse_json_pattern, Event};

#[test]
fn test_evaluator_stack_mixed_events() {
    setup(); // Initialize RTS

    let pattern = "stack [s \"bd bd bd\" # gain 0.8 # cutoff 4000 # pan 0.2, n \"c5 e5 g5\" # gain 0.7 # cutoff 2000 # pan 0.5]";
    let arc_length = "1.0";

    let result = eval_pattern(pattern, arc_length).unwrap();
    let pattern = parse_json_pattern(result).unwrap();

    assert_eq!(pattern.arc_len, 1.0);
    assert!(!pattern.events.is_empty());

    let mut sound_count = 0;
    let mut note_count = 0;
    let expected_midi = vec![60, 64, 67]; // MIDI values for C5, E5, G5

    for event in &pattern.events {
        match event {
            Event::Sound(sound_event) => {
                assert_eq!(sound_event.value.s, "bd");
                assert_eq!(sound_event.value.controls.gain, Some(0.8));
                assert_eq!(sound_event.value.controls.cutoff, Some(4000.0));
                assert_eq!(sound_event.value.controls.pan, Some(0.2));
                sound_count += 1;
            }
            Event::Note(note_event) => {
                let midi_pitch = note_event
                    .value
                    .get_midi_pitch()
                    .expect("Failed to get MIDI pitch");
                assert!(
                    expected_midi.contains(&midi_pitch),
                    "Unexpected MIDI pitch: {}",
                    midi_pitch
                );

                // Validate control parameters
                assert_eq!(note_event.value.controls.gain, Some(0.7));
                assert_eq!(note_event.value.controls.cutoff, Some(2000.0));
                assert_eq!(note_event.value.controls.pan, Some(0.5));

                note_count += 1;
            }
        }
    }

    assert!(sound_count > 0, "Expected sound events in the pattern");
    assert!(note_count > 0, "Expected note events in the pattern");

    teardown();
}
