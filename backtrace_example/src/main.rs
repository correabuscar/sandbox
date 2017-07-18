extern crate backtrace;
extern crate colored;

use backtrace::Backtrace as b;
// thanks to https://github.com/retep998  for the idea to use this crate!

use std::panic;

use colored::*;

static STACKTRACE_COLOR: &'static str = "darkgray";
static NORMALTEXT_COLOR: &'static str = "yellow";

fn main() {
    //println!("!!Hello, world! \n{}\n!!done", format!("{:?}",b::new()).red());

    println!("{}",format!("!!Hello, world! \n{}\n!!done", format!("{:?}",b::new()).color(STACKTRACE_COLOR)).color(NORMALTEXT_COLOR));//works!
    //println!("{}{}{}","!!Hello, world! \n".yellow(), format!("{:?}",b::new()).red(),"\n!!done".yellow());//works too but repeating yellow twice!

    //the panic variant:
    //first with custom hook and using the above backtrace
    panic::set_hook(Box::new(|_| {
        println!("{}", format!("!! Custom panic hook\n{}",format!("{:?}",b::new()).color(STACKTRACE_COLOR)).color(NORMALTEXT_COLOR));
    }));
    std::env::set_var("RUST_BACKTRACE", "1");
    let arg1 = std::env::args().nth(1).unwrap();
    if arg1 == "2" {
        //remove the custom hook set above!
        println!("{}","!! removing custom panic hook!".color(NORMALTEXT_COLOR));
        let _ = panic::take_hook();
    }
    panic!();
    //unreachable!();
}

