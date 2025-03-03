//! Manages Haskell RTS lifecycle (initialization and shutdown).

use libc::{c_char, c_int};
use once_cell::sync::OnceCell;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

#[link(name = "tidalparseffi", kind = "static")]
extern "C" {
    fn hs_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
    fn hs_exit();
}

/// Ensures Haskell RTS is only initialized **once** per application.
static RTS_COUNTER: AtomicUsize = AtomicUsize::new(0);
static INIT: OnceCell<()> = OnceCell::new();

/// Initializes Haskell RTS **only once**.
pub fn setup() {
    INIT.get_or_init(|| unsafe {
        hs_init(ptr::null_mut(), ptr::null_mut());
        println!("Haskell RTS initialized");
    });
    RTS_COUNTER.fetch_add(1, Ordering::SeqCst);
}

/// Shuts down Haskell RTS **only when the last reference is dropped**.
pub fn teardown() {
    if RTS_COUNTER.fetch_sub(1, Ordering::SeqCst) == 1 {
        unsafe {
            hs_exit();
        }
        println!("Haskell RTS shut down");
    }
}
