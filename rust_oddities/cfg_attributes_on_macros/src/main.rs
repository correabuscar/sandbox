#![feature(plugin)]
#![plugin(clippy)]

#![deny(print_stdout)]

fn main() {

    #[allow(print_stdout)] {
        println!("Hello, world!"); // all good now
    }

    {
        #![allow(print_stdout)]
        println!("Hello, world!"); // all good
    }

    #[allow(print_stdout)]
    println!("Hello, world!"); //error: use of `println!`
} // https://github.com/rust-lang/rust/issues/15701#issuecomment-368774192
