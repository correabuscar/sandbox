//src: chatgpt 3.5 generated initial example!
use std::io::Write;
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
#[allow(unused_macros)]
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
        //if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        //FIXME: need mutex protected block? eg. enter it if not held, skip it if held, but if
        //concurrent threads want to get in, they should wait in line, not one get in and other
        //skip!
        if false==BEEN_HERE.swap(true, Ordering::SeqCst) {
        //if let(_prev)=BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {

            //println!("Allocating");
            // Print a message indicating the allocation
            //std::rt::
            //rtprintpanic!( //same as println! so far.
            //println!(
            eprintln!(
//                "allocating");
                "Allocating {} bytes at {:?} (+sleeping)", layout.size(), ptr);
            std::thread::sleep(std::time::Duration::from_millis(100));
//            let backtrace=std::backtrace::Backtrace::force_capture();
//            let frames=backtrace.frames();//needs nightly
            //eprintln!("bt={}",std::backtrace::Backtrace::force_capture());
            BEEN_HERE.store(false, Ordering::SeqCst);
        }
        //panic!("panic in alloc, on purpose");

        // Return the allocated pointer
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        static BEEN_HERE:AtomicBool=AtomicBool::new(false);//inited to false the first time it's
                                                           //encountered!
        //so if it was false set it to true, then do this block:
        //if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        if false==BEEN_HERE.swap(true, Ordering::SeqCst) {
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
            //let _=BEEN_HERE.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
            BEEN_HERE.store(false, Ordering::SeqCst);
        }
        //panic!("panic in dealloc, on purpose");
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
    println!("Hello, world!");//allocates 1024 bytes the first time
    //let foo="some formatting";
    //let one=1;
    //println!("Hello, world! {}{}", foo,one);
    //println!("Hello, world2!");
    //println!("Hello, world!");
    //let mut o=std::io::stdout();
    //rtprintpanic!(e, "Hi");
    //rtprintpanic!(o, "Hi");
    //let a=1;
    //eprintln!("bt={}",std::backtrace::Backtrace::force_capture());
    let handler = std::thread::spawn(|| {
        // thread code
        // Create a dynamic integer variable
        let dynamic_int = Box::new(42);

        // Print the value stored in the dynamic integer
        println!("Dynamic integer value: {}", dynamic_int);

        // You can also use the dereference operator (*) to access the value inside the box
        println!("Dereferenced dynamic integer value: {}", *dynamic_int);
    });

    handler.join().unwrap();
    let _ = e.flush();
    let _ = o.flush();
    println!("Bye from main!");
}
