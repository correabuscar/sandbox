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

