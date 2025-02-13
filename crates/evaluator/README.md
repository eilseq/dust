# Evaluator

The `evaluator` crate provides functionality for evaluating TidalCycles patterns using Haskell's runtime system (RTS) via FFI. It integrates with `event_handling` to process parsed events.

## Features

- Parses and evaluates TidalCycles patterns.
- Interfaces with Haskell via FFI.
- Manages the lifecycle of Haskell RTS.
- Provides structured event parsing for sound and note events.

## Usage

1. **Initialize RTS**: Before evaluation, the Haskell runtime system must be set up.
2. **Evaluate Patterns**: Pass TidalCycles patterns to be processed.
3. **Process Events**: Retrieve parsed events in a structured format.
4. **Shutdown RTS**: Properly clean up resources after usage.

### Example

```rust
use evaluator::{eval_pattern, setup, teardown};

setup();
let pattern = "s $ \"bd hh\"";
let arc_length = "1.0";
let result = eval_pattern(pattern, arc_length);
teardown();
```

## Modules

- `ffi`: Handles communication with Haskell's TidalCycles FFI.
- `parser`: Processes event queues and formats structured events.
- `rts`: Manages Haskell RTS initialization and shutdown.

## Dependencies

- `event_handling`
- `libc`
- `once_cell`

## Testing

Integration tests verify correct parsing and event handling for both sound and note events. Use `cargo test` to run them.
