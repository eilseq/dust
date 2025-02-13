fn main() {
    // Ensure Cargo searches the correct directory for libraries
    println!("cargo:rustc-link-search=native=ext");

    // Link to the static library libtidalparseffi.a
    println!("cargo:rustc-link-lib=static=tidalparseffi");
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup"); // Ensures missing symbols are resolved dynamically
    println!("cargo:rustc-link-lib=dylib=ffi"); // Ensure libffi is linked for tests

    // Link the C++ standard library
    println!("cargo:rustc-link-lib=c++");
}
