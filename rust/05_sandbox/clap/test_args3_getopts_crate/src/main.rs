//initial example from: https://docs.rs/getopts/0.2.21/getopts/index.html#example
//extern crate getopts;
use getopts::{Options, Occur, HasArg};
use std::env;

//fn do_work(inp: &str, out: Option<String>) {
//    println!("{}", inp);
//    match out {
//        Some(x) => println!("{}", x),
//        None => println!("No Output"),
//    }
//}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    /* Single-character options are expected to appear on the command line with a single preceding dash; multiple-character options are expected to be proceeded by two dashes. Options that expect an argument accept their argument following either a space or an equals sign. Single-character options don't require the space. */
    //FIXME: well that's not true because --unified 34 is taken as --unified and the free arg 34, and not like --unified=34 !
    opts.opt("u", "unified", "output NUM (default 3) lines of unified contex", "NUM", HasArg::Maybe, Occur::Multi);
    opts.opt("U", "unified", "output NUM (default 3) lines of unified contex", "NUM", HasArg::Maybe, Occur::Multi);
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let context_length = match matches.opt_str("u") { //this catches the uppercase -U too! unclear why, maybe due to --unified being same?
        Some(cl) => cl.parse::<i32>().expect(&format!("Context length '{}' isn't an i32 number.", cl)),
        None => 3,
    };
    eprintln!("Context length: {}", context_length);
    if context_length < 0 {
        panic!("negative context length given");
    }
    eprintln!("Free: {} {:?}",matches.free.len(), matches.free);
    assert_eq!(matches.free.len(), 0, "Expected no free args!");
//    let input = if !matches.free.is_empty() {
//        matches.free[0].clone()
//    } else {
//        print_usage(&program, opts);
//        return;
//    };
    //do_work(&input, output);
}
