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

struct Foo(bool);//true if it should change exit code on drop
impl Drop for Foo {
    fn drop(&mut self) {
        //eprintln!("cleaning up stuff(u should see this even during panics)!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        if self.0 {
            std::rt::EXIT_CODE_ON_PANIC.store(TEST_CUSTOM_EXIT_CODE, std::sync::atomic::Ordering::Relaxed); // if rustc errors on this, it's because it's not patched with /patches/portage/dev-lang/rust.reused/2300_rust_exitcode_on_panic.patch
        }
    }
}


fn read_cmdline_args(pid: u32) -> Vec<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;
    let path = format!("/proc/{}/cmdline", pid);
    let mut file = File::open(&path).unwrap_or_else(|e| panic!("Couldn't open file '{}', error: '{}'", &path, e));

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|e| panic!("Couldn't read from opened file '{}', error: '{}'", &path, e));

    let buf_len:usize=buffer.len();
    if buf_len <= 0 { // < kept in case it changes type to eg. isize in the future!
        panic!("Read no chars from file '{}'", path);
    }
    let without_last_element:usize=buf_len-1;
    // Split the buffer by null characters into arguments
    // but split will add an extra empty element if the last char is \0 which is always is
    let args: Vec<Vec<u8>> = buffer[..without_last_element]
        .split(|&b| b == b'\0')
        //.filter(|slice| !slice.is_empty())
        .map(|slice| {
            let mut arg = Vec::with_capacity(slice.len() + 2); // +2 for single quotes

            // Append single quote
            arg.push(b'\'');
            // Append argument slice
            arg.extend(slice);
            // Append single quote
            arg.push(b'\'');

            arg
        }).collect();

    return args;
}

fn find_parent_pid(pid: u32) -> u32 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    // Construct the path to the stat file
    let path = format!("/proc/{}/stat", pid);
    let file = File::open(&path).unwrap_or_else(|e| panic!("Couldn't open file '{}', error: '{}'", &path, e));
    let reader = BufReader::new(file);

    // Read the first line (which should be the only line in stat)
    // Attempt to read the first line and handle errors
    let line = match reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => panic!("Error reading line from file '{}', error: '{}'", path, e),
        None => panic!("Empty file: '{}'", path),
    };

    // Split the line by spaces into fields
    let fields: Vec<&str> = line.split_whitespace().collect();

    // Parse the fourth field as u32 (which is the PPID)
    if let Some(ppid_str) = fields.get(3) {
        match ppid_str.parse::<u32>() {
            Ok(ppid) => return ppid,
            Err(e) => panic!("Couldn't parse the 4th whitespace-separated field as u32 in file '{}', error: '{}'", path, e),
        };
    } else {
        panic!("Couldn't read the 4th whitespace-separated field in file '{}'", path);
    }
}

fn get_callers_tree() -> Vec<u8> {
    const EXPECTED_INITIAL_DEPTH:usize=15;
    use std::collections::HashSet;
    fn recurser(pid:u32, processed_pids: &mut HashSet<u32>, indent: &mut String, output: &mut Vec<u8>) {
        /* $ sysctl kernel.pid_max
           kernel.pid_max = 4194304
        */
        if processed_pids.contains(&pid) {
            eprintln!("Avoided infinite loop for pid '{}' due to bad coding!(this msg is from rust binary, not the bash script)", pid);
            return;
        } else {
            processed_pids.insert(pid);
        }
        let args_of_pid=read_cmdline_args(pid);
        let how_many_args=args_of_pid.len();
        output.extend(format!("{}'{}'-'{}'-", indent, pid, how_many_args).bytes());
        let mut count=0;
        for each in args_of_pid {
            count+=1;
            output.extend(each);
            if count<how_many_args {
                output.extend(b" ");
            }
        }
        output.extend(b"\n");
        let parent_pid=find_parent_pid(pid);
        if parent_pid > 0 {
            indent.push(' ');
            recurser(parent_pid, processed_pids, indent, output);
        }//else well we were already at root pid, likely pid 1, so no other pid is its parent!
    }

    //let output:String = String::with_capacity(1024).push_str(" Our callers:\n");
    let mut output: Vec<u8> = Vec::with_capacity(1024);
    output.extend(b" Our callers:\n");
    //let processed_pids: Vec<u32>=Vec::with_capacity(EXPECTED_INITIAL_DEPTH);
    let mut processed_pids: HashSet<u32> = HashSet::with_capacity(EXPECTED_INITIAL_DEPTH);
    let mut indent:String=String::with_capacity(1+EXPECTED_INITIAL_DEPTH);
    indent.push(' ');//because 'Our callers' has an indent for 1 already.
    indent.push(' ');//start from indent 1
    recurser(std::process::id(), &mut processed_pids, &mut indent, &mut output);
    output
}

fn show_all_args<S>(exe_name:&str, the_args: &[S], save_to_file:bool)
    where S: AsRef<str> + std::fmt::Debug
{
    let text=format!("exe name:'{}', passed args({}):{:?}\n", exe_name, the_args.len(), the_args);
    eprint!("{}", text);
    if !save_to_file {
        return;
    }

    //doneTODO: maybe lock so multiple threads can't call this at same time?
    // Define a static mutex that is shared across all calls to this function
    use std::sync::Mutex;
    static FUNCTION_MUTEX: Mutex<()> = Mutex::new(());
    // Lock the mutex before executing the function
    let _guard = FUNCTION_MUTEX.lock().unwrap();

    use std::sync::atomic::{AtomicBool, Ordering};
    static ALREADY_SAVED:AtomicBool=AtomicBool::new(false);
    if ALREADY_SAVED.load(Ordering::Relaxed) {
        return;
    } else {
        ALREADY_SAVED.store(true, Ordering::Relaxed);
    }

    let log_file:&str=&format!("/var/log/{}.unhandled_args.log", exe_name);
    // Open a file in append mode
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file).unwrap_or_else(|e| panic!("Can't create or append to file '{}', error: '{}'", log_file, e));
    // Write to file
    std::io::Write::write_all(&mut file, text.as_bytes()).unwrap_or_else(|e|
        panic!("Can't write/append the first text to file '{}', error: '{}'", log_file, e)
    );
    std::io::Write::write_all(&mut file, &get_callers_tree()).unwrap_or_else(|e|
        panic!("Can't write/append the callers tree to file '{}', error: '{}'", log_file, e)
    );
    //let mut long_delim_line=String::with_capacity(80);//"=".repeat(79);
    let mut line = String::with_capacity(80);
    for _ in 0..79 {
        line.push('=');
    }
    line.push('\n');
    //long_delim_line.push('\n');
    std::io::Write::write_all(&mut file, line.as_bytes()).unwrap_or_else(|e|
        panic!("Can't write/append the last delimiter line text to file '{}', error: '{}'", log_file, e)
    );
    drop(file);
}

//fn insert_sorted(list: &mut Vec<usize>, value: usize) {
//    // Find the insertion point using binary search
//    let pos = list.binary_search(&value).unwrap_or_else(|e| e);
//    // Insert the value at the correct position
//    list.insert(pos, value);
//}

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
        //XXX: was cleanup executed tho?! NOT if I exit here! but anyway the cleanup func is https://github.com/rust-lang/rust/blob/59a4f02f836f74c4cf08f47d76c9f6069a2f8276/library/std/src/rt.rs#L105 and executed by line 146 below.
        //use std::io::Write;//else can't see: no method named `flush` found for struct `Stdout` in the current scope: method not found in `Stdout`
        //std::io::stdout().flush().unwrap();
        //std::io::stderr().flush().unwrap();
        //XXX: using std::process::exit() does call rt::cleanup which flushes stdout/stderr! however it won't run destructors like drop() for Foo, but if you use /patches/portage/dev-lang/rust.reused/2300_rust_exitcode_on_panic.patch then you can set the exit code that panic uses (was 101) by doing this:
        //std::process::exit(2);
        //XXX: letting this fall thru allows it to exit with the exit code we set in std::rt::EXIT_CODE_ON_PANIC
        let args: Vec<String> = env::args().collect();

        let exe_name_as_called= &args[0];
        let exe_name=Path::new(&exe_name_as_called).file_stem().and_then(|stem| stem.to_str()).expect("basename");
        //let _ = std::panic::catch_unwind(|| show_all_args(exe_name, &args[1..], true));//no effect, i'm already in a panic
        if ["diff", "patch"].contains(&exe_name) {
            show_all_args(exe_name, &args[1..], true);
        }
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
            //opts.opt("u", "unified", "output NUM (default 3) lines of unified contex", "", HasArg::No, Occur::Multi);
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
            let the_args=&args[1..];
            let matches = match opts.parse(the_args) {
                Ok(m) => { m }
                Err(f) => {
                    print_usage_diff(exe_name, opts);
                    show_all_args(exe_name, the_args, true);
                    panic!("{}", f.to_string());
                }
            };
            if matches.opt_present("h") {
                print_usage_diff(exe_name, opts);
                //assert_eq!(ExitCode::SUCCESS, 0);//binary operation `==` cannot be applied to type `ExitCode`
                //assert_eq!(ExitCode::SUCCESS, ExitCode::from(0));//same
                return ExitCode::SUCCESS;
            }
            //let args: Vec<String> = env::args().collect();
            let hm=matches.free.len();
            if hm != 2 {
                print_usage_diff(exe_name, opts);
                show_all_args(exe_name, the_args, true);
                let extra=if hm<2 {
                    "Missing the two files to compare, or maybe one of them was accidentally taken as an arg to some earlier option, if you forgot that arg.".to_string()
                } else {
                    format!("Too many files to compare, expecting exactly 2, got {}",hm)
                };
                panic!("{}", extra);
                //return ExitCode::from(2);
            }
            let file1_name=matches.free[0].clone();
            let file2_name=matches.free[1].clone();
            if matches.opt_count("label") > 2 {
                show_all_args(exe_name, the_args, true);
                panic!("too many file label options");
            }
            let labels=matches.opt_strs("label");
            let label1=if labels.len() >= 1 {
                &labels[0]
            } else { &file1_name };
            let label2=if labels.len() == 2 {
                &labels[1]
            } else { &file2_name };
            if matches.opt_present("label") {//FIXME: later, labels will be used actually!
                eprintln!("Ignoring labels '{}' '{}'", label1, label2);
            }
            if matches.opt_present("p") {
                eprintln!("Ignoring --show-c-function aka -p");
            }
            //an array of args that choose a type of output, but only one of which can be chosen, else they'd be conflicting!
            const DEE_SIZE:usize=6;
            let array_of_output_types:[&str; DEE_SIZE]=["u","c","normal","e","n","y"];
            //let array_of_pos_of_the_args=[-1; array_of_output_types.len()];
            //XXX: should fit isize because $ getconf ARG_MAX shows "2097152" aka 2MiB ...
            //let mut array_of_pos_of_the_args:[isize;DEE_SIZE]=[-1; DEE_SIZE];
            //let mut vec_of_last_poses_sorted:Vec<usize>=Vec::with_capacity(DEE_SIZE);
            //assert_eq!(array_of_pos_of_the_args.len(), array_of_output_types.len());
            assert_eq!(array_of_output_types.len(), DEE_SIZE);
            //FIXME: need a better way, HashSet? Vec?
            let mut highest_pos:isize=-1; // the highest position of one of the output_type args! is the overriding one!
            let mut overridden_output_type_is:&str="normal";//by default
            let mut count_of_found_types=0;
            for index in 0..DEE_SIZE {
                let current_output_type=array_of_output_types[index];
                if let Some(last_pos)=matches.opt_positions(current_output_type).last() {
                    //as isize;//.map_or(-1, |v| *v as isize);
                    //array_of_pos_of_the_args[index]= *last_pos as isize;
                    //vec_of_last_poses.push(*last_pos);
                    let deref:isize=*last_pos as isize;
                    //insert_sorted(&mut vec_of_last_poses_sorted, *last_pos);
                    assert_ne!(highest_pos, deref,"cannot be the same as next arg's pos which is at least higher by 1");
                    if highest_pos < deref {
                        highest_pos=deref;
                        overridden_output_type_is=current_output_type;
                    }
                    count_of_found_types+=1;
                //} else {
                }
            }
            //if vec_of_last_poses_sorted.len() > 1 {
            if count_of_found_types > 1 {
                show_all_args(exe_name, the_args, true);
                panic!("conflicting output style options");
            } else {
                if highest_pos >= 0 {
                    // XXX: position of the arg isn't the same as argv[position], for example: `./diff -a --has_arg 1 -b` has the `-b` at position 2, because `--has_arg 1` is considered one position, and position is 0-based, so it's seeing 3 args at positions 0,1, and 2.
                    //eprintln!("Arg at position '{}' overrides output type to '{}'", highest_pos, overridden_output_type_is);
                    eprintln!("Arg at position '{}' (which is 0-based and eg. `--foo 99` is 1 arg) overrides output type to '{}'", highest_pos, overridden_output_type_is);
                } else {
                    eprintln!("No output type overriding args, defaulting to '{}'", overridden_output_type_is);
                }
            }
            //let u_last:isize = matches.opt_positions("u").last().map_or(-1, |v| *v as isize);
            ////eprintln!("{}",u_last);//pos can be 0 because it's index
            //let c_last = matches.opt_positions("c").last().map_or(-1, |v| *v as isize);
            //let norm_last= matches.opt_positions("normal").last().map_or(-1, |v| *v as isize);
            //if [u_last, c_last, norm_last].iter().filter(|&x| *x > -1).count() > 1 {
            //let last_diff_type = if u_last > c_last {
            //let last_diff_type:&str=overridden_output_type_is;
//                "Unified diff"
//            } else if c_last > 0 {
//                "Context diff"
//            } else {
//                "No diff type specified, assuming --normal"
//            };
            //eprintln!("{}", last_diff_type);//obsoleteFIXME: make this an enum?
            //eprintln!("{}", overridden_output_type_is);
            eprintln!("{:?}", matches.opt_strs("unified"));
            const DEFAULT_CONTEXT_LENGTH_WHEN_UNSPECIFIED:i32=3;
            let context_length = match matches.opt_strs("unified").last() { //this catches the uppercase -U too! unclear why, maybe due to --unified being same? and it matches the --unified as well, for what's worth. so either "u" or "unified" here is same.
                Some(cl) => {
                    let res= cl.parse::<i32>();
                    match res {
                        Ok(lines) => {
                            lines
                        },
                        Err(e) => {
                            //FIXME: this is lame hack to allow -up because getopts can't allow it, it expects -uNUM
                            if !["p"].contains(&cl.as_str()) {
                                panic!("Context length '{}' isn't an i32 number, error: '{}'", cl, e);
                            }
                            DEFAULT_CONTEXT_LENGTH_WHEN_UNSPECIFIED
                        }
                    }//match
                },
                None => DEFAULT_CONTEXT_LENGTH_WHEN_UNSPECIFIED,
            };
            eprintln!("Context length: {}", context_length);
            if context_length < 0 {
                show_all_args(exe_name, the_args, true);
                panic!("negative context length given");
            }
            eprintln!("Free: {} {:?}",matches.free.len(), matches.free);
            if overridden_output_type_is != "u" {
                show_all_args(exe_name, the_args, true);
                panic!("Unsupported output type via rust, TODO: maybe delegate to real '{}' ?", exe_name);
            } else {
                //ok
                show_all_args(exe_name, the_args, false);
            }

            let file1 = fs::read(file1_name.clone()).unwrap_or_else(|e| panic!("Failed to read file1 '{}' (pwd='{}'), error: '{}'", &file1_name, std::env::current_dir().map_or("N/A".to_string(), |v| v.display().to_string()), e));
            let file2 = fs::read(file2_name.clone()).unwrap_or_else(|e| panic!("Failed to read file2 '{}' (pwd='{}'), error: '{}'", &file2_name, std::env::current_dir().map_or("N/A".to_string(), |v| v.display().to_string()), e));
            //let file2 = fs::read_to_string(file2_name.clone()).unwrap_or_else(|e| panic!("Failed to read file2 '{}', error: '{}'", &file2_name, e));

            //TODO: maybe just have diffy get us the correct context length for unambiguity and delegate the patch making to original gnu 'diff' command with that context length(aka lines of context)! But the problem is that's difficult to find out where to insert the new --unified=CONTEXTLENGTH_NUM in the original args due to possibly '--' or args coming after the 2 file names; or, just use getopts to understand all args and only pass the overrides to the original 'diff'; so `diff -u1 -u2 -u3 file1 file2 -u4`  will pass `diff -u4 file1 file2` only but this means all args must be understood via getopts crate here. Another thing is, that it might be better to use diffy due to rust safety. And then if using 'diffy' to make the patch, must allow for --label to work, and -p is currently not possible and for some reason gnu 'diff' does get it right, most of the time, for rust too.
            let patch = create_patch_bytes(&file1, &file2);
            let stdout = std::io::stdout(); // Get the handle to the standard output
            let mut handle = stdout.lock(); // Lock the handle for writing
            let handle_ref = &mut handle;
            //std::io::Write::write_all(&mut handle,
            std::io::Write::write_all(handle_ref,
            //use std::io::Write;
            //handle.write_all(
                patch.to_bytes().as_slice()).unwrap(); // Write the byte slice to the standard output
            //handle.flush().unwrap(); // Flush the output buffer to ensure all data is written
            //std::io::Write::flush(&mut handle).unwrap();
            std::io::Write::flush(handle_ref).unwrap();
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
