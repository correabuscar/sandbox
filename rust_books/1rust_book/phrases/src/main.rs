extern crate phrases;

use phrases::english::{greetings,farewells};
use phrases::japanese;//it's different than 'english' see its mod.rs

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese(日本語): {}", japanese::goodbye());
}
