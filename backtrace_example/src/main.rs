extern crate backtrace;

use backtrace::Backtrace as b;
// thanks to https://github.com/retep998  for the idea to use this crate!

use std::panic;

fn main() {
    println!("!!Hello, world! \n{:?}\n!!done", b::new());

    //the panic variant:
    //first with custom hook and using the above backtrace
    panic::set_hook(Box::new(|_| {
        println!("Custom panic hook\n{:?}",b::new());
    }));
    std::env::set_var("RUST_BACKTRACE", "1");
    let arg1 = std::env::args().nth(1).unwrap();
    if arg1 == "2" {
        //remove the custom hook set above!
        println!("!! removing custom panic hook!");
        let _ = panic::take_hook();
    }
    panic!();
    //unreachable!();
}

