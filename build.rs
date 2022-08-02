extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.c");
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.c")
        .use_core()
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

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/wrapper.c")
        .warnings(false)
        .extra_warnings(false)
        .compile("lang-3bc");
}
