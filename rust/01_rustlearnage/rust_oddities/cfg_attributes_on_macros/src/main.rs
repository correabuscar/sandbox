//#![feature(plugin)]
//#![plugin(clippy)]

#![deny(clippy::print_stdout)]

fn main() {

    //XXX: but this creates a block!
    #[allow(clippy::print_stdout)] {
        println!("Hello, world!"); // all good now
    }

    {
        #![allow(clippy::print_stdout)]
        println!("Hello, world!"); // all good
    }

    #[allow(clippy::print_stdout)]  // - has no effect!
    println!("Hello, world!"); //error: use of `println!`
} // https://github.com/rust-lang/rust/issues/15701#issuecomment-368774192
