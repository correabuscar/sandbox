#![feature(nll)]
//^ requires nightly, not beta, not stable!

fn main() {
    let mut y: &i32;
    let x = 5;

    y = &x;

    println!("{}", y);
    y = &3;
    println!("{}", y);
} // error[E0597]: `x` does not live long enough  (without the NLL thing!!)
