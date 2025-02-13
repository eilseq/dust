# Event Handling

The `event_handling` crate defines the core event structures and parsing logic for processing TidalCycles events. It provides structured representations for both sound and note events.

## Features

- Defines structured event types (`SoundEvent`, `NoteEvent`, `Arc`).
- Parses and processes sound and note events from TidalCycles patterns.
- Supports serialization and deserialization via `serde`.

## Usage

1. **Create Events**: Instantiate `SoundEvent` or `NoteEvent` with structured data.
2. **Parse Note Values**: Convert raw TidalCycles note strings into `NoteValue` objects.
3. **Handle Time Spans**: Use `Arc` to represent event timing.

### Example

```rust
use event_handling::{SoundEvent, NoteValue};

let sound_event = SoundEvent::from_value("hh:3");
let note = NoteValue::from_str("2.0n (d5)").unwrap();
```

## Modules

- `arc`: Defines the `Arc` struct for time span representation.
- `note`: Handles parsing and structuring of musical notes.
- `pattern`: Converts raw pattern data into structured events.
- `sound`: Parses and represents sound events.

## Dependencies

- `regex`
- `serde`
- `serde_json`

## Testing

Unit tests ensure correct parsing and event handling. Run `cargo test` to verify functionality.
