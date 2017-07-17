extern crate backtrace;

use backtrace::Backtrace as b;
// thanks to https://github.com/retep998  for the idea to use this crate!

fn main() {
    println!("!!Hello, world! \n{:?}\n!!done", b::new());

    //the panic variant:
    std::env::set_var("RUST_BACKTRACE", "1");
    panic!();
}

