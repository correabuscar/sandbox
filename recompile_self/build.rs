use std::env;

fn main() {
  println!("Hello from build.rs !");
//  std::process::Command::new("/usr/bin/env").status().expect("failed to run 'env' command");
//  let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var"); //": NotPresent"
//  println!("Here's OUT_DIR={}", out_dir);
//  println!("Here's CARGO_TARGET_BINFILE_FULLPATH={}", env::var("CARGO_TARGET_BINFILE_FULLPATH").unwrap_or("not freakin set".to_string()));
  //std::env::set_var("CARGO_PKG_NAME2","this isn't gonna be seen when compiling my pkg's exe");
  //XXX: oh look at this https://doc.crates.io/build-script.html#outputs-of-the-build-script
  println!("cargo:rustc-env=CARGO_TARGET={}", std::env::var("TARGET").expect("env var TARGET"));
  //^ indicates that the specified environment variable will be added to the environment which the compiler is run within. The value can be then retrieved by the env! macro in the compiled crate. This is useful for embedding additional metadata in crate's code, such as the hash of Git HEAD or the unique identifier of a continuous integration server.
  println!("cargo:rerun-if-env-changed=A"); //if the environment variable's value changes the build script should be rerun. 
  println!("cargo:warning=Hey, here's a warning from build.rs, for no reason!"); //is a message that will be printed to the main console after a build script has finished running.
}
