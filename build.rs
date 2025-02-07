fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=tidalparse");
    println!("cargo:rustc-link-lib=static=ffi");

    // Explicitly link the C++ standard library
    println!("cargo:rustc-link-lib=c++"); // macOS uses "c++", Linux uses "stdc++"
}
