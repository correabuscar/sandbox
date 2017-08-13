//use std::env;
extern crate filetime;

// eg. /home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/compiletime_env
const PWD_AT_COMPILETIME: &'static str = env!("CARGO_MANIFEST_DIR");

fn main() {

    //all envs at runtime
    /*    for (key, value) in std::env::vars() {
          println!("{}: {}", key, value);
          }*/

    //one compile time env:
    println!("Hello, world! {}", PWD_AT_COMPILETIME);
    //println!("{}", env!("CARGO_TARGET_DIR"));
    //println!("{}", env!("OUT_DIR"));

    // FIXME: find better way to detect main.rs and others
    let sources = [std::path::Path::new(&PWD_AT_COMPILETIME).join("src/main.rs")];
    //TODO: use * glob to find all *.rs in src/ ! or something

    //TODO: detect if source changed!
    let exe_full_name=std::env::current_exe().unwrap();
    let metadata0 = std::fs::metadata(&exe_full_name).unwrap();
    let mtime0 = filetime::FileTime::from_last_modification_time(&metadata0);
    //println!("{}", mtime0);

    let mut changed=false;
    for each in &sources {
        let metadatax = std::fs::metadata(
            std::path::Path::new(&PWD_AT_COMPILETIME)
            .join(each)
            ).unwrap();
        let mtimex=filetime::FileTime::from_last_modification_time(&metadatax);
        if mtimex > mtime0 {
            eprintln!("{:?} is newer than {:?}", each, exe_full_name);
            if !changed {changed=true}
        }
    }

    if changed {
        print!("Recompiling executable due to source changed...");
        let output=std::process::Command::new("cargo")
            //FIXME: cargo command is assumed to be in PATH, instead of using CARGO env var.; perhaps
            //it's for the best? but should have a fallback!
            .current_dir(PWD_AT_COMPILETIME)
            .args(&["build","--release"])
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            println!("status: {}", output.status);
        }else{
            println!("all good!");
        }
        //prevent possible endless loop by making sure the timestamps for the old exe and new exe
        //are now different due to compilation placing the new exe in the same place!
        let metadata1 = std::fs::metadata(&exe_full_name).unwrap();
        let mtime1 = filetime::FileTime::from_last_modification_time(&metadata1);
        assert!(mtime0 < mtime1, "old exe mtime isn't less than newly compiled exe mtime!");
        //now have to re-execute self
        let child=std::process::Command::new(exe_full_name)
            .args(std::env::args())
            .status()
            .expect("failed to re-execute self after recompilation");
        //exit with the above exit code, to prevent executing the old program
        std::process::exit(child.code().unwrap());
        #[allow(unreachable_code)]
        println!("moo");
        #[allow(unreachable_code)]
        unreachable!();
    }

    println!("Exe fname is '{:?}'", exe_full_name);
}
