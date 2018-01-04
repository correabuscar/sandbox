//use std::env;  

fn main() {
  println!("Hello from build.rs !");
//  std::process::Command::new("/usr/bin/env").status().expect("failed to run 'env' command");
//  let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var"); //": NotPresent"
//  println!("Here's OUT_DIR={}", out_dir);
//  println!("Here's CARGO_TARGET_BINFILE_FULLPATH={}", env::var("CARGO_TARGET_BINFILE_FULLPATH").unwrap_or("not freakin set".to_string()));
  //std::env::set_var("CARGO_PKG_NAME2","this isn't gonna be seen when compiling my pkg's exe");
  //XXX: oh look at this https://doc.crates.io/build-script.html#outputs-of-the-build-script
  let profile=std::env::var("PROFILE").expect("env var PROFILE");
  //^ eg. debug or release; set by cargo, see: https://doc.crates.io/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
  println!("cargo:rustc-cfg=profile_{}",profile);
  println!("cargo:rustc-env=CARGO_PROFILE={}", profile);
  //^ indicates that the specified environment variable will be added to the environment which the compiler is run within. The value can be then retrieved by the env! macro in the compiled crate. This is useful for embedding additional metadata in crate's code, such as the hash of Git HEAD or the unique identifier of a continuous integration server.
  println!("cargo:rerun-if-env-changed=A"); //if the environment variable's value changes the build script should be rerun. 

  //the following code from https://github.com/mitnk/cicada/blob/5fac888ccc3cef0abc24e2d3bdf1655eddfdbc98/src/build.rs and slightly modified:
  extern crate time;
  use std::process::Command;//1234
  match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
      Ok(x) => {
          let git_hash = String::from_utf8_lossy(&x.stdout);
          println!("cargo:rustc-env=GIT_HASH={}", git_hash);
      }
      Err(e) => {
          panic!(format!("{}",e));
          //println!("cargo:rustc-env=GIT_HASH={:?}", e);
      }
  }
  let tm = time::now();
  println!("cargo:rustc-env=BUILD_DATE={}", tm.to_utc().rfc822());
  println!("cargo:warning=Hey, here's a warning from build.rs, for a reason! noting that BUILD_DATE is {}",tm.to_utc().rfc822()); //is a message that will be printed to the main console after a build script has finished running.
}
