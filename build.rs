extern crate bindgen;

use std::path::PathBuf;

fn main() {
    dotenv::dotenv().ok();
    // Load env path
    let erlang_path = std::env::var("ERLANG_PATH").expect("Missing erlang path variable");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}/usr/lib", erlang_path);

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ei");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rustc-link-search={}/usr/lib", erlang_path);
    println!("cargo:rerun-if-changed=build.rs");

    // THE bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/usr/include/ei.h", erlang_path))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_item("IPPORT_.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src/");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
