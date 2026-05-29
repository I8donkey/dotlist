extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Generate C header (.h)
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("bindings/c/list_lang_ffi.h");

    // Generate C++ header (.hpp)
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::Cxx)
        .generate()
        .expect("Unable to generate C++ bindings")
        .write_to_file("bindings/cpp/list_lang_ffi.hpp");
}
