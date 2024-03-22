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

// Implement the GlobalAlloc trait for the custom allocator wrapper
unsafe impl<A: GlobalAlloc> GlobalAlloc for PrintingAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Call the inner allocator's alloc function
        let ptr = self.inner.alloc(layout);

        static BEEN_HERE:AtomicBool=AtomicBool::new(false);//inited to false the first time it's
                                                           //encountered!
        //so if it was false set it to true, then do this block:
        if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        //if let(_prev)=BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {

            //println!("Allocating");
            // Print a message indicating the allocation
            println!("Allocating {} bytes at {:?}", layout.size(), ptr);
        }

        // Return the allocated pointer
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Call the inner allocator's dealloc function
        self.inner.dealloc(ptr, layout);

        // Print a message indicating the deallocation
        //println!("Deallocating {} bytes at {:?}", layout.size(), ptr);
    }
}

// Define a global instance of the printing allocator
#[global_allocator]
static GLOBAL_ALLOCATOR: PrintingAllocator<std::alloc::System> = PrintingAllocator::new(std::alloc::System);

fn main() {
    //let foo="some formatting";
    //println!("Hello, world! {}", foo);
    println!("Hello, world!");
}
