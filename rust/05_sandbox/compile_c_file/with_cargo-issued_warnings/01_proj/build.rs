fn main() {
    println!("cargo:rerun-if-changed=src/lib.c");
    cc::Build::new()
        .file("src/lib.c")
        .flag("-Wdeprecated-declarations") // Enable warnings for deprecated declarations
        .flag("-Wall")
        .flag("-Wextra")
        //.flag("-Werror")
        .compile("libproj");
}

