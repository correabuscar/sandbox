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
//extern "C" fn handle_abort(signal: libc::c_int) {
extern "C" fn handle_abort(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void) {
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
    // Set up signal handling for SIGABRT
    // way1:
    let mut sigset: libc::sigset_t = unsafe { std::mem::zeroed() }; // sets it to 0
    unsafe {
        libc::sigemptyset(&mut sigset as *mut libc::sigset_t); //redundant since this set it to 0
    }
    let sig_action = libc::sigaction {
        sa_sigaction: handle_abort as usize, // when you cast a function to usize you're "getting" the memory address of the function; it's common to use usize for function pointers to maintain compatibility with C's function pointer representation.
        //sa_mask: 0, //whoops?
        //sa_mask: sigset,
        sa_mask: unsafe { std::mem::zeroed() }, // No signals blocked during handler execution
        sa_flags: libc::SA_SIGINFO,
        sa_restorer: None,
    };
    //way2: (less clear which fields and why are zeroed) XXX: also assumes None is first variant in Option which if I recompile rust with them swapped would break this! and anything that assumes None is 0 in memory, so other projects, maybe even within rust itself, TODO: for fun, at some point, if ever. Set as todo in: https://github.com/correabuscar/knowhow_and_TODO/blob/main/rust/todo_rust.wofl
    //let mut sig_action: libc::sigaction = unsafe { std::mem::zeroed() }; //sa_mask and sa_restorer are to be 0 and None
    //sig_action.sa_sigaction = handle_abort as usize;
    //sig_action.sa_flags = libc::SA_SIGINFO;
    // Install a signal handler for SIGABRT
    unsafe {
        //libc::signal(libc::SIGABRT, handle_abort as usize);
        // WARNING: the behavior of signal() varies across UNIX versions, and has also varied historically across different versions of Linux.  Avoid its use: use sigaction(2) instead.  See Portability below.
        libc::sigaction(libc::SIGABRT, &sig_action, ptr::null_mut());
    }

    set_panic_hook_too();

    // Trigger an abort
    println!("Triggering abort");
    let e=std::panic::catch_unwind(|| { // because this is planned for libtest which wraps every
                                        // in-process test thread in this.
        #[allow(unused_unsafe)]
        unsafe {
            //libc::abort();//caught
            std::process::abort(); // caught
        };
    });
    println!("Bye from main. {:?}",e);
}

