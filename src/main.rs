use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::ptr;

// Declare Haskell FFI functions
extern "C" {
    fn hs_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
    fn hs_exit();
    fn eval_pattern_c(input: *const c_char, arc: *const c_char) -> *mut c_char;
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

            print!("Enter cycle length (default 16): ");
            io::stdout().flush().unwrap();

            let mut arc_input = String::new();
            io::stdin()
                .read_line(&mut arc_input)
                .expect("Failed to read line");
            let arc_input = arc_input.trim();

            // Default to 16 if input is empty
            let arc_str = if arc_input.is_empty() {
                "16"
            } else {
                arc_input
            };

            let c_pattern = CString::new(input).expect("CString::new failed");
            let c_arc = CString::new(arc_str).expect("CString::new failed");

            println!(
                "Sending to Haskell: {:?} with arc length {:?}",
                input, arc_str
            );

            // Evaluate pattern using Haskell FFI
            let result_ptr = eval_pattern_c(c_pattern.as_ptr(), c_arc.as_ptr());
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
