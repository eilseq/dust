use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::ptr;

// Declare Haskell FFI functions
extern "C" {
    fn hs_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
    fn hs_exit();
    fn eval_pattern_c(input: *const c_char) -> *mut c_char;
}

fn main() {
    unsafe {
        // Initialize Haskell RTS
        hs_init(ptr::null_mut(), ptr::null_mut());

        loop {
            print!("Enter pattern (or :quit to exit): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();

            if input == ":quit" {
                break;
            }

            let c_pattern = CString::new(input).expect("CString::new failed");
            println!("Sending to Haskell: {:?}", input);

            // Evaluate pattern using Haskell FFI
            let result_ptr = eval_pattern_c(c_pattern.as_ptr());
            if result_ptr.is_null() {
                eprintln!("Failed to evaluate pattern");
            } else {
                let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
                println!("Evaluation Result: {}", result);
            }
        }

        // Clean up Haskell RTS
        hs_exit();
    }
}
