use evaluator::{eval_pattern, setup, teardown};
use event_handling::Event;

#[test]
fn test_multiple_note_events() {
    setup(); // Initialize RTS

    let pattern = "n $ \"1 2 3 4\"";
    let arc_length = "1.0";

    let result = eval_pattern(pattern, arc_length);
    assert!(result.is_ok());

    let event_queue = result.unwrap();
    assert_eq!(event_queue.len(), 4);

    let expected_notes = [
        (1.0, "cs5", 61),
        (2.0, "d5", 62),
        (3.0, "ds5", 63),
        (4.0, "e5", 64),
    ];

    for (i, (note_number, scale_note, midi_pitch)) in expected_notes.iter().enumerate() {
        match &event_queue[i] {
            Event::Note(note_event) => {
                assert_eq!(note_event.note.note_number, *note_number);
                assert_eq!(note_event.note.scale_note, *scale_note);
                assert_eq!(note_event.note.midi_pitch, *midi_pitch);
            }
            _ => panic!("Expected NoteEvent"),
        }
    }

    teardown(); // Decrement counter & possibly shut down RTS
}
