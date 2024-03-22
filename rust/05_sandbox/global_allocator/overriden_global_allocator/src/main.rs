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
    ($($t:tt)*) => {
        let mut out=std::io::stdout();
        //if let Some(mut out) = std::io::stdout { // std::sys::stdio::panic_output() {
            let _ = std::io::Write::write_fmt(&mut out, format_args!($($t)*));
            let _ = std::io::Write::write_fmt(&mut out, format_args!("\n")); //FIXME: find a better
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
        //FIXME: this compare_exchange isn't atomic itself, hence can return Err() and we'd miss an
        //alloc this way!
        if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        //if let(_prev)=BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {

            //println!("Allocating");
            // Print a message indicating the allocation
            //std::rt::
            //rtprintpanic!( //same as println! so far.
            //println!(
            eprintln!(
                "Allocating {} bytes at {:?}", layout.size(), ptr);
            let _=BEEN_HERE.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
        }

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
            eprintln!(//wtf, eprintln works here, hmm!
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

fn main() {
    let foo="some formatting";
    println!("Hello, world! {}", foo);
    //println!("Hello, world!");
}
