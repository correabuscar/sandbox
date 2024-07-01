use diffy::create_patch;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: diff <file1> <file2>");
        std::process::exit(1);
    }

    let file1 = fs::read_to_string(&args[1]).expect("Failed to read file1");
    let file2 = fs::read_to_string(&args[2]).expect("Failed to read file2");

    let patch = create_patch(&file1, &file2);
    println!("{}", patch);
}

