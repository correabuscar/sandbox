#![allow(unused_variables)]
#![allow(dead_code)]
//#![feature(no_std)]
#![no_implicit_prelude]

//#[feature(uniform_paths)]
//extern crate std;
//use std::string::String;

// "::" required, see issue: https://github.com/rust-lang/rust/issues/56390
use ::std::marker::Copy as OwnershipBypasser;//can't use it in #[derive()] tho
use ::std::string::String;
use ::std::ops::Drop;
use ::std::convert::From;
use ::std::string::ToString;
use ::std::println;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug, Clone)]
struct Sense(i32,f64, String);

#[derive(Debug, Clone)]
struct Sense2(i32,f64, i64);
impl Drop for Sense2 { //hahaaaaaa bye bye Copy :)) error[E0184]: the trait `Copy` may not be implemented for this type; the type has a destructor
    fn drop(&mut self) {
        self.0=0;
        self.1=0.0;
        self.2=0;
    }
}


//#[derive(Debug, Clone, OwnershipBypasser)] // can't use OwnershipBypasser here : error: cannot find derive macro `OwnershipBypasser` in this scope
#[derive(Debug, Clone, Copy)] //still works without any 'use' or stuff
struct EvilSense3(i32,f64, i64);

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,

    };

    user1.email = String::from("anotheremail@example.com");

    let a= Sense(1,2.0, "blah".to_string());
    println!("{:?}", a);
    takes_ownership(a);
    //println!("{:?}", a);//error[E0382]: borrow of moved value: `a`
    let b=Sense2(4,3.0,-5);
    takes_ownership2(b);
    //println!("{:?}", b);//error[E0382]: borrow of moved value: `b`

    let evil=EvilSense3(10,11.01,13);
    thinks_it_takes_ownership3(evil);
    println!("{:?}", evil);
}

fn takes_ownership(what: Sense) {

}

fn takes_ownership2(tobemoved: Sense2) {

}

fn thinks_it_takes_ownership3(mut tobemoved:  EvilSense3) {
  //but actually just copies, due to Copy trait of the EvilSense3 type
  tobemoved.0=34;
}

