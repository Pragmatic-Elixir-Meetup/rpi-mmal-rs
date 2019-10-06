extern crate bindgen;

use std::env::var;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .constified_enum_module("MMAL_STATUS_T")
        .whitelist_type(r"MMAL_.*")
        .whitelist_function(r"(?:bcm_|mmal_|vcos_).*")
        .whitelist_var(r"MMAL_.*")
        .derive_debug(true)
        .impl_debug(true)
        .clang_arg("-I/opt/vc/include")
        .clang_arg("-I/usr/arm-linux-gnueabihf/include/")
        .clang_arg("-I/usr/lib/gcc/arm-linux-gnueabihf/4.6/include-fixed/")
        .clang_arg("-I/usr/lib/gcc/arm-linux-gnueabihf/4.6/include/")
        .header("vendor/include/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=bcm_host");
    println!("cargo:rustc-link-lib=mmal_core");
    println!("cargo:rustc-link-lib=mmal_util");
    println!("cargo:rustc-link-lib=mmal_vc_client");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-search=native=/opt/vc/lib");
}
