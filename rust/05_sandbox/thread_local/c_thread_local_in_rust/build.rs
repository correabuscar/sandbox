fn main() {
    // Compile the C code into a shared library using cc crate
    cc::Build::new()
        .file("tl.c")
        .shared_flag(true)
        .compile("tl");
}
