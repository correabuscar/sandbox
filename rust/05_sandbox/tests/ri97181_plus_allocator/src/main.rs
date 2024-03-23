use std::fmt::{Display, self};
use libc::atexit;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU64,Ordering, AtomicBool};
//src: chatgpt 3.5 generated initial example of global allocator override
use std::alloc::{GlobalAlloc, Layout};
//use std::sync::atomic::{AtomicBool, Ordering};
//use std::ptr;

// Define a custom allocator wrapper struct
pub struct PrintingAllocator<A: GlobalAlloc> {
    inner: A,
}

// Implement the custom allocator wrapper
impl<A: GlobalAlloc> PrintingAllocator<A> {
    // Create a new instance of the wrapper
    pub const fn new(inner: A) -> Self {
        PrintingAllocator { inner }
    }
}

// Prints to the "panic output", depending on the platform this may be:
// - the standard error output
// - some dedicated platform specific output
// - nothing (so this macro is a no-op)
macro_rules! rtprintpanic { //src: https://stdrs.dev/nightly/x86_64-pc-windows-gnu/src/std/rt.rs.html#34-40
    ($out:expr, $($t:tt)*) => {
        //let mut out=std::io::stdout();
        //if let Some(mut out) = std::io::stdout { // std::sys::stdio::panic_output() {
            let _ = std::io::Write::write_fmt(&mut $out, format_args!($($t)*));
            let _ = std::io::Write::write_fmt(&mut $out, format_args!("\n")); //FIXME: find a better
                                                                             //way
        //}
    }
}

// Implement the GlobalAlloc trait for the custom allocator wrapper
unsafe impl<A: GlobalAlloc> GlobalAlloc for PrintingAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Call the inner allocator's alloc function
        let ptr = self.inner.alloc(layout);

        static BEEN_HERE:AtomicBool=AtomicBool::new(false);//inited to false the first time it's
                                                           //encountered!
        //so if it was false set it to true, then do this block:
        //this compare_exchange is atomic (chatgpt confused me before about it not being so)
        if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        //if let(_prev)=BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {

            //println!("Allocating");
            // Print a message indicating the allocation
            //std::rt::
            //rtprintpanic!( //same as println! so far.
            //println!(
            eprintln!(
//                "allocating");
                "Allocating {} bytes at {:?}", layout.size(), ptr);
            use std::io::Write;
            let _=std::io::stderr().flush(); //needs: use std::io::Write; else no method found!
            let _=BEEN_HERE.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
        }
        panic!("intentional");
        //let instance = MyStruct;
        //assert!(false, "oh no, '{}' was unexpected", instance);//infinitely nested panics attempt
                                                               //withing alloc

        // Return the allocated pointer
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        static BEEN_HERE:AtomicBool=AtomicBool::new(false);//inited to false the first time it's
                                                           //encountered!
        //so if it was false set it to true, then do this block:
        if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        // Call the inner allocator's dealloc function
            //XXX: can't use println! because it tries to re acquire lock ? ie. println within
            //println? i guess?
            //rtprintpanic!( //won't work, panic
            //println!(//same, panic
            eprintln!(//wtf, eprintln works here, hmm! ok now we know why: XXX: doesn't alloc 1024
                      //bytes like println! does! because std::io::stdout() but not ::stderr() does
                      //alloc those 1024 bytes buffer!
            //dbg!( //works, somehow
                "Deallocating {} bytes at {:?}", layout.size(), ptr);
            let _=BEEN_HERE.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
        }
        self.inner.dealloc(ptr, layout);

        // Print a message indicating the deallocation
        //println!("Deallocating {} bytes at {:?}", layout.size(), ptr);
    }
}

// Define a global instance of the printing allocator
#[global_allocator]
static GLOBAL_ALLOCATOR: PrintingAllocator<std::alloc::System> = PrintingAllocator::new(std::alloc::System);


static HOOKS: AtomicU64 = AtomicU64::new(0);

//src: https://github.com/rust-lang/rust/issues/97181
//this double panic no longer shows stacktrace due to https://github.com/rust-lang/rust/pull/110975
//and thus doesn't get to execute our user panic hook!

struct MyStruct;

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        //todo!(); // good but let's try infinite panics:
        let instance = MyStruct;

        //this double panic used to be catchable, ie. https://github.com/rust-lang/rust/issues/97181#issuecomment-1132157218
        //println!("{} {} {}", false, "oh no, '{}' was unexpected", instance); //this is caught
        static BEEN_HERE_TIMES:AtomicU64=AtomicU64::new(0);
        BEEN_HERE_TIMES.fetch_add(1, Ordering::SeqCst);
        let i = BEEN_HERE_TIMES.load(Ordering::SeqCst);
        assert!(false, "oh no displaynum={:?}, '{}' was unexpected", i,instance);
        panic!("unreachable");
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
        //assert!(false, "oh no, '{}' was unexpected", instance);
}
