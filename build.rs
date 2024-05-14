fn main() {
    println!("cargo:rustc-env=LD_LIBRARY_PATH=./gosrc/");
    println!("cargo:rustc-link-lib=ser");

    let bindings = bindgen::Builder::default()
        .header("gosrc/libser.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
