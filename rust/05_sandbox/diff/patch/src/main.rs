use diffy::{Patch, apply};
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: patch <original_file> <patch_file> <output_file>");
        std::process::exit(1);
    }

    let original_file = fs::read(&args[1]).expect("Failed to read original file");
    let patch_file = fs::read(&args[2]).expect("Failed to read patch file");

    let original_str = str::from_utf8(&original_file).expect("Failed to convert original file to string");
    let patch_str = str::from_utf8(&patch_file).expect("Failed to convert patch file to string");

    let patch = Patch::from_str(patch_str).expect("Failed to parse patch file");
    let patched_str = apply(original_str, &patch).expect("Failed to apply patch");

    fs::write(&args[3], patched_str.as_bytes()).expect("Failed to write output file");
}

