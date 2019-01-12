//extern crate rand; //hmm not needed
use rand::prelude::*;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &mut v {
        println!("{:?}", i);
        *i = SpreadsheetCell::Text(String::from("grass"));
        println!("{:?}", i);
    }
    for i in &v {
        println!("{:?}", i);
    }

    println!("next:");
    for i in &mut v {
        //println!("{:?}", i);
        change(i);
        println!("{:?}", i);
    }
    println!("next2:");
    for _ in 1..=100 - v.len() {
        // XXX: wicked!!!
        let   a:  SpreadsheetCell;//getting bad error message with this; so, how do I create a new 'a' here on every loop? //see https://github.com/rust-lang/rust/issues/57553
        //let   a=SpreadsheetCell::Int(1); //getting better error message with this!(says to make it 'mut')
        //let    mut a:  SpreadsheetCell; //getting no errors with this! because made it 'mut'!
        change(&mut a);
        v.push(a);
    }
    println!("next3:");
    for i in &v {
        println!("{:?}", i);
    }
}

fn change(i: &mut SpreadsheetCell) {
    let mut rng = rand::thread_rng(); //TODO: how can I init this only once? but not before this function runs
    let y: i64 = rng.gen();
    let r#mod = y % 10;
    match r#mod {
        2 => *i = SpreadsheetCell::Text(String::from("two")),
        3 => *i = SpreadsheetCell::Int(rng.gen::<i32>()),
        5 => *i = SpreadsheetCell::Float(rng.gen::<f64>()),
        _ => *i = SpreadsheetCell::Text(String::from(r#mod.to_string())),
    }
}

