//fn main() {
//    println!("cargo:rustc-link-lib=dylib=abort_hook");
//}

use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();//eg. ./target/debug/build/abort_hook-6e9231fa4d206e9d/out/ but absolute (probably)
    let lib_name = "libcustom_abort.so";

    // Compile the Rust code into a shared library
    let status = Command::new("rustc")
        .args(&["src/lib.rs", "--crate-type=cdylib", "-o"])
        .arg(&format!("{}/{}", out_dir, lib_name))
        // Link with libc
//        .arg("-l")
//        .arg("c") // aka libc for libc::puts
        .status()
        .expect("failed to compile custom abort library");

    if !status.success() {
        panic!("Failed to compile custom abort library");
    }

    // Output the path to the shared library
    println!("cargo:rustc-link-search=native={}", out_dir); // needed
    //println!("cargo:rustc-link-lib=dylib=custom_abort");
    //println!("cargo:rustc-link-lib=custom_abort");

    // Link with libc
    //println!("cargo:rerun-if-changed=build.rs");
    //println!("cargo:rustc-link-lib=c");
}

