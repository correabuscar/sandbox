use std::env;

fn main() {
  println!("Hello from build.rs !");
  std::process::Command::new("/usr/bin/env").status().expect("failed to run 'env' command");
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var"); //": NotPresent"
  println!("Here's OUT_DIR={}", out_dir);
  std::env::set_var("CARGO_PKG_NAME2","this isn't gonna be seen when compiling my pkg's exe");
}
