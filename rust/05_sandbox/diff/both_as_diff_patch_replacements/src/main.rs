use diffy::create_patch;
use diffy::PatchFormatter;
use diffy::{Patch, apply};
use std::str;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};



fn resolve_realpath<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Attempt to canonicalize the path to resolve symlinks and get absolute path
    match std::fs::canonicalize(path) {
        Ok(abs_path) => Some(abs_path),
        Err(_) => None,
    }
}

fn main() {
    let exe_name_as_called:String = std::env::args().next().unwrap_or_else(|| "unknown".to_string());
    let realpath_of_exe_name_as_called:String = resolve_realpath(std::env::args().next().unwrap_or_else(|| "unknown".to_string())).expect("can't realpath").into_os_string().into_string().expect("non utf8");
    let exe_name_abs_path=std::env::current_exe().expect("why would this fail");
    assert_eq!(exe_name_abs_path.to_string_lossy(), realpath_of_exe_name_as_called, "discrepancy detected, likely the path or exe name aren't UTF-8 ! FIXME: handle this case");
    let exe_name=Path::new(&exe_name_as_called).file_stem().and_then(|stem| stem.to_str()).expect("basename");
    eprintln!("Executable name: {}", exe_name);
    match exe_name {
        "diff" => {

            let args: Vec<String> = env::args().collect();
            if args.len() != 3 {
                eprintln!("Usage: diff <file1> <file2>");
                std::process::exit(1);
            }

            let file1 = fs::read_to_string(&args[1]).expect("Failed to read file1");
            let file2 = fs::read_to_string(&args[2]).expect("Failed to read file2");

            let patch = create_patch(&file1, &file2);
            let color: bool = false;
            if color {
                let f = PatchFormatter::new().with_color();
                print!("{}", f.fmt_patch(&patch));
            } else {
                print!("{}", patch);
            }
        }, //diff
        "patch" => {
            let args: Vec<String> = env::args().collect();
            if args.len() > 5 || args.len() < 4 {
                eprintln!("Usage: patch <original_file> <patch_file> <output_file> true/false");
                eprintln!("true/false is for unambiguous");
                std::process::exit(1);
            }
            #[allow(unused_variables)]
            let unambiguous:bool;
            #[allow(unused_assignments)]
            if args.len()==5 {
                unambiguous=args[4].parse().expect("Failed to parse arg number 5 into bool: true/false");
            } else {
                unambiguous=false;
            }

            let original_file = fs::read(&args[1]).expect("Failed to read original file");
            let patch_file = fs::read(&args[2]).expect("Failed to read patch file");

            let original_str = str::from_utf8(&original_file).expect("Failed to convert original file to string");
            let patch_str = str::from_utf8(&patch_file).expect("Failed to convert patch file to string");

            let patch = Patch::from_str(patch_str).expect("Failed to parse patch file");
            let patched_str = apply(original_str, &patch
                ,unambiguous
            ).expect("Failed to apply patch");

            fs::write(&args[3], patched_str.as_bytes()).expect("Failed to write output file");
        }, //patch
        anything_else => {
            panic!("unrecognized self name '{}'", anything_else);
        }
    }//match
}
