#![deny(large_assignments)]
#![feature(large_assignments)]
#![move_size_limit = "1"] // 0 here disables it.

//see: https://github.com/rust-lang/rust/pull/83519/files#diff-a83e0054300d480e1f72a82b5b49c0f01c5f4e49475b0ed2427074d14c67f82cR152

fn a_copy(x: u32) {
    println!("func:{x}");
}

fn main() {
    let x=1000;
    //let y=x.clone();
    let y=x;
    //a_copy(x);
    println!("Hello, world!{y}");
}
