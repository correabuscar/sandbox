use std::fmt::{Display, self};
use libc::atexit;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU64,Ordering};

static HOOKS: AtomicU64 = AtomicU64::new(0);

//src: https://github.com/rust-lang/rust/issues/97181
//this double panic no longer shows stacktrace due to https://github.com/rust-lang/rust/pull/110975
//and thus doesn't get to execute our user panic hook!

struct MyStruct;

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[allow(dead_code)]
extern "C" fn cleanup() {
//fn cleanup() {
    let hooks = HOOKS.load(Ordering::SeqCst);
    println!("! project's Cleaning up resources before exit... hooks registered={}",hooks);
}

fn set_exit_hook() {
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
    }));
    // Register the exit handler
    unsafe {
        let result = atexit(cleanup);// as extern "C" fn());
        if result != 0 {
            panic!("Failed to register project's exit handler");
        } else {
            HOOKS.fetch_add(1, Ordering::SeqCst);
            let hooks = HOOKS.load(Ordering::SeqCst);
            println!("! registered, so far hooks registered={}",hooks);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_does_stuff() {
        set_exit_hook();
        let instance = MyStruct;

        assert!(false, "oh no, '{}' was unexpected", instance);
    }
}

fn main() {

        set_exit_hook();
        let instance = MyStruct;

        //this double panic used to be catchable, ie. https://github.com/rust-lang/rust/issues/97181#issuecomment-1132157218
        //println!("{} {} {}", false, "oh no, '{}' was unexpected", instance); //this is caught
        assert!(false, "oh no, '{}' was unexpected", instance);
}
