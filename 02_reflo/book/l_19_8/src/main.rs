#![deny(clippy::pedantic, clippy::all, clippy::correctness, clippy::nursery)]
#![deny(warnings)]

extern "C" {
    //fn abs(input: i32) -> i32; //XXX good
    fn abs(input: i64) -> i64; //this still works somehow ; XXX: BAD
                               //fn abs(input: i8) -> i8; //this too!
    fn floor(input: f64) -> f64; // unknown
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        let val = i64::min_value();
        println!("Absolute value of {} according to C: {}", val, abs(val)); // XXX bad !
        println!("Floor: {} {}", floor(-3.233_333_1), floor(3.233_333_1));
    }
}
