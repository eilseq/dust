use evaluator::{eval_pattern, setup, teardown};
use event_parser::{parse_json_pattern, Pattern};

#[test]
fn test_multiple_sound_events() {
    setup(); // Initialize RTS

    let pattern = "s $ \"bd bd bd bd\"";
    let arc_length = "1.0";

    let result = eval_pattern(pattern, arc_length);
    assert!(result.is_ok());

    let json_result = result.unwrap();
    let pattern = parse_json_pattern(json_result).expect("Failed to parse pattern JSON");

    match pattern {
        Pattern::PatternSound { arc_len, events } => {
            assert_eq!(arc_len, 1.0);
            assert_eq!(events.len(), 4);

            for event in events {
                assert_eq!(event.value.s, "bd");
            }
        }
        Pattern::PatternNote { .. } => {
            panic!("Pattern is supposed to hold sound events");
        }
        Pattern::Failure { error } => {
            panic!("Pattern parsing failed with error: {}", error);
        }
    }

    teardown(); // Decrement counter & possibly shut down RTS
}
