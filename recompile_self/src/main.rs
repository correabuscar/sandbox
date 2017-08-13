//use std::env;

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

    //TODO: detect if source changed!
    for each in &sources {
    }
    print!("Recompiling executable due to source changed...");
    let output=std::process::Command::new("cargo")
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
}
