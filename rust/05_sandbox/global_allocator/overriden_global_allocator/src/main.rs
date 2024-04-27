//src: chatgpt 3.5 generated initial example!
use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicBool, Ordering};
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

        //well, it's thread local now!
        //XXX: so this is same var for all threads, not thread-local; TODO: see if this is right
        //But if thread-local is wanted it should wrap an atomic to ensure thread migration can
        //still see the correct bool value(it would if atomic), even tho it's rare that it would happen to see it
        //inconsitently due to thread being moved to different core.
        //So without a thread local, a thread that's within this alloc block, will cause other
        //threads that alloc to skip it, so it's clearly not right. FIXME.
        thread_local! {
            //XXX:shouldn't need to be atomic except we're trying to prevent the case where same thread
            //gets migrated between cores at the "right" times to see cache incoherence (ie. it
            //sees a prev. value not the last one that it set while it was on a diff. core)
            //TODO: actually I'm not sure cache incoherence would happen, I even tried to test for
            //in on qemu emulated ARM; need to read more docs, eg. https://marabos.nl/atomics/memory-ordering.html
            static BEEN_HERE_IN_THIS_THREAD:AtomicBool=AtomicBool::new(false);//inited to false the first time it's
                                                               //encountered!
        }
        //so if it was false set it to true, then do this block:
        //this compare_exchange is atomic (chatgpt confused me before about it not being so)
        const WHAT_WAS:bool=false;
        assert_eq!(false, WHAT_WAS);
        assert_eq!(true, !WHAT_WAS);
        BEEN_HERE_IN_THIS_THREAD.with(|been_here_in_this_thread| {
            //TODO: since this is thread local, there's no need for an atomic set of operations compare-and-exchange
        //TODO: other places in this repo use .swap() and need to use this .compare_exchange()
        if Ok(WHAT_WAS)==been_here_in_this_thread.compare_exchange(WHAT_WAS, !WHAT_WAS/*true*/, Ordering::SeqCst, Ordering::SeqCst) {
            // "Yes, that's correct. The core functionality of compare_and-exchange (compare, exchange if match) is achieved as a single atomic operation. This ensures that even in multi-threaded scenarios, the operation appears indivisible to other threads." src: Gemini (Google's LLM) 28 March 2024
        //if false==BEEN_HERE_IN_THIS_THREAD.swap(true, Ordering::SeqCst) { //XXX: this sets it to true always,
        //even if it were true.
        //if let(_prev)=BEEN_HERE_IN_THIS_THREAD.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {

            //println!("Allocating");
            // Print a message indicating the allocation
            //std::rt::
            //rtprintpanic!( //same as println! so far.
            //println!(
            eprintln!(
//                "allocating");
                "Allocating {} bytes at {:?}", layout.size(), ptr);
            //let _=BEEN_HERE_IN_THIS_THREAD.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
            been_here_in_this_thread.store(false, Ordering::SeqCst);
        } //if
        }//closure of with()
        );//with

        // Return the allocated pointer
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        thread_local! {
            static BEEN_HERE_IN_THIS_THREAD:AtomicBool=AtomicBool::new(false);//inited to false the first time it's
                                                                              //encountered!
        }
        //so if it was false set it to true, then do this block:
        //if Ok(false)==BEEN_HERE_IN_THIS_THREAD.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        const WHAT_WAS:bool=false;
        assert_eq!(false, WHAT_WAS);
        assert_eq!(true, !WHAT_WAS);
        BEEN_HERE_IN_THIS_THREAD.with(|been_here_in_this_thread| {
        if Ok(WHAT_WAS)==been_here_in_this_thread.compare_exchange(WHAT_WAS, !WHAT_WAS/*true*/, Ordering::SeqCst, Ordering::SeqCst) {
        //if false==BEEN_HERE_IN_THIS_THREAD.swap(true, Ordering::SeqCst) {
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
            //let _=BEEN_HERE_IN_THIS_THREAD.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
            been_here_in_this_thread.store(false, Ordering::SeqCst);
        }
        });//with
        self.inner.dealloc(ptr, layout);

        // Print a message indicating the deallocation
        //println!("Deallocating {} bytes at {:?}", layout.size(), ptr);
    }
}

// Define a global instance of the printing allocator
#[global_allocator]
static GLOBAL_ALLOCATOR: PrintingAllocator<std::alloc::System> = PrintingAllocator::new(std::alloc::System);

//fn one() {
//    println!("Hello, world!");//allocates 1024 bytes the first time
//}
fn main() {
    let mut o=std::io::stdout();//XXX: allocates 1024 bytes
    //let e=std::os::unix::stdio::StdErr::new();
    let mut e=std::io::stderr();//XXX: doesn't alloc any bytes!
    //println!("Hello, world!");//allocates 1024 bytes the first time
    //let foo="some formatting";
    //let one=1;
    //println!("Hello, world! {}{}", foo,one);
    //println!("Hello, world2!");
    //println!("Hello, world!");
    //let mut o=std::io::stdout();
    //rtprintpanic!(e, "Hi");
    //rtprintpanic!(o, "Hi");
    //let a=1;
}
