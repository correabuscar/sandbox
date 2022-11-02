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
#![allow(clippy::default_numeric_fallback)] // might want to deny later!
#![allow(clippy::dbg_macro)]

//src: https://users.rust-lang.org/t/rust-book-suggestion-add-a-section-regarding-copy-vs-move/1549/2
fn foo(mut x: [i32; 4]) {
    println!("x(before) = {:?}", x);
    x = [1, 2, 3, 4];
    println!("x(after) = {:?}", x);
}

//src: https://stackoverflow.com/a/58119924/19999437
fn print_type_of<T>(_: &T) {
    //println!("{}", std::any::type_name::<T>());
    println!("{}", core::any::type_name::<T>());
}

fn main() {
    let a = [0; 4];
    //a.something();//method not found in `[{integer}; 4]`
    //a=1;//so this is an array
    //dbg!(a);
    println!("{:#?}", print_type_of(&a)); // i32
    foo(a); //sneakily copied!
    println!("a = {:?}", a); //unchanged, doh! but since it was just copied above, can use it here
                             //without errors!
}
