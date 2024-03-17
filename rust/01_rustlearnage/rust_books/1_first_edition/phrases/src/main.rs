extern crate phrases;
extern crate phrases as sayings;

use phrases::english::{greetings,farewells};
use phrases::japanese;//it's different than 'english' see its mod.rs
//use sayings::japanese::{self};//equivalent with the above ^
use sayings::japanese::greetings::hello as hi2;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hi());
    println!("Hello in Japanese: {}", hi2());
    println!("Goodbye in Japanese(日本語): {}", japanese::goodbye());
}
