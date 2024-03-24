extern crate libc;

use std::ffi::CString;
use std::ptr;

use std::fmt::{Display, self};

struct MyStruct;
impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
        //let instance = MyStruct;
        //panic!("oh1 no, '{}' was unexpected", instance);
    }
}
// Custom signal handler for SIGABRT
extern "C" fn handle_abort(signal: libc::c_int) {
    println!("Custom abort handler called");

    // Add your custom handling logic here
    let inst = MyStruct;
    panic!("from within my custom abort handler");
    //panic!("from within my custom abort handler {}", inst); // infinite recursion panic

    // Terminate the process
    unsafe { libc::_exit(128+6) };
}



fn main() {
    // Install a signal handler for SIGABRT
    unsafe {
        libc::signal(libc::SIGABRT, handle_abort as usize);
    }

    // Trigger an abort
    println!("Triggering abort");
    let e=std::panic::catch_unwind(|| { // because this is planned for libtest which wraps every
                                        // in-process test thread in this.
        unsafe {
        //libc::abort();//caught
        std::process::abort(); // caught
    };
    });
    println!("Bye from main. {:?}",e);
}

