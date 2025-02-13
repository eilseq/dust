//! Main module for evaluator functions.

mod ffi;
mod parser;
mod rts;

pub use ffi::eval_pattern;
pub use parser::parse_event_queue;
pub use rts::{setup, teardown};
