// To test if build.rs works correctly run: cargo build --features=test_build_rs_of_ncurses_rs
// when doing that, the following cfg_attr ensures there are no warnings about unused stuff.
#![cfg_attr(
    all(
        feature = "test_build_rs_of_ncurses_rs",
        not(feature = "dummy_feature_to_detect_that_--all-features_arg_was_used")
    ),
    allow(dead_code)
)]

extern crate cc;
extern crate pkg_config;

use pkg_config::Library;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS: &str = "NCURSES_RS_CFLAGS";
const ENV_VAR_NAME_FOR_LIB: &str = "NCURSES_RS_RUSTC_LINK_LIB";
const ENV_VAR_NAME_FOR_NCURSES_RS_RUSTC_FLAGS: &str = "NCURSES_RS_RUSTC_FLAGS";

const IS_WIDE: bool = cfg!(all(feature = "wide", not(target_os = "macos")));

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
        ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS
    );
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

    let mut already_printed: bool = false;
    let lib_name: String = match std::env::var(ENV_VAR_NAME_FOR_LIB) {
        Ok(value) => value,
        _ => {
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
                    NCURSES_LIB_NAMES.last().unwrap().to_string()
                }
            } else {
                println!("cargo:warning=You may not have either pkg-config or pkgconf, or ncurses installed (it's 'ncurses-devel' on Fedora). Using fallback but if compilation fails below, that is why.");
                //pkg-config didn't find the lib, fallback to 'ncurses' or 'ncursesw'
                NCURSES_LIB_NAMES.last().unwrap().to_string()
            }
        }
    };
    if !already_printed {
        println!("cargo:rustc-link-lib={}", lib_name);
    }

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
        for path in lib.include_paths.iter() {
            build.include(path);
        }
    }
    // The following creates `libwrap.a` on linux
    build.file("src/wrap.c").compile("wrap");
}

fn gen_rs(
    source_c_file: &str,
    out_bin_file: &str,
    gen_rust_file: &str,
    ncurses_lib: &Option<Library>,
    lib_name: &str,
) {
    println!("cargo:rerun-if-changed={}", source_c_file);
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    let bin = Path::new(&out_dir)
        .join(format!(
            "{}{}",
            out_bin_file,
            if cfg!(windows) { ".exe" } else { "" }
        ))
        .display()
        .to_string();
    //Note: env.var. "CC=mycompiler" can override the compiler used and will rebuild if changed.
    let mut build = cc::Build::new();
    let mut linker_searchdir_args = Vec::new();
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

    if let Ok(value) = std::env::var(ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS) {
        // assuming we want env.var "NCURSES_RS_CFLAGS",
        // Build::try_flags_from_environment() below,
        // will try the following env.vars in this order:
        // 1. "NCURSES_RS_CFLAGS_x86_64-unknown-linux-gnu" (notice dashes)
        // 2. "NCURSES_RS_CFLAGS_x86_64_unknown_linux_gnu" (notice underscores)
        // 3. "HOST_NCURSES_RS_CFLAGS"
        // 4. "NCURSES_RS_CFLAGS" (our original wanted)
        // and the first one that exists is used instead.
        // see: https://docs.rs/cc/1.0.92/src/cc/lib.rs.html#3571-3580
        build
            .try_flags_from_environment(ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to add CFLAGS from env.var '{}'='{}', reason: {}",
                    ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS, value, err
                )
            });
    };

    //'cc::Build' can do only lib outputs but we want a binary
    //so we get the command (and args) thus far and add our own args
    //Presumably all args will be kept, as per: https://docs.rs/cc/1.0.92/cc/struct.Build.html#method.get_compiler
    //but at least the setting for build.file(source_c_file) won't be,
    //so we don't use that way and instead set it later as an arg to compiler.
    let compiler = build
        .try_get_compiler()
        .expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    //create a bin(not a lib) from a .c file
    //adding the relevant args for the libs that we depend upon such as ncurses
    command
        .arg("-o")
        .arg(&bin)
        .arg(source_c_file)
        .args(["-l", lib_name])
        .args(linker_searchdir_args);
    command.success_or_panic(); //runs compiler

    //execute the compiled binary
    let consts = Command::new(&bin)
        .output()
        .unwrap_or_else(|err| panic!("Executing '{}' failed, reason: '{}'", bin, err));

    //write the output from executing the binary into a new rust source file .rs
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
    if let Ok(value) = std::env::var(ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS) {
        //TODO: find out which env.vars are tried first and make a note! use dbg!() in a modded
        //local cc crate from the repo
        build
            .try_flags_from_environment(ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to add CFLAGS from env.var '{}'='{}', reason: {}",
                    ENV_VAR_NAME_FOR_NCURSES_RS_CFLAGS, value, err
                )
            });
    };

    let compiler = build
        .try_get_compiler()
        .expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    command.arg("-o").arg(&bin_full).arg(&src);
    command.success_or_panic();

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

use std::process::ExitStatus;
// Define an extension trait for Command
trait MyCompilerCommand {
    // Define your custom method
    //fn assert_success<F: FnOnce(E) -> T>(&mut self, op: F) -> T;
    fn success_or_panic(&mut self) -> ExitStatus;
    fn success_or_else<F: FnOnce(ExitStatus) -> ExitStatus>(&mut self, op: F) -> ExitStatus;
    fn status_or_panic(&mut self) -> ExitStatus;
    fn show_what_will_run(&mut self) -> &mut Self;
    fn get_what_will_run(&self) -> (String, usize, String);
}

// Implement the extension trait for Command
impl MyCompilerCommand for Command {
    //TODO: test these functions!
    fn success_or_panic(&mut self) -> ExitStatus {
        self.success_or_else(|exit_status| {
            // Doesn't tell you here which compiler command was used, but
            // self.show_what_will_run() did tell you before it ran it,
            // so it will be above this panic.
            panic!(
                "Compiler failed with exit code '{}'. Is ncurses installed? \
        pkg-config or pkgconf too? it's ncurses-devel on Fedora. \
        Or maybe it failed for different reasons.",
                exit_status
            )
        })
    }
    fn success_or_else<F: FnOnce(ExitStatus) -> ExitStatus>(&mut self, op: F) -> ExitStatus {
        let exit_status: ExitStatus = self.show_what_will_run().status_or_panic();
        if exit_status.success() {
            exit_status
        } else {
            op(exit_status)
        }
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
                let mut broken_arg = String::new();
                use std::fmt::Write;
                use std::os::unix::ffi::OsStrExt;
                for byte in arg.as_bytes() {
                    match std::char::from_u32(*byte as u32) {
                        Some(c) if c.is_ascii() => broken_arg.push(c),
                        _ => write!(&mut broken_arg, "\\x{:02X}", byte).expect("Failed to write"),
                    }
                }
                broken_arg
            })
            .collect::<Vec<String>>()
            .join("\" \"");
        //TODO: maybe a better way to get the args as a Vec<String> and impl Display ? but not
        //for the generic Vec<String> i think. Then, we won't have to return how_many_args!
        return (
            p_prog.to_string(),
            how_many_args,
            format!("\"{}\"", formatted_args),
        );
    }
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
    test_nul_in_arg();
    test_no_panic_in_command();

    eprintln!("\n-------------------------------------
              \n!!! All build.rs tests have passed successfully! Ignore the above seemingly erroneous output, it was part of the successful testing !!!\nYou're seeing this because you tried to build with --features=test_build_rs_of_ncurses_rs");

    // This stops the build from continuing which will fail in other places due to build.rs not
    // doing its job, since we've only just tested build.rs not used it to generate stuff.
    std::process::exit(5);
}

#[allow(dead_code)]
fn test_assert_works() {
    let result = std::panic::catch_unwind(|| {
        assert!(false);
    });
    if result.is_ok() {
        panic!("Assertions are disabled in build.rs, should not happen!");
    }
}

//test functions are left outside of 'test_build_rs_of_ncurses_rs' so they're tested to compile.
#[allow(dead_code)]
fn test_no_panic_in_command() {
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("cc");
        use std::ffi::OsString;
        command.args([OsString::from("ar❤️g1")]);
        command.status_or_panic();
    });
    assert!(result.is_ok(), "This shouldn't have panicked!");
}

#[allow(dead_code)]
fn test_nul_in_arg() {
    let result = std::panic::catch_unwind(|| {
        let mut command = Command::new("test_nul_in_arg.exe");
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        command.args([
            OsString::from("ar❤️g1"),
            OsString::from("a\0rg2"),
            OsString::from_vec(b"my\xffarg3".to_vec()),
        ]);
        command.status_or_panic();
    });
    expect_panic(result,
        "Failed to run compilation command 'test_nul_in_arg.exe' with '3' args: '\"ar\\xE2\\x9D\\xA4\\xEF\\xB8\\x8Fg1\" \"<string-with-nul>\" \"my\\xFFarg3\"', reason: 'nul byte found in provided data'");
}

#[allow(dead_code)]
fn test_invalid_utf8_in_program() {
    let result = std::panic::catch_unwind(|| {
        use std::ffi::OsString;
        let mut command = Command::new(OsString::from_vec(
            b"test_invalid_utf8_\xFFin_program".to_vec(),
        ));
        use std::os::unix::ffi::OsStringExt;
        command.args([
            OsString::from("ar❤️g1"),
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
                expected_panic_message, err,
                "Got different panic message than expected"
            );
        }
    } else {
        panic!(
            "No panic was thrown! But was expecting this panic: '{}'",
            expected_panic_message
        );
    };
}
