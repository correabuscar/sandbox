extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    //break this ^ with 1: cargo run </dev/urandom
    //or 2: echo -e '\xc3\x28'| cargo run
    //src for 2: https://stackoverflow.com/questions/1301402/example-invalid-utf8-string/3886015#3886015

    println!("You guessed: {}", guess);
}
