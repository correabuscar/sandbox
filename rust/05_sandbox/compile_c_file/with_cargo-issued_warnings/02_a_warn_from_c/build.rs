// build.rs

fn main() {
    println!("cargo:rerun-if-changed=src/c/warning.c");
    //println!("cargo:warning=Building C code..."); //this also works as a cargo warning hmm
    //cargo::warning is too new(end of 2023) https://github.com/rust-lang/cargo/commit/9ebe3b332a51cf413a2ee50d011339633bf2ed22

    //// Run the C compiler to compile the C source file
    //let status = std::process::Command::new("cc")
    //    .arg("-c")
    //    //.arg("src/c/warningo")
    //    .arg("src/c/a_warning.c")
    //    .status()
    //    .expect("Failed to compile C code");

    //if !status.success() {
    //    panic!("Failed to compile C code");
    //}
    println!("cargo:rerun-if-changed=src/lib.c");
    cc::Build::new()
        .file("src/c/a_warning.c")
        .flag("-Wdeprecated-declarations") // Enable warnings for deprecated declarations
        .flag("-Wall")
        .flag("-Wextra")
        //.flag("-Werror")
        .compile("warningo");
}

