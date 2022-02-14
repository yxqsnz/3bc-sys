extern crate bindgen;
use std::env;
use std::path::PathBuf;
fn main() {
    println!("cargo:rerun-if-changed=wrapper.c");
    let bindings = bindgen::Builder::default()
        .header("wrapper.c")
        .impl_debug(true)
        .trust_clang_mangling(true)
        .generate_block(true)
        .blocklist_type("FP_.*")
        .blocklist_function("FP_.*")
        .blocklist_item("FP_.*")
        .allowlist_type("driver_.*")
        .allowlist_function("driver_.*")
        .allowlist_type("app_.*")
        .allowlist_function("app_.*")
        .allowlist_function("interpreter_.*")
        .allowlist_type("driver_.*")
        .allowlist_function("ds_.*")
        .allowlist_type("ds_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("wrapper.c")
        .compile("lang-3bc");
}
