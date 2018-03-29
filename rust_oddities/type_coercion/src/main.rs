#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];

let initial_scores: Vec<i32> = vec![10, 50]; //if this (i32),
let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect(); //then this &i32 must be! or sure you can use '_'
let scores: HashMap<&String, _> = teams.iter().zip(initial_scores.iter()).collect();

let initial_scores = vec![10, 50];//if unspecified integer,
let scores: HashMap<&String, &_> = teams.iter().zip(initial_scores.iter()).collect(); //this keeps the &{integer} type
let scores: HashMap<&String, _> = teams.iter().zip(initial_scores.iter()).collect(); //this keeps the &{integer} type
let scores: HashMap<&String, &u64> = teams.iter().zip(initial_scores.iter()).collect();//this coerces the type of initial_scores to Vec<u64> ! so then &u64, &i32, &i8, &u8, &u32 etc. can be used here, but must be integer, so can't do &f64
//let _:() =initial_scores[0];//error: expected (), found u64; note: This line is just to show the type, at compile time!
//let scores: HashMap<&String, _> = teams.iter().zip(initial_scores.iter()).collect();//using '_' means {integer} is used by default (not i32 heh); IF you comment out the above ^u64 line, obviously!
//let _:() =initial_scores[0];//error: expected (), found {integer}; note: This line is just to show the type, at compile time!

    #[deny(overflowing_literals)]
    let _v = vec![-4000000000, 10, 50, -4000000000]; //this is still i32, unless coerced by hashmap type below with &i64
    #[deny(overflowing_literals)]
    let _v2:Vec<i64> = vec![-4000000000, 10, 50, -4000000000]; // manually set type - works!
    //_v[0] = -4000000000;//no effect even when mut _v, to coerce the type into Vec<i64>

    let names=vec!["a","b"];
    let hm: HashMap<_, _> = names.iter().zip(_v.iter()).collect();//still i32!
    let hm: HashMap<_, &i64> = names.iter().zip(_v.iter()).collect(); //needed to set the right type for _v aka Vec<i64>
    //let _: () = _v[0]; //just showing the type of _v[0] aka type of the elements of vector _v
}
