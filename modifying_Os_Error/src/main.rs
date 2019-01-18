#![allow(unused)]
#![warn(dead_code)]
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {:?}", inner_err);
    } else {
        println!("No inner error");
    }
}

trait SEI {
  //fn set_extra_info(&self, s: &str) -> Error;
  fn set_extra_info(self, s: &str) -> Self;
}

//TODO: use a lazy_static!() to init only once an env! call to get some env. var's value like
//RUST_PEDANTIC=1 and only if non-zero add the extra error info / execute the block that adds it !
//example lazy_static! (unless I get to read rustbook about it by then): https://github.com/fortanix/rust-sgx/blob/jb/sgx-detect/sgxs-tools/src/sgx_detect/proc_macro.rs#L122-L294
//
impl SEI for Error {
  //fn set_extra_info(&self, s: &str) -> Error{
  fn set_extra_info(self, s: &str) -> Self{ //TODO: define this in src/libstd/io/error.rs
      //self.repr=Repr::Os(5);
      //^ repr is still private, obviously!!!!!
      eprintln!("Setting extra info '{}' for '{:?}'",s,self);
      self
  }
}

fn main() -> std::io::Result<()> {
    let r#fn = "inexistent";
    fs::remove_file(r#fn).map_err(|e| {
        //e.repr=Repr::Os(5);//ErrorKind::PermissionDenied;//modify something inside the error?
        //^ private field!
        e.set_extra_info("blah")
            //e
    })?;
    //meh:
    fs::remove_file(r#fn).map_err(|e| {
        Error::new(
            e.kind(),
            format!("{:?} extrainfo:'{}'", e, format!("filename:{}" , r#fn)),
        )
    })?;

    if let Result::Err(err) = fs::remove_file("unexistent") {
        println!("{:?}", err.kind());
        let custom_error = Error::new(err.kind(), "oh no!");
        print_error(&custom_error);
        return Result::Err(custom_error);
    }
    let path = env::current_dir()?;
    fs::remove_file("unexistent")?;
    println!("The current directory is {}", path.display());
    Ok(())
}

