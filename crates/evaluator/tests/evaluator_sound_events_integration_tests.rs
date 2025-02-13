use evaluator::{eval_pattern, setup, teardown};
use event_handling::Event;

#[test]
fn test_multiple_sound_events() {
    setup(); // Initialize RTS

    let pattern = "s $ \"bd hh\"";
    let arc_length = "1.0";

    let result = eval_pattern(pattern, arc_length);
    assert!(result.is_ok());

    let event_queue = result.unwrap();
    assert_eq!(event_queue.len(), 2);

    match &event_queue[0] {
        Event::Sound(sound_event) => {
            assert_eq!(sound_event.sound, "bd");
            assert_eq!(sound_event.index, 0);
        }
        _ => panic!("Expected SoundEvent"),
    }

    match &event_queue[1] {
        Event::Sound(sound_event) => {
            assert_eq!(sound_event.sound, "hh");
            assert_eq!(sound_event.index, 0);
        }
        _ => panic!("Expected SoundEvent"),
    }

    teardown(); // Decrement counter & possibly shut down RTS
}
