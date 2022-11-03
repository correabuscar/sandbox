#![no_implicit_prelude]

// "::" required, see issue: https://github.com/rust-lang/rust/issues/56390
//use ::std::io;
//use ::std::io as io;
//use ::std::io::stdin;
//use ::std::println;
//use ::std::string::String;

fn main() {
    ::std::println!("Guess the number!");

    ::std::println!("Please input your guess.");

    let mut guess = ::std::string::String::new();

    //stdin()
    //io::stdin()
    ::std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    ::std::println!("You guessed: '{guess}'");
}
