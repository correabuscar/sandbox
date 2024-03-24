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
    panic!("panic from within my custom abort handler");
    //panic!("from within my custom abort handler {}", inst); // infinite recursion panic

    // Terminate the process
    unsafe { libc::_exit(128+6) };
}


fn set_panic_hook_too() {
    use std::cell::RefCell;
    std::panic::set_hook(Box::new(move |panic_info| {
        println!("Custom panic handler starting!");
        thread_local! {
            static CALLED_ONCE: RefCell<bool> = RefCell::new(false);
        }
        let double_panic=
        // Check if CALLED_ONCE is true, if yes, abort, otherwise set it to true
            CALLED_ONCE.with(|called_once| {
            let mut called_once = called_once.borrow_mut();
            if *called_once {
                true
            } else {
                *called_once = true;
                false
            }
        });

        let double_text=
            if double_panic {
                "double "
            } else {
                ""
            };
        // Print the panic message
        println!("Custom panic handler processing payload");
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            println!("Custom panic handler caught {}panic: {}", double_text,message);
        } else {
            println!("Custom panic handler caught {}panic",double_text);
        }

        // Print a backtrace if available
        if let Some(location) = panic_info.location() {
            println!("Panic occurred in file '{}' at line {}", location.file(), location.line());
            println!("{}", std::backtrace::Backtrace::capture());
        }
        if double_panic {
            println!("Aborting the process due to double panic detected...");
            println!("{}", std::backtrace::Backtrace::capture());
        }
        CALLED_ONCE.with(|c| *c.borrow_mut() = false);
        println!("Custom panic handler finished.");
    }));
}

fn main() {
    // Install a signal handler for SIGABRT
    unsafe {
        libc::signal(libc::SIGABRT, handle_abort as usize);
    }

    set_panic_hook_too();

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

