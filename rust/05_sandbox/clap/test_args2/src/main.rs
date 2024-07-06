// initially from tutorial: https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
use clap::Parser;
use std::path::{Path, PathBuf};

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

fn resolve_realpath<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Attempt to canonicalize the path to resolve symlinks and get absolute path
    match std::fs::canonicalize(path) {
        Ok(abs_path) => Some(abs_path),
        Err(_) => None,
    }
}

fn main() {
    let args = Args::parse();
    if args.debug > 0 {
        //use clap::crate_name;
        //let executable_name = crate_name!();
        //relative path here:
        let executable_name:String = resolve_realpath(std::env::args().next().unwrap_or_else(|| "unknown".to_string())).expect("can't realpath").into_os_string().into_string().expect("non utf8");
//        let executable_name:&str = match std::env::args().next() {
//            //.unwrap_or_else(|| "unknown".to_string());
//            Some(exe) => resolve_realpath(exe.clone()).expect(&format!("can realpath $0 aka '{}'",exe)).to_str().unwrap_or(&exe),
//            None => "unknown",
//        };
        //absolute path here:
        let exe_name_abs_path=std::env::current_exe().expect("why would this fail");
        assert_eq!(exe_name_abs_path.to_string_lossy(), executable_name, "discrepancy detected, likely the path or exe name aren't UTF-8 ! FIXME: handle this case");
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
