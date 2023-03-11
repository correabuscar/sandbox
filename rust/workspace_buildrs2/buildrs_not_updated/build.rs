//use std::env;  

fn main() {
  println!("Hello from build.rs !");
  println!("cargo:rerun-if-env-changed=A"); //if the environment variable's value changes the build script should be rerun.  FIXME: this line is the problem, remove it and build.rs gets recompiled as it should!
//
  //the following code is from https://github.com/mitnk/cicada/blob/5fac888ccc3cef0abc24e2d3bdf1655eddfdbc98/src/build.rs and slightly modified:
  extern crate time;
  use std::process::Command;
  match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
      Ok(x) => {
          let git_hash = String::from_utf8_lossy(&x.stdout);
          println!("cargo:rustc-env=GIT_HASH={}", git_hash);
      }
      Err(e) => {
          panic!("{}",e);
          //println!("cargo:rustc-env=GIT_HASH={:?}", e);
      }
  }
  //let tm = time::now();
  let tm = time::OffsetDateTime::now_utc();
  let tmf=tm.format(&time::format_description::well_known::Rfc2822).unwrap();
  //println!("cargo:rustc-env=BUILD_DATE={}", tm.to_utc().rfc822()); //used by src/main.rs
  println!("cargo:rustc-env=BUILD_DATE={}", tmf); //used by src/main.rs
}
