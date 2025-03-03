use embedded_tidal_core::{eval_pattern, setup, teardown};

#[test]
fn test_embedded_tidal_core_evaluation() {
    setup(); // Initialize Haskell RTS

    let pattern = "stack [s \"bd bd bd\" # gain 0.8 # cutoff 4000 # pan 0.2, n \"c4 e4 g4\" # gain 0.7 # cutoff 2000 # pan 0.5]";
    let arc_length = "1.0";

    let result = eval_pattern(pattern, arc_length);
    assert!(result.is_ok(), "Pattern evaluation failed: {:?}", result);

    let json_output = result.unwrap();
    println!("Raw JSON Output: {}", json_output); // Debug output

    assert!(
        json_output.contains("bd") || json_output.contains("c4"),
        "Expected sound or note events in JSON output"
    );

    teardown(); // Clean up RTS
}
