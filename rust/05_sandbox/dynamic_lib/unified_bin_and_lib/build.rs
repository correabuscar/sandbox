//use cargo_toml::{Manifest, CargoToml};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    eprintln!("!!!!!!!!! outdir={}",out_dir);//not seen, maybe -v is needed? nope, something else?!
    //println!("cargo:warning=outdir={}",out_dir);
    //XXX: kind odd that only one ":" is needed, as per:
    //https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472
    //vs 2 "::" as per: https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning

//    // Parse the Cargo.toml file
//    let cargo_toml = CargoToml::from_path("Cargo.toml").unwrap();
//    let manifest: Manifest = cargo_toml.parse().unwrap();
//
//    // Get the package name from the Cargo.toml file
//    let package_name = manifest.package.unwrap().name;
    //let package_name="unified_bin_and_lib";

    // Check the compilation context
    //let is_library = std::env::var("CARGO_CFG_TARGET_KIND").unwrap() == "cdylib";
//    if is_library {
        // Build the dynamic library
//    } else {
        // Set linker flags for the binary
        // Link against the library specified by the package name
//        println!("cargo:rustc-link-lib=dylib={}", package_name);
        //no effect:
//        println!("cargo:rustc-link-search=native={}", out_dir);
        //testing:
        // needed, because Cargo.toml's rpath=true isn't enough. FIXME: didn't work!
//        println!("cargo:rustc-link-arg=-Wl,--no-as-needed,-rpath={}",out_dir); //no effect
//    }
}

