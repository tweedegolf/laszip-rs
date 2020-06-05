use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=laszip/src/laszip_dll.cpp");
    println!("cargo:rerun-if-changed=laszip/dll/laszip_api.c");
    println!("cargo:rerun-if-changed=laszip/include/laszip/laszip_api.h");

    // build laszip library
    let dst = cmake::Config::new("laszip")
        .define("LASZIP_BUILD_STATIC", "off")
        .build();

    // tell cargo to tell rustc where to find the compiled laszip
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Tell cargo to tell rustc to link to laszip statically
    println!("cargo:rustc-link-lib=laszip_api");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .header(format!(
            "{}/include/laszip/laszip_api_version.h",
            dst.display()
        ))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
