use std::env;

fn main() {
  println!("Hello from build.rs !");
  std::process::Command::new("/usr/bin/env").status().expect("failed to run 'env' command");
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var"); //": NotPresent"
  println!("Here's OUT_DIR={}", out_dir);
  println!("Here's CARGO_TARGET_BINFILE_FULLPATH={}", env::var("CARGO_TARGET_BINFILE_FULLPATH").unwrap_or("not freakin set".to_string()));
  //std::env::set_var("CARGO_PKG_NAME2","this isn't gonna be seen when compiling my pkg's exe");
}
