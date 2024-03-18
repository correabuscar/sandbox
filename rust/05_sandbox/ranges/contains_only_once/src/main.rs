#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    //clippy::cargo,
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
    clippy::separated_literal_suffix,
    clippy::blanket_clippy_restriction_lints,
    clippy::min_ident_chars,
    clippy::missing_assert_message,
    clippy::missing_docs_in_private_items,
    clippy::bool_comparison
    )]
fn main() {
    #[allow(unused_mut)]
    let mut r=1_u8..=20_u8;
    assert!(r.contains(&10_u8)); //sure, but consumes the iterator
    assert!(r.contains(&10_u8)); //still contains it
    assert!(r.contains(&1_u8)); //still contains it
    assert!(r.contains(&20_u8)); //still contains it
    assert!(r.contains(&20_u8)); //still contains it
    for _ in r.by_ref() {} //ok, this finishes iteration!
    //r.start();
    //r.end();
    println!("{} {} {}", r.contains(&20_u8), r.start(), r.end()); //why this doesn't panic! i dno.
    assert!(false == r.contains(&20)); // so it's false because iteration finished TODO: patch rust
                                       // to panic when start/end/contains are used and iteration
                                       // was finished.
    //assert!(false);
    println!("Hello, world!");
}
