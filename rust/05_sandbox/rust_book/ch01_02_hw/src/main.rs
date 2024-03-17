#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    warnings,
    future_incompatible,
    nonstandard_style,
    non_ascii_idents,
    clippy::restriction,
    rust_2018_compatibility,
    rust_2021_compatibility,
    unused
)]
#![allow(
    clippy::print_stdout,
    clippy::use_debug,
    clippy::missing_docs_in_private_items
)]
#![allow(clippy::blanket_clippy_restriction_lints)] //workaround clippy

// might want to deny later:
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::dbg_macro)]

fn main() {
    println!("Hello, world!");
}
