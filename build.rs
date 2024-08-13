use std::{env::var, path::PathBuf};

use bindgen::{Builder, CargoCallbacks};

fn main() {
    let out_path =
        PathBuf::from(var("OUT_DIR").expect("failed to get OUT_DIR environment variable"));
    let bindings = Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .blocklist_file(".*stdio.h")
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
