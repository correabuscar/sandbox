#![warn(temporary_cstring_as_ptr)]

use std::ffi::{CString, NulError};
use std::process::ExitCode;

fn some_function(data: *const i8) {
    unsafe { println!("{}", *data as u8 as char); }
}

fn main() -> Result<ExitCode, NulError> {
    some_function(CString::new("1 this correctly doesn't warn")?.as_ptr());
    some_function(CString::new("2 this warns but it shouldn't").unwrap().as_ptr());
    //see: https://github.com/rust-lang/rust/issues/78691

    Ok(ExitCode::SUCCESS)
}

