#![no_implicit_prelude]

// "::" required, see issue: https://github.com/rust-lang/rust/issues/56390
use ::std::io;
//use std::io as io;
use ::std::println;
use ::std::string::String;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: '{guess}'");
}
