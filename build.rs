fn main() {
    #[cfg(feature = "bindgen")]
    bindgen();
}

#[cfg(feature = "bindgen")]
fn bindgen() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_file(".*stdio.h")
        .generate()
        .expect("Failed to generate bindings");

    let pwd = std::env::current_dir().unwrap();
    bindings
        .write_to_file(pwd.join("src").join("bindings.rs"))
        .expect("Failed to write bindings");
}
