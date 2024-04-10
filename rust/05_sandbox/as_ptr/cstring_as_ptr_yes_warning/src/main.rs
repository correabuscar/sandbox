#![warn(temporary_cstring_as_ptr)]
use std::ffi::CString;

fn foo(p: *const u8) {
        unsafe { println!("Good:{:?}", *p as char);}
}
fn main() {
    let pointer = CString::new("Hello world!").unwrap().as_ptr(); // should WARN
    unsafe {
        println!("Bad:{:?}", *pointer as u8 as char);

        //see: https://github.com/rust-lang/rust/issues/78691
        println!("Good:{:?}", *CString::new("Hello world!").unwrap().as_ptr() as u8 as char); //shouldn't warn
        foo(CString::new("Hello world!").unwrap().as_ptr() as *const u8); // shouldn't warn
    }


}
