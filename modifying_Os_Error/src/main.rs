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

fn main() -> std::io::Result<()> {
    let r#fn = "inexistent";
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

