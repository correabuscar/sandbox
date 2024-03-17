//use std::env;

// eg. /home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/compiletime_env
// aka current dir (coincidentally)
const PWD_AT_COMPILETIME: &'static str = env!("CARGO_MANIFEST_DIR");

fn main() {

    //all envs at runtime
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    //one compile time env:
    println!("Hello, world! {}", PWD_AT_COMPILETIME);

}
