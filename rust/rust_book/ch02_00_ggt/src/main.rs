#![no_implicit_prelude]
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

extern crate alloc; //or else: error[E0433]: failed to resolve: could not find `alloc` in the list of imported crates

// "::" required, see issue: https://github.com/rust-lang/rust/issues/56390
//use ::std::io;
//use ::std::io as io;
//use ::std::io::stdin;
//use ::std::println;
//use ::std::string::String;

#[derive(Debug)]
struct MyType(i32);

fn foo_rw(a: &mut MyType) {
    a.0 = 2;
}

const fn foo_r(_a: &MyType) {
    //a.0 = 3; // E0594: cannot assign to `a.0`, which is behind a `&` reference `a` is a `&`
    // reference, so the data it refers to cannot be written!
}

fn foob_r(mut a: ::alloc::boxed::Box<i32>) {
    use ::core::convert::AsMut;
    let b = a.as_mut();
    ::std::println!("b_before={b}");
    *b = 2;
    ::std::println!("b_after={b}");
    //yeah but 'a' is just a copy of the passed in param, I'm not actually modifying the passed in
    //param, but the local copy of it. hmm
}

//TODO: can you sneakily change the value if it's on the heap, for an immutable binding?

//impl ::std::marker::Copy for ::alloc::boxed::Box<i32> {} // `Copy` not allowed on types with destructors

fn main() {
    let mut a = MyType(1);
    let b = ::alloc::boxed::Box::new(10);

    ::std::println!("a_before= {a:?}");
    ::std::println!("b_before= {b}");
    foo_rw(&mut a);
    foo_r(&a);
    foob_r(b);
    ::std::println!("a_after = {a:?}");
    //::std::println!("b_after = {b}"); //can't , moved above!

    ::std::println!("Please input your guess.");

    let mut guess = ::alloc::string::String::new();

    #[allow(clippy::expect_used)]
    //stdin()
    //io::stdin()
    ::std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    ::std::println!("You guessed: '{guess}'");
}
