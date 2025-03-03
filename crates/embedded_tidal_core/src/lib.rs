//! Main module for evaluator functions.

mod ffi;
mod rts;

pub use ffi::eval_pattern;
pub use rts::{setup, teardown};
