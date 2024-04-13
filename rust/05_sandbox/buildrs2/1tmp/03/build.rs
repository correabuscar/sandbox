// To test some of build.rs' code correctness run:
// cargo build --features=test_build_rs_of_ncurses_rs
// when doing that, the following cfg_attr ensures there are no warnings about unused stuff.
#![cfg_attr(
    all(
        feature = "test_build_rs_of_ncurses_rs",
        not(feature = "dummy_feature_to_detect_that_--all-features_arg_was_used")
    ),
    allow(dead_code)
)]
#![allow(clippy::uninlined_format_args)] // or is it more readable inlined?

extern crate cc;
extern crate pkg_config;

use pkg_config::Library;
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;

// Optional environment variables:

// The below doc comment doesn't apply for these 2 env.vars:
const ENV_VAR_NAME_FOR_LIB: &str = "NCURSES_RS_RUSTC_LINK_LIB";
const ENV_VAR_NAME_FOR_NCURSES_RS_RUSTC_FLAGS: &str = "NCURSES_RS_RUSTC_FLAGS";

/// Assuming we want env.var "NCURSES_RS_CFLAGS" here,
/// and target==host and is "x86_64-unknown-linux-gnu"
/// then calls to Build::try_flags_from_environment() below in code,
/// will try the following env.vars in this order:
/// 1. "NCURSES_RS_CFLAGS_x86_64-unknown-linux-gnu" (notice dashes)
/// 2. "NCURSES_RS_CFLAGS_x86_64_unknown_linux_gnu" (notice underscores)
/// 3. "HOST_NCURSES_RS_CFLAGS" or "TARGET_NCURSES_RS_CFLAGS" (if target!=host)
/// 4. "NCURSES_RS_CFLAGS" (our original wanted)
/// and the first one that exists is used instead.
/// see: https://docs.rs/cc/1.0.92/src/cc/lib.rs.html#3571-3580
const ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS: &str = "NCURSES_RS_CFLAGS";

const IS_WIDE: bool = cfg!(all(feature = "wide", not(target_os = "macos")));

// will search for these and if not found
// then the last one in list will be used as fallback
// and still try linking with it eg. -lncursesw
const NCURSES_LIB_NAMES: &[&str] = if IS_WIDE {
    &["ncursesw5", "ncursesw"]
} else {
    &["ncurses5", "ncurses"]
};

fn find_library(names: &[&str]) -> Option<Library> {
    for name in names {
        if let Ok(lib) = pkg_config::probe_library(name) {
            return Some(lib);
        }
    }
    None
}

// This is the normal build.rs main(),
// it's only disabled when you used: `cargo build --feature=test_build_rs_of_ncurses_rs`
#[cfg(any(
    not(feature = "test_build_rs_of_ncurses_rs"),
    feature = "dummy_feature_to_detect_that_--all-features_arg_was_used"
))]
fn main() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!(
        "cargo:rerun-if-env-changed={}",
        ENV_VAR_NAME_FOR_NCURSES_RS_RUSTC_FLAGS
    );
    println!("cargo:rerun-if-env-changed={}", ENV_VAR_NAME_FOR_LIB);

    let ncurses_lib = find_library(NCURSES_LIB_NAMES);

    if cfg!(feature = "menu") {
        if IS_WIDE {
            find_library(&["menuw5", "menuw"]);
        } else {
            find_library(&["menu5", "menu"]);
        }
    }

    if cfg!(feature = "panel") {
        if IS_WIDE {
            find_library(&["panelw5", "panelw"]);
        } else {
            find_library(&["panel5", "panel"]);
        }
    }

    // gets the name of ncurses lib found by pkg-config, if it found any!
    // else (warns and)returns the default one like 'ncurses' or 'ncursesw'
    let lib_name = get_ncurses_lib_name(&ncurses_lib);

    if let Ok(x) = std::env::var(ENV_VAR_NAME_FOR_NCURSES_RS_RUSTC_FLAGS) {
        println!("cargo:rustc-flags={}", x);
    }

    check_chtype_size(&ncurses_lib);

    gen_rs(
        "src/genconstants.c",
        "genconstants",
        "raw_constants.rs",
        &ncurses_lib,
        &lib_name,
    );

    gen_rs(
        "src/menu/genconstants.c",
        "genmenuconstants",
        "menu_constants.rs",
        &ncurses_lib,
        &lib_name,
    );

    build_wrap(&ncurses_lib);
}

fn build_wrap(ncurses_lib: &Option<Library>) {
    println!("cargo:rerun-if-changed=src/wrap.c");
    let mut build = cc::Build::new();
    if let Some(lib) = ncurses_lib {
        build.includes(&lib.include_paths);
        //for path in lib.include_paths.iter() {
        //    build.include(path);
        //}
    }
    // The following creates `libwrap.a` on linux
    build.file("src/wrap.c").compile("wrap");
}

/// Compiles a .c file then generates a .rs file from its output.
/// Uses ncurses include paths and links with ncurses lib(s)
fn gen_rs(
    source_c_file: &str,
    out_bin_file: &str,
    gen_rust_file: &str,
    ncurses_lib: &Option<Library>,
    lib_name: &str,
) {
    println!("cargo:rerun-if-changed={}", source_c_file);
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    #[cfg(unix)]
    let out_bin_file = format!("{}.exe", out_bin_file);
    let bin_full = Path::new(&out_dir).join(out_bin_file).display().to_string();

    //Note: env.var. "CC" can override the compiler used and will cause rebuild if changed.
    let mut build = cc::Build::new();
    let mut linker_searchdir_args: Vec<String> = Vec::new();
    if let Some(lib) = ncurses_lib {
        build.includes(&lib.include_paths);
        //for path in lib.include_paths.iter() {
        //    build.include(path);
        //}
        for link_path in &lib.link_paths {
            linker_searchdir_args.push("-L".to_string());
            linker_searchdir_args.push(link_path.display().to_string());
        }
    }

    println!(
        "cargo:rerun-if-env-changed={}",
        ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS
    );

    let _ = build.try_flags_from_environment(ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS);

    //'cc::Build' can do only lib outputs but we want a binary
    //so we get the command (and args) thus far set and add our own args.
    //Presumably all args will be kept, as per: https://docs.rs/cc/1.0.92/cc/struct.Build.html#method.get_compiler
    //(though at least the setting for build.file(source_c_file) won't be,
    // but we don't use that way and instead set it later as an arg to compiler)
    let compiler = build
        .try_get_compiler()
        .expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    //create a bin(not a lib) from a .c file
    //adding the relevant args for the libs that we depend upon such as ncurses
    command
        .arg("-o")
        .arg_checked(&bin_full)
        .arg_checked(source_c_file)
        .args_checked(["-l", lib_name])
        .args_checked(linker_searchdir_args);
    command.success_or_panic(); //runs compiler

    //execute the compiled binary
    let consts = Command::new(&bin_full)
        .output()
        .unwrap_or_else(|err| panic!("Executing '{}' failed, reason: '{}'", bin_full, err));

    //write the output from executing the binary into a new rust source file .rs
    //that .rs file is later used outside of this build.rs, in the normal build
    let gen_rust_file_full_path = Path::new(&out_dir)
        .join(gen_rust_file)
        .display()
        .to_string();
    let mut file = File::create(&gen_rust_file_full_path).unwrap_or_else(|err| {
        panic!(
            "Couldn't create rust file '{}', reason: '{}'",
            gen_rust_file_full_path, err
        )
    });

    file.write_all(&consts.stdout).unwrap_or_else(|err| {
        panic!(
            "Couldn't write to rust file '{}', reason: '{}'",
            gen_rust_file_full_path, err
        )
    });
}

fn check_chtype_size(ncurses_lib: &Option<Library>) {
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    let src = Path::new(&out_dir)
        .join("chtype_size.c")
        .display()
        .to_string();
    let bin_name = if cfg!(windows) {
        "chtype_size.exe"
    } else {
        "chtype_size"
    };
    let bin_full = Path::new(&out_dir).join(bin_name).display().to_string();

    let mut fp = File::create(&src)
        .unwrap_or_else(|err| panic!("cannot create '{}', reason: '{}'", src, err));
    fp.write_all(
        b"
#include <assert.h>
#include <limits.h>
#include <stdio.h>

#include <ncurses.h>

int main(void)
{
    if (sizeof(chtype)*CHAR_BIT == 64) {
        puts(\"cargo:rustc-cfg=feature=\\\"wide_chtype\\\"\");
    } else {
        /* We only support 32-bit and 64-bit chtype. */
        assert(sizeof(chtype)*CHAR_BIT == 32 && \"unsupported size for chtype\");
    }

#if defined(NCURSES_MOUSE_VERSION) && NCURSES_MOUSE_VERSION == 1
	puts(\"cargo:rustc-cfg=feature=\\\"mouse_v1\\\"\");
#endif
    return 0;
}
    ",
    )
    .unwrap_or_else(|err| panic!("cannot write into file '{}', reason: '{}'", src, err));

    let mut build = cc::Build::new();
    if let Some(lib) = ncurses_lib {
        build.includes(&lib.include_paths);
        //for path in lib.include_paths.iter() {
        //    build.include(path);
        //}
    }

    let _ = build.try_flags_from_environment(ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS);

    let compiler = build
        .try_get_compiler()
        .expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    command.arg("-o").arg_checked(&bin_full).arg_checked(&src);
    command.success_or_panic(); //runs compiler

    let features = Command::new(&bin_full)
        .output()
        .unwrap_or_else(|err| panic!("Executing '{}' failed, reason: '{}'", bin_full, err));
    print!("{}", String::from_utf8_lossy(&features.stdout));

    std::fs::remove_file(&src)
        .unwrap_or_else(|err| panic!("Cannot delete generated file '{}', reason: '{}'", src, err));
    std::fs::remove_file(&bin_full).unwrap_or_else(|err| {
        panic!(
            "cannot delete compiled file '{}', reason: '{}'",
            bin_full, err
        )
    });
}

//call this only once
fn get_ncurses_lib_name(ncurses_lib: &Option<Library>) -> String {
    let mut already_printed: bool = false;
    let lib_name: String = match std::env::var(ENV_VAR_NAME_FOR_LIB) {
        Ok(value) => value,
        Err(_) => {
            if let Some(ref lib) = ncurses_lib {
                // if here, `pkg-config`(shell command) via pkg_config crate,
                // has found the ncurses lib (eg. via the `ncurses.pc` file)
                // You can get something like this ["ncurses", "tinfo"] as the lib.libs vector
                // but we shouldn't assume "ncurses" is the first ie. lib.libs[0]
                // and the exact name of it can be ncurses,ncursesw,ncurses5,ncursesw5 ...
                // so find whichever it is and return that:
                let substring_to_find = "curses";
                if let Some(found) = lib.libs.iter().find(|&s| s.contains(substring_to_find)) {
                    //If we're here, the function calls to pkg_config::probe_library()
                    //from above ie. through find_library(), have already printed these:
                    //   cargo:rustc-link-lib=ncurses
                    //   cargo:rustc-link-lib=tinfo
                    //so there's no need to re-print the ncurses line as it would be the same.
                    already_printed = true;
                    found.clone()
                } else {
                    //if here, we should probably panic, but who knows it might still work even without pkg-config
                    //I've found cases where we were here and it still worked, so don't panic!

                    // Construct the repeated pkg-config command string
                    let repeated_pkg_config_command: String = NCURSES_LIB_NAMES
                        .iter()
                        .map(|ncurses_lib_name| format!("pkg-config --libs {}", ncurses_lib_name))
                        .collect::<Vec<_>>()
                        .join("` or `");

                    // Construct the warning message string with the repeated pkg-config commands
                    let warning_message = format!(
                    "pkg_config reported that it found the ncurses libs but the substring '{}' was not among them, ie. in the output of the shell command(s) eg. `{}`",
                    substring_to_find,
                    repeated_pkg_config_command
                    );

                    // Print the warning message, but use old style warning with one ":" not two "::",
                    // because old cargos(pre 23 Dec 2023) will simply ignore it and show no warning if it's "::"
                    println!("cargo:warning={}", warning_message);

                    //fallback lib name: 'ncurses' or 'ncursesw'
                    //if this fails later, there's the warning above to get an idea as to why.
                    (*NCURSES_LIB_NAMES.last().unwrap()).to_string()
                }
            } else {
                println!("cargo:warning=You may not have either pkg-config or pkgconf, or ncurses installed (it's 'ncurses-devel' on Fedora). Using fallback but if compilation fails below, that is why.");
                //pkg-config didn't find the lib, fallback to 'ncurses' or 'ncursesw'
                (*NCURSES_LIB_NAMES.last().unwrap()).to_string()
            }
        }
    };
    if !already_printed {
        println!("cargo:rustc-link-lib={}", lib_name);
    }
    lib_name
}

// Define an extension trait for Command
trait MyCompilerCommand {
    fn success_or_panic(&mut self) -> ExitStatus;
    //fn success_or_else<F: FnOnce(ExitStatus) -> ExitStatus>(&mut self, op: F) -> ExitStatus;
    fn status_or_panic(&mut self) -> ExitStatus;
    fn show_what_will_run(&mut self) -> &mut Self;
    fn get_what_will_run(&self) -> (String, usize, String);
    fn assert_no_nul_in_args(&mut self) -> &mut Self;
    /// Panics if arg has \0 in it.
    fn args_checked<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
    /// Panics if arg has \0 aka NUL in it,
    /// otherwise the original Command::arg would've set it to "<string-with-nul>"
    /// Doesn't do any other checks, passes it to Command::arg()
    fn arg_checked<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command;
}

fn has_null_byte<S: AsRef<OsStr>>(arg: S) -> bool {
    let os_str = arg.as_ref();
    for &byte in os_str.as_bytes() {
        if byte == 0 {
            return true;
        }
    }
    false
}

/// args with \0 in them, passed to std::process::Command::arg() or ::args()
/// get replaced entirely with this: "<string-with-nul>"
const REPLACEMENT_FOR_ARG_THAT_HAS_NUL: &str = "<string-with-nul>";
// Implement the extension trait for Command
impl MyCompilerCommand for Command {
    //TODO: test that these functions work as expected!
    /// you can't use an arg value "<string-with-nul>", or this will panic.
    fn success_or_panic(&mut self) -> ExitStatus {
        let exit_status: ExitStatus = self
            .show_what_will_run()
            .assert_no_nul_in_args()
            .status_or_panic();
        if exit_status.success() {
            exit_status
        } else {
            let how: String;
            if let Some(code) = exit_status.code() {
                how = format!(" with exit code {}", code);
            } else {
                how = ", was terminated by a signal".to_string();
            }
            panic!(
                "Compiler failed{}. Is ncurses installed? \
        pkg-config or pkgconf too? it's ncurses-devel on Fedora. \
        Or maybe it failed for different reasons which are seen in the output above.",
                how
            )
        }
    }
    //note: can't override arg/args because they're not part of a Trait in Command
    //so would've to wrap Command in my own struct for that. This would've ensured
    //that any added args were auto-checked.
    fn args_checked<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        for arg in args {
            self.arg_checked(arg.as_ref());
        }
        self
    }
    fn arg_checked<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        if has_null_byte(&arg) {
            //If the arg has NUL ie. \0  in it then arg got replaced already
            //with "<string-with-nul>", internally, by std::process::Command::arg() .
            //The found arg here will be shown with \0 in this Debug way.
            panic!(
                "Found arg '{:?}' that has at least one \\0 aka nul in it! \
                   This would've been replaced with '{}'.",
                arg.as_ref(),
                REPLACEMENT_FOR_ARG_THAT_HAS_NUL
            );
        }
        self.arg(arg)
    }
    /// Beware if user set the arg on purpose to the value of REPLACEMENT_FOR_ARG_THAT_HAS_NUL
    /// which is "<string-with-nul>" then this will panic, it's a false positive.
    fn assert_no_nul_in_args(&mut self) -> &mut Self {
        let args = self.get_args();
        for (count, arg) in args.enumerate() {
            if let Some(fully_utf8_arg) = arg.to_str() {
                //If the arg had NUL ie. \0  in it then arg got replaced already
                //with "<string-with-nul>", internally, by std::process::Command::arg() .
                if fully_utf8_arg == REPLACEMENT_FOR_ARG_THAT_HAS_NUL {
                    panic!(
                        "Found arg number '{}' that has \\0 aka NUL in it! \
                           It got replaced with '{}'.",
                        count, REPLACEMENT_FOR_ARG_THAT_HAS_NUL
                    );
                }
            }
        }
        self
    }
    fn get_what_will_run(&self) -> (String, usize, String) {
        let program = self.get_program();
        let p_prog = program
            .to_str()
            .unwrap_or_else(|| panic!("Compiler executable {:?} isn't valid rust string", program));
        let args = self.get_args();
        let how_many_args: usize = args.len();
        let formatted_args: String = args
            .map(|arg| {
                //If the arg had NUL ie. \0  in it then arg got replaced already
                //with "<string-with-nul>", internally, by std::process::Command::arg()
                //if it was added via Command::arg() or Command::args().
                //To prevent that use Command::arg_checked() and ::args_checked()
                if let Some(fully_utf8_arg) = arg.to_str() {
                    fully_utf8_arg.to_string()
                } else {
                    //None aka not fully utf8 arg
                    //then we show it as ascii + hex
                    let mut broken_arg = String::new();
                    use std::fmt::Write; // can't globally import this ^, conflicts with std::io::Write
                    for byte in arg.as_bytes() {
                        match std::char::from_u32(*byte as u32) {
                            Some(c) if c.is_ascii() => broken_arg.push(c),
                            _ => {
                                write!(&mut broken_arg, "\\x{:02X}", byte).expect("Failed to write")
                            }
                        }
                    }
                    broken_arg
                }
            })
            .collect::<Vec<String>>()
            .join("\" \"");
        //TODO: maybe a better way to get the args as a Vec<String> and impl Display ? but not
        //for the generic Vec<String> i think. Then, we won't have to return how_many_args!

        //return this tuple
        (
            p_prog.to_string(),
            how_many_args,
            format!("\"{}\"", formatted_args),
        )
    }
    /// just like Command::status() but panics if it can't execute it,
    /// ie. if status() would've returned an Err
    /// returns ExitStatus whether it be 0 or !=0
    fn status_or_panic(&mut self) -> ExitStatus {
        // Call the original status() method and handle the potential error
        self.status().unwrap_or_else(|err| {
            let (p_prog, how_many_args, formatted_args) = self.get_what_will_run();
            panic!(
                "Failed to run compilation command '{}' with '{}' args: '{}', reason: '{}'",
                p_prog, how_many_args, formatted_args, err
            )
        })
    }
    fn show_what_will_run(&mut self) -> &mut Self {
        let (exe_name, how_many_args, formatted_args) = self.get_what_will_run();
        eprintln!(
            "Next, attempting to run compilation command '{}' with '{}' args: '{}'",
            exe_name, how_many_args, formatted_args
        );
        self
    }
}

/// This is used to test build.rs, run with: cargo build --features=test_build_rs_of_ncurses_rs
/// This won't happen if you use --all-features
#[cfg(all(
    feature = "test_build_rs_of_ncurses_rs",
    not(feature = "dummy_feature_to_detect_that_--all-features_arg_was_used")
))]
fn main() {
    test_assert_works();
    test_invalid_utf8_in_program();
    test_nul_in_arg_unchecked();
    test_nul_in_arg();
    test_no_panic_in_command();
    test_panic_for_not_found_command();
    test_panic_for_command_non_zero_exit();
    test_get_what_will_run();

    eprintln!("\n-------------------------------------
              \n!!! All build.rs tests have passed successfully! Ignore the above seemingly erroneous output, it was part of the successful testing !!!\nYou're seeing this because you tried to build with --features=test_build_rs_of_ncurses_rs");

    // This stops the build from continuing which will fail in other places due to build.rs not
    // doing its job, since we've only just tested build.rs not used it to generate stuff.
    std::process::exit(5);
}
//The test functions are left outside of 'test_build_rs_of_ncurses_rs'
//so they're tested to still compile ok.

#[allow(dead_code)]
fn test_assert_works() {
    let result = std::panic::catch_unwind(|| {
        #[allow(clippy::assertions_on_constants)]
        {
            assert!(false, "!! just tested if asserts are enabled !!");
        }
    });
    #[allow(clippy::manual_assert)]
    if result.is_ok() {
        panic!("Assertions are disabled in build.rs, should not happen!");
    }
}

#[allow(dead_code)]
fn test_no_panic_in_command() {
    let cmd = if cfg!(windows) { "dir" } else { "ls" };
    let arg = if cfg!(windows) { "/?" } else { "--help" };
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new(cmd);
        command.arg(arg);
        command.status_or_panic();
    });
    let fail_msg = format!(
        "!!! This should not have panicked! Unless you don't have '{}' command, in PATH={:?} !!!",
        cmd,
        std::env::var("PATH")
    );
    assert!(result.is_ok(), "{}", fail_msg);

    // executed bin exits with exit code 0, or it would panic ie. fail the test
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new(cmd);
        command.arg(arg);
        command.success_or_panic();
    });
    assert!(result.is_ok(), "{}", fail_msg);

    // executed bin exits with specific exit code 2
    let result = std::panic::catch_unwind(|| {
        //TODO: windows variant here? (or does 'ls' exist there too? and exits with code 2?)
        let mut command = Command::new(cmd);
        let arg = "hopefully non exitent dir here";
        command.arg(arg);
        let exit_status = command.status_or_panic();
        let expected_ec = 2;
        assert_eq!(
            exit_status.code().expect("was command killed by a signal?"),
            expected_ec,
            "Command should've exited with exit code '{}'.",
            expected_ec
        );
    });
    assert!(result.is_ok(), "{}", fail_msg);
}

#[allow(dead_code)]
fn test_panic_for_not_found_command() {
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("some non-exitent command");
        command.args([OsString::from("ar♥g1")]);
        command.status_or_panic();
    });
    let expected_panic_msg=
     "Failed to run compilation command 'some non-exitent command' with '1' args: '\"ar♥g1\"', reason: 'No such file or directory (os error 2)'";
    expect_panic(result, expected_panic_msg);

    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("some non-exitent command");
        command.args([OsString::from("ar♥g1")]);
        command.success_or_panic();
    });
    expect_panic(result, expected_panic_msg);
}

#[allow(dead_code)]
fn test_panic_for_command_non_zero_exit() {
    let cmd = if cfg!(windows) { "dir" } else { "ls" };
    let arg = "hopefully non exitent dir here";
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new(cmd);
        command.arg(arg);
        command.success_or_panic();
    });
    let expected_panic_msg = "Compiler failed with exit code 2. Is ncurses installed? pkg-config or pkgconf too? it's 'ncurses-devel' on Fedora; run `nix-shell` first, on NixOS. Or maybe it failed for different reasons which are seen in the errored output above.";
    expect_panic(result, expected_panic_msg);
}

#[allow(dead_code)]
fn test_invalid_utf8_in_program() {
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new(OsString::from_vec(
            b"test_invalid_utf8_\xFFin_program".to_vec(),
        ));
        command.args([
            OsString::from("ar♥g1"),
            OsString::from_vec(b"my\xffarg3".to_vec()),
        ]);
        command.status_or_panic();
    });
    expect_panic(
        result,
        "Compiler executable \"test_invalid_utf8_\\xFFin_program\" isn't valid rust string",
    );
}

fn expect_panic(result: Result<(), Box<dyn std::any::Any + Send>>, expected_panic_message: &str) {
    if result.is_err() {
        if let Some(err) = result.unwrap_err().downcast_ref::<String>() {
            // Uncomment this to can copy/paste it for asserts:
            //println!("!!!!!!!!!! Panic message: {:?}", err);
            assert_eq!(
                err, expected_panic_message,
                "!!! Got different panic message than expected !!!"
            );
        }
    } else {
        panic!(
            "No panic was thrown! But was expecting this panic: '{}'",
            expected_panic_message
        );
    };
}

#[allow(dead_code)]
fn test_nul_in_arg_unchecked() {
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("test_nul_in_arg_unchecked.exe");
        command.args([
            OsString::from("ar♥g1"),
            OsString::from("a\0rg2"),
            OsString::from_vec(b"my\xffarg3".to_vec()),
        ]);
        command.status_or_panic();
    });
    expect_panic(result,
         "Failed to run compilation command 'test_nul_in_arg_unchecked.exe' with '3' args: '\"ar♥g1\" \"<string-with-nul>\" \"my\\xFFarg3\"', reason: 'nul byte found in provided data'"
        );
}

#[allow(dead_code)]
fn test_nul_in_arg() {
    //via .arg()
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("test_nul_in_arg.exe");
        command.arg_checked(OsString::from("ar♥g1"));
        command.arg_checked(
            // would panic here
            OsString::from("a\0rg2"),
        );
        command.arg_checked(OsString::from_vec(b"my\xffarg3".to_vec()));
        command.status_or_panic();
    });
    let expected_panic_msg=
         "Found arg '\"a\\0rg2\"' that has at least one \\0 aka nul in it! This would've been replaced with '<string-with-nul>'.";
    expect_panic(result, expected_panic_msg);
    //via .args()
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("test_nul_in_args.exe");
        command.args_checked([
            // would panic here
            OsString::from("ar♥g1"),
            OsString::from("a\0rg2"),
            OsString::from_vec(b"my\xffarg3".to_vec()),
        ]);
        command.status_or_panic();
    });
    expect_panic(result, expected_panic_msg);
}

#[allow(dead_code)]
fn test_get_what_will_run() {
    let expected_prog = "test_get_what_will_run.exe";
    let mut command = Command::new(expected_prog);
    command.arg_checked(OsString::from("ar♥g1"));
    command.args_checked([
        // would panic here
        OsString::from_vec(b"my\xffarg3".to_vec()),
        OsString::from("arg4"),
    ]);
    command.arg_checked(OsString::from_vec(b"my\xffarg3".to_vec()));
    let (prog, how_many_args, formatted_args) = command.get_what_will_run();
    let expected_hma = 4;
    let expected_fa = "\"ar♥g1\" \"my\\xFFarg3\" \"arg4\" \"my\\xFFarg3\"";
    //r###"ar♥g1" "my\xFFarg3" "arg4" "my\xFFarg3"###;
    assert_eq!(prog, expected_prog);
    assert_eq!(how_many_args, expected_hma);
    assert_eq!(formatted_args, expected_fa);
}
