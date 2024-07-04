// initially from tutorial: https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(
        short = 'U',
        //short='u', //this overwrites the prev. short=
        long,
        default_value_t = 3,
        value_name = "NUM",
        help = "Output NUM lines of unified context (default 3)"
    )]
    unified: u64,

    #[arg(
        short = 'p',
        long,
        default_value_t = false,
        help = "Show which C function each change is in (ignored)"
    )]
    show_c_function: bool,
}

fn main() {
    let args = Args::parse();
    if args.debug > 0 {
        //use clap::crate_name;
        //let executable_name = crate_name!();
        let executable_name = std::env::args().next().unwrap_or_else(|| "unknown".to_string());
        println!("Executable name: {}", executable_name);
        eprintln!("{args:?}");
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match args.debug {
        0 => eprintln!("Debug mode is off"),
        1 => eprintln!("Debug mode is kind of on"),
        2 => eprintln!("Debug mode is on"),
        _ => eprintln!("Don't be crazy"),
    }
}
