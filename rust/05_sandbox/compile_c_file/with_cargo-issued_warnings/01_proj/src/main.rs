extern crate libc;

//use libc::c_void;

// Declare the external function
extern "C" {
    fn deprecated_function();
}

fn main() {
    // Call the deprecated function
    unsafe {
        deprecated_function();
    }
}

