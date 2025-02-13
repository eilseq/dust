use event_handling::{Arc, NoteEvent, NoteValue, SoundEvent};

#[test]
fn test_arc_creation() {
    let arc = Arc {
        start: 0.0,
        stop: 1.0,
    };
    assert_eq!(arc.start, 0.0);
    assert_eq!(arc.stop, 1.0);
}

#[test]
fn test_sound_event_parsing() {
    let event = SoundEvent::from_value("hh:3");
    assert_eq!(event.sound, "hh");
    assert_eq!(event.index, 3);

    let event = SoundEvent::from_value("bd");
    assert_eq!(event.sound, "bd");
    assert_eq!(event.index, 0);
}

#[test]
fn test_note_parsing() {
    let note = NoteValue::from_str("2.0n (d5)").unwrap();
    assert_eq!(note.note_number, 2.0);
    assert_eq!(note.scale_note, "d5");
    assert_eq!(note.midi_pitch, 62);
}

#[test]
fn test_note_event_creation() {
    let note_value = NoteValue::from_str("1.0n (cs5)").unwrap();
    let note_event = NoteEvent {
        whole: Some(Arc {
            start: 0.0,
            stop: 1.0,
        }),
        part: Arc {
            start: 0.0,
            stop: 0.5,
        },
        note: note_value,
    };

    assert_eq!(note_event.note.note_number, 1.0);
    assert_eq!(note_event.note.scale_note, "cs5");
    assert_eq!(note_event.note.midi_pitch, 61);
}
