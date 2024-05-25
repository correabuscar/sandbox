fn main() {
    //#[allow(non_snake_case)]
    // bug: this ^ below is ignored by: $ rust-analyzer diagnostics .
    // "non_snake_case: Variable `FOO` should have snake_case name, e.g. `foo`"
    // unless any editing has happened
    // XXX: ^ that's explained by rust-analyzer running `cargo check` for on-the-fly code due to:
    //  "on-the-fly diagnostics are mostly unimplemented (`cargo check` diagnostics will be shown when saving a file): #3107" https://github.com/rust-lang/rust-analyzer/issues/3107
    #[allow(non_snake_case)]
    let SWOO:i32 = 14;
    println!("{}", SWOO);
}
