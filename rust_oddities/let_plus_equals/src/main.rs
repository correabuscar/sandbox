#![allow(unused_variables)]
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s1 = s1 + &s2; // this works
    //let s1 += &s2; // this doesn't work, doesn't even recognize +=, which kinda makes sense!
    //however, it was meant to emulate the previous 'let'

    let mut s1=s1; 
    s1 += &s2; //this works but needs 's1' to be 'mut'(done on prev. line) - thanks sarnold on #rust-beginners irc

    let mut _i = 10;
    _i += 1; //this works
}

