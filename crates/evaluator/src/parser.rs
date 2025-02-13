use crate::ffi::eval_pattern;
use event_handling::{Event, NoteEvent, SoundEvent};

/// Structure representing the event queue to be played.
pub struct EventQueue {
    pub events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

/// Processes and returns the parsed events from a pattern as an `EventQueue`.
pub fn parse_event_queue(pattern: &str, arc_length: &str) -> Result<EventQueue, String> {
    println!(
        "\nProcessing pattern: {:?} with arc length {:?}\n",
        pattern, arc_length
    );

    match eval_pattern(pattern, arc_length) {
        Ok(events) => {
            let mut event_queue = EventQueue::new();

            for event in events {
                match event {
                    Event::Sound(sound) => {
                        event_queue.add_event(Event::Sound(SoundEvent {
                            whole: sound.whole,
                            part: sound.part,
                            sound: sound.sound,
                            index: sound.index,
                        }));
                    }
                    Event::Note(note) => {
                        event_queue.add_event(Event::Note(NoteEvent {
                            whole: note.whole,
                            part: note.part,
                            note: note.note,
                        }));
                    }
                }
            }

            Ok(event_queue)
        }
        Err(e) => Err(format!("Parser error: {}", e)),
    }
}
