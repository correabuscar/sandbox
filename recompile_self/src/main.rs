#[macro_use]
extern crate log;
extern crate filetime;


//use std::env;

// eg. /home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/compiletime_env
const PWD_AT_COMPILETIME: &'static str = env!("CARGO_MANIFEST_DIR");
/*
#[cfg(debug_assertions)] //thanks to Arnavion on irc
const CARGO_MODE: &'static str = //this repetition is necessary
"";
#[cfg(not(debug_assertions))]
const CARGO_MODE: &'static str = //this repetition is necessary
"--release";
*/

//lol my first macro, without reading the docs for how to macro, but used: file:///home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/src/std/macros.rs.html#315
//macro_rules! fflush { ($name:expr) => ({ $name.flush().ok().expect(stringify!(Could not flush $name)); }) }
//"Could not flush $name");}) }

macro_rules! fflush {
    () => ({
        fflush!(stdout);
        fflush!(stderr);
    });
    ($stdwhat:ident) => ({
        use std::io::Write; //XXX: needed for flush() to be seen in scope!
        std::io::$stdwhat().flush().ok().expect(stringify!(Could not flush $stdwhat));
//XXX: how to place $name into str; find a better way?
    });
}

fn main() {


    // FIXME: find better way to detect main.rs and others
    let sources = [std::path::Path::new(&PWD_AT_COMPILETIME).join("src/main.rs")];
    //TODO: use * glob to find all *.rs in src/ ! or something

    // detect if source changed!
    let exe_full_name=std::env::current_exe().unwrap();
    let metadata0 = std::fs::metadata(&exe_full_name).unwrap();
    let mtime0 = filetime::FileTime::from_last_modification_time(&metadata0);
    debug!("old exe mtime={}", mtime0);

    let mut changed=false;
    for each in &sources {
        let metadatax = std::fs::metadata(
            std::path::Path::new(&PWD_AT_COMPILETIME)
            .join(each)
            ).unwrap();
        let mtimex=filetime::FileTime::from_last_modification_time(&metadatax);
        if mtimex > mtime0 {
            /*//#[cfg(debug_assertions)] {
            if cfg!(debug_assertions) {
                eprintln!("{:?} is newer than {:?}", each, exe_full_name);
            }*/
            debug!("{:?} is newer than {:?}", each, exe_full_name);
            if !changed {changed=true}
        }
    }

    if changed {
        eprint!("!! Recompiling executable due to source changed...");
        //std::io::stdout().flush().ok().expect("Could not flush stdout");
        //fflush!(std::io::stdout());
        fflush!();
        let args=vec!["build","-v",
        #[cfg(not(debug_assertions))]
        "--release"
        ]; //XXX: nvm: replace with 'run' so we don't have to manually also run it below! Actually NO, because then we have to show stdout/stderr from compilation too!
        /*if !CARGO_MODE.is_empty() {
            args.push(CARGO_MODE);
        }*/
        let output=std::process::Command::new("cargo")
            //FIXME: cargo command is assumed to be in PATH, instead of using CARGO env var.; perhaps
            //it's for the best? but should have a fallback!
            .current_dir(PWD_AT_COMPILETIME)
            .args(&args)
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            eprintln!("all good! {}", output.status);
        }else{
            eprintln!("failed! {}", output.status);
            println!("!! stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("!! stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        //prevent possible endless loop by making sure the timestamps for the old exe and new exe
        //are now different due to compilation placing the new exe in the same place!
        let metadata1 = std::fs::metadata(&exe_full_name).unwrap();
        let mtime1 = filetime::FileTime::from_last_modification_time(&metadata1);
        assert!(mtime0 < mtime1, "old exe mtime {} isn't less than newly compiled exe mtime {}!", mtime0, mtime1);
        //now have to re-execute self
        let child=std::process::Command::new(exe_full_name)
            .args(std::env::args())
            .status()
            .expect("failed to re-execute self after recompilation");
        //exit with the above exit code, to prevent executing the old program
        std::process::exit(child.code().unwrap());
        #[allow(unreachable_code)] { //thanks to mbrubeck for the block idea!
            unreachable!();
        }
    }

    println!("Exe fname is {:?}", exe_full_name);
    //all envs at runtime
    /*    for (key, value) in std::env::vars() {
          println!("{}: {}", key, value);
          }*/

    //one compile time env:
    println!("Hello, world! CARGO_MANIFEST_DIR={}", PWD_AT_COMPILETIME);
    //println!("{}", env!("CARGO_TARGET_DIR"));
    //println!("{}", env!("OUT_DIR"));
}
