#![feature(panic_update_hook)]
#![feature(rt)] // for std::rt::EXIT_CODE_ON_PANIC, needs: /patches/portage/dev-lang/rust.reused/2300_rust_exitcode_on_panic.patch

use diffy::create_patch_bytes;
//use diffy::PatchFormatter;
use diffy::{Patch, apply};
use std::str;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use getopts::{Options, Occur, HasArg};
use std::process::ExitCode;





/// resolving symlinks, can't be turned off!
fn resolve_realpath<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Attempt to canonicalize the path to resolve symlinks and get absolute path
    match std::fs::canonicalize(path) {
        Ok(abs_path) => Some(abs_path),
        Err(_) => None,
    }
}

const HARDCODED_DIFF_EXE:&str ="/usr/bin/diff";
const HARDCODED_PATCH_EXE:&str ="/usr/bin/patch";

fn exec_diff<I,S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr> + std::fmt::Debug,
{
    exec(HARDCODED_DIFF_EXE, args);
}
fn exec_patch<I,S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr> + std::fmt::Debug,
{
    exec(HARDCODED_PATCH_EXE, args);
}
/// returns exit code, or panics if exe got killed by signal!
fn exec<I,S>(exe:&str, args: I) -> i32
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr> + std::fmt::Debug,
{
     // Convert the iterator into a vector to count the number of arguments
    let args: Vec<S> = args.into_iter().collect();
    let num_args = args.len();
    let cmd_res=std::process::Command::new(exe).args(&args).output();
    let out=cmd_res.expect(&format!("Failed to start executable '{}' with '{}' args '{:?}'", exe, num_args, args));
    let stdout=String::from_utf8_lossy(&out.stdout);
    let stderr=String::from_utf8_lossy(&out.stderr);
    print!("{}", stdout);
    eprint!("{}", stderr);
    let exit_status=out.status;
    exit_status.code().expect(&format!("The execution of the following failed due to being killed by a signal(can't determine which tho), exe '{}' with '{}' args '{:?}'", exe, num_args, args))
}

fn print_usage_diff(exe: &str, opts: Options) {
    let brief=format!("Usage: {} <file1> <file2>", exe);
    print!("{}", opts.usage(&brief));
    exec_diff(["--help"]);
}

const TEST_CUSTOM_EXIT_CODE:i32=82;

struct Foo(bool);
impl Drop for Foo {
    fn drop(&mut self) {
        //eprintln!("cleaning up stuff(u should see this even during panics)!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        if self.0 {
            std::rt::EXIT_CODE_ON_PANIC.store(TEST_CUSTOM_EXIT_CODE, std::sync::atomic::Ordering::Relaxed); // if rustc errors on this, it's because it's not patched with /patches/portage/dev-lang/rust.reused/2300_rust_exitcode_on_panic.patch
        }
    }
}

fn main() -> ExitCode {
    let _f = Foo(false);// to see if dropped on panic!
    // Set the RUST_BACKTRACE environment variable to enable backtrace
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    // Set a custom panic hook
//    let default_hook = std::panic::take_hook();
//    std::panic::set_hook(Box::new(|panic_info| {
//        // Print the panic info
//        eprintln!("Panic occurred: {:?}", panic_info);
//
//        // Get the default panic hook and invoke it
//        default_hook(panic_info);
//
//        // FIXME: how about cleanup and exit with exit code 2 instead of 101?
//        // Exit with a specific exit code (2) during panic
//        std::process::exit(2); //2 means trouble in diff/patch cmdlines
//    }));
    std::rt::EXIT_CODE_ON_PANIC.store(2, std::sync::atomic::Ordering::Relaxed); // if rustc errors on this, it's because it's not patched with /patches/portage/dev-lang/rust.reused/2300_rust_exitcode_on_panic.patch
    //exit code 2 means trouble in diff/patch cmdlines
    std::panic::update_hook(move |prev, info| { // E0658: use of unstable library feature 'panic_update_hook'
        //println!("Print custom message and execute panic handler as usual");
        prev(info);
        //println!("fooooooooooooo");//yes this is reached
        //FIXME: was cleanup executed tho?! not if I exit here! but anyway the cleanup func is https://github.com/rust-lang/rust/blob/59a4f02f836f74c4cf08f47d76c9f6069a2f8276/library/std/src/rt.rs#L105 and executed by line 146 below.
        //use std::io::Write;//else can't see: no method named `flush` found for struct `Stdout` in the current scope: method not found in `Stdout`
        //std::io::stdout().flush().unwrap();
        //std::io::stderr().flush().unwrap();
        //XXX: using std::process::exit() does call rt::cleanup which flushes stdout/stderr! however it won't run destructors like drop() for Foo, but if you use /patches/portage/dev-lang/rust.reused/2300_rust_exitcode_on_panic.patch then you can set the exit code that panic uses (was 101) by doing this:
        //std::process::exit(2);
        //XXX: letting this fall thru allows it to exit with the exit code we set in std::rt::EXIT_CODE_ON_PANIC
    });
    let args: Vec<String> = env::args().collect();

    let exe_name_as_called:String = std::env::args().next().unwrap_or_else(|| "unknown".to_string());
    let realpath_of_exe_name_as_called:String = resolve_realpath(&exe_name_as_called).expect("can't realpath").into_os_string().into_string().expect("non utf8");
    let exe_name_abs_path=std::env::current_exe().expect("why would this fail");
    assert_eq!(exe_name_abs_path.to_string_lossy(), realpath_of_exe_name_as_called, "discrepancy detected, likely the path or exe name aren't UTF-8 ! FIXME: handle this case");
    let exe_name=Path::new(&exe_name_as_called).file_stem().and_then(|stem| stem.to_str()).expect("basename");
    eprintln!("Executable name: {}", exe_name);

    let mut opts = Options::new();
    match exe_name {
        "diff" => {
            opts.opt("u", "unified", "output NUM (default 3) lines of unified contex", "NUM", HasArg::Maybe, Occur::Multi);
            opts.opt("U", "unified", "output NUM (default 3) lines of unified contex", "NUM", HasArg::Maybe, Occur::Multi);
            opts.opt("c", "context", "output NUM (default 3) lines of copied contex", "NUM", HasArg::Maybe, Occur::Multi);
            opts.opt("C", "context", "output NUM (default 3) lines of copied contex", "NUM", HasArg::Maybe, Occur::Multi);
            opts.opt("p", "show-c-function", "show which C function each change is in", "", HasArg::No, Occur::Multi);
            opts.opt("e", "ed", "output an ed script", "", HasArg::No, Occur::Multi);
            opts.opt("n", "rcs", "output an RCS format script", "", HasArg::No, Occur::Multi);
            opts.opt("", "normal", "output a normal diff (the default)", "", HasArg::No, Occur::Multi);
            opts.opt("", "label", "use LABEL instead of file name and timestamp (can be repeated)", "LABEL", HasArg::Yes, Occur::Multi);
            opts.opt("y", "side-by-side", "output in two columns", "", HasArg::No, Occur::Multi);
            opts.opt("q", "brief", "report only when files differ (doesn't output a patch thus doesn't try to gen.unambiguous hunks)", "", HasArg::No, Occur::Multi);
            opts.opt("W", "width", "output at most NUM (default 130) print columns", "NUM", HasArg::Yes, Occur::Optional);//real 'diff' won't allow two -W unless they've same NUM, but we simplify by not allowing two -W
            opts.opt("s", "report-identical-files", "report only when two files are the same (doesn't output a patch thus doesn't try to gen.unambiguous hunks)", "", HasArg::No, Occur::Multi);
            opts.optflag("h", "help", "print this help text");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => { m }
                Err(f) => {
                    print_usage_diff(exe_name, opts);
                    panic!("{}", f.to_string())
                }
            };
            if matches.opt_present("h") {
                print_usage_diff(exe_name, opts);
                //assert_eq!(ExitCode::SUCCESS, 0);//binary operation `==` cannot be applied to type `ExitCode`
                //assert_eq!(ExitCode::SUCCESS, ExitCode::from(0));//same
                return ExitCode::SUCCESS;
            }
            //let args: Vec<String> = env::args().collect();
            if matches.free.len() != 2 {
                print_usage_diff(exe_name, opts);
                panic!("Missing the two files to compare, or maybe one of them was accidentally taken as an arg to some earlier option, if you forgot that arg.");
                //return ExitCode::from(2);
            }
            let file1_name=matches.free[0].clone();
            let file2_name=matches.free[1].clone();
            if matches.opt_count("label") > 2 {
                panic!("too many file label options");
            }
            let labels=matches.opt_strs("label");
            let label1=if labels.len() >= 1 {
                &labels[0]
            } else { &file1_name };
            let label2=if labels.len() == 2 {
                &labels[1]
            } else { &file2_name };
            eprintln!("Ignoring labels '{}' '{}'", label1, label2);
            //XXX: should fit isize because $ getconf ARG_MAX shows "2097152" aka 2MiB ...
            let u_last:isize = matches.opt_positions("u").last().map_or(-1, |v| *v as isize);
            //eprintln!("{}",u_last);//pos can be 0 because it's index
            let c_last = matches.opt_positions("c").last().map_or(-1, |v| *v as isize);
            let norm_last= matches.opt_positions("normal").last().map_or(-1, |v| *v as isize);
            if [u_last, c_last, norm_last].iter().filter(|&x| *x > -1).count() > 1 {
                panic!("conflicting output style options");
            }
            let last_diff_type = if u_last > c_last {
                "Unified diff"
            } else if c_last > 0 {
                "Context diff"
            } else {
                "No diff type specified, assuming --normal"
            };
            eprintln!("{}", last_diff_type);//FIXME: make this an enum?
            let context_length = match matches.opt_strs("unified").last() { //this catches the uppercase -U too! unclear why, maybe due to --unified being same? and it matches the --unified as well, for what's worth. so either "u" or "unified" here is same.
                Some(cl) => cl.parse::<i32>().expect(&format!("Context length '{}' isn't an i32 number.", cl)),
                None => 3,
            };
            eprintln!("Context length: {}", context_length);
            if context_length < 0 {
                panic!("negative context length given");
            }
            eprintln!("Free: {} {:?}",matches.free.len(), matches.free);

            let file1 = fs::read(file1_name.clone()).unwrap_or_else(|e| panic!("Failed to read file1 '{}' (pwd='{}'), error: '{}'", &file1_name, std::env::current_dir().map_or("N/A".to_string(), |v| v.display().to_string()), e));
            let file2 = fs::read(file2_name.clone()).unwrap_or_else(|e| panic!("Failed to read file2 '{}' (pwd='{}'), error: '{}'", &file2_name, std::env::current_dir().map_or("N/A".to_string(), |v| v.display().to_string()), e));
            //let file2 = fs::read_to_string(file2_name.clone()).unwrap_or_else(|e| panic!("Failed to read file2 '{}', error: '{}'", &file2_name, e));

            let patch = create_patch_bytes(&file1, &file2);
            let stdout = std::io::stdout(); // Get the handle to the standard output
            let mut handle = stdout.lock(); // Lock the handle for writing

            use std::io::Write;
            handle.write_all(patch.to_bytes().as_slice()).unwrap(); // Write the byte slice to the standard output
            handle.flush().unwrap(); // Flush the output buffer to ensure all data is written
            drop(handle);
            //let color: bool = false;
            //if color {
            //    let f = PatchFormatter::new().with_color();
            //    print!("{}", f.fmt_patch(&patch));
            //} else {
            //    print!("{}", patch);
            //}
            return ExitCode::SUCCESS;
        }, //diff
        "patch" => {
            let args: Vec<String> = env::args().collect();
            if args.len() > 5 || args.len() < 4 {
                eprintln!("Usage: patch <original_file> <patch_file> <output_file> true/false");
                eprintln!("true/false is for unambiguous");
                exec_patch(["--help"]);
                return ExitCode::from(2);
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
            let patch_file = fs::read(&args[2]).expect(&format!("Failed to read patch file: '{}'", &args[2]));

            let original_str = str::from_utf8(&original_file).expect("Failed to convert original file to string");
            let patch_str = str::from_utf8(&patch_file).expect("Failed to convert patch file to string");

            let patch = Patch::from_str(patch_str).expect("Failed to parse patch file");
            let patched_str = apply(original_str, &patch
                ,unambiguous
            ).unwrap_or_else(|e| {
                std::rt::EXIT_CODE_ON_PANIC.store(1, std::sync::atomic::Ordering::Relaxed);
                panic!("Failed to apply patch, '{}'",e);
            });

            fs::write(&args[3], patched_str.as_bytes()).expect("Failed to write output file");
            return ExitCode::SUCCESS;
        }, //patch
        "test_drops_and_exit_code_on_panic" => {
            let _change_it=Foo(true);//will change exit code on drop()
            panic!("induced to to see if drop() destructors get executed on panic and thus custom exit code '{}' is set on panic exit; reminder, without patching rustc you can't have custom exit code upon panic", TEST_CUSTOM_EXIT_CODE);
        },
        anything_else => {
            panic!("unrecognized self name '{}'", anything_else);
        }
    }//match
}
