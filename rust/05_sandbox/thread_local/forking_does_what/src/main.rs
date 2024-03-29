use std::cell::Cell;

use std::clone::Clone;
use std::sync::atomic::{AtomicUsize, Ordering};

// Define a newtype wrapper around AtomicUsize
struct TrackedAtomic {
    inner: AtomicUsize,
}

impl TrackedAtomic {
    // Constructor function for TrackedAtomic
    //const fn new(value: usize) -> TrackedAtomic {
    fn new(value: usize) -> TrackedAtomic {
        eprintln!("allocation(is it tho?) in pid={:?} tid={:?}", std::process::id(), std::thread::current());
        //dbg!("alloc!");
        TrackedAtomic {
            inner: AtomicUsize::new(value),
        }
    }

    // Accessor function for getting the inner value
    fn get(&self) -> usize {
        self.inner.load(Ordering::SeqCst)
    }

    // Mutator function for setting the inner value
    fn set(&self, value: usize) {
        self.inner.store(value as usize, Ordering::SeqCst);
    }
}

// Define a newtype wrapper around Cell
struct TrackedCell<T: Copy + Clone> {
    inner: Cell<T>,
}

impl<T: Copy + Clone> TrackedCell<T> {
    // Constructor function for TrackedCell
    fn new(value: T) -> TrackedCell<T> {
        //hmm, only one allocation happened! and thread local var seems to work properly in each
        //forked process
        println!("allocation in pid={:?} tid={:?}", std::process::id(), std::thread::current());
        TrackedCell {
            inner: Cell::new(value),
        }
    }

    // Accessor function for getting the inner value
    fn get(&self) -> T {
        self.inner.get()
    }

    // Mutator function for setting the inner value
    fn set(&self, value: T) {
        self.inner.set(value);
    }
}


// Define the static thread-local variable outside of any function
thread_local! {
    //static ATOMIC_COUNT: Cell<usize> = Cell::new(0);
    //static ATOMIC_COUNT: TrackedCell<usize> = TrackedCell::new(0);
    static ATOMIC_COUNT: TrackedAtomic = TrackedAtomic::new(0);
    //static ATOMIC_COUNT: TrackedAtomic = const { TrackedAtomic::new(0) };//TODO: does this
    //one do any new allocations or what? ie. is this true: https://github.com/rust-lang/rust/commit/8e70c82f572be26a9d838e52f451b270160ffdba#diff-88e2a536317b831c2e958b9205fde12f5edaabefba963bdd3a7503bbdedf8da9R300-R315
    //that "Accessing ATOMIC_COUNT in a child created by `libc::fork` would lead to a memory allocation."
    //even tho there's a 'const' there.
}

//static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
//    let _=catch_unwind(|| {
//
//        panic!("panic1");
//    });
//    let _=catch_unwind(|| {
//        panic!("panic2");
//    });
    let o = std::io::stdout();//allocate 1024 bytes before fork. (println!() does this first time)
    let main=std::process::id();
    // Fork the process before any inits!
    unsafe {
        libc::fork();
    }
    let who=if main!=std::process::id() {
        std::thread::sleep(std::time::Duration::from_millis(200));
        "fork"
    } else {
        "main"
    };
    // Accessing the thread-local variable from the main function
    ATOMIC_COUNT.with(|count| {
        // Modify or access the value of the thread-local variable
        let current_count = count.get();
        count.set(current_count + 1);
        println!("Atomic count in {who} process: {} pid={:?} tid={:?}", count.get(), std::process::id(), std::thread::current());
    });

    // Fork the process
//    unsafe {
//        libc::fork();
//    }

    // Accessing the thread-local variable from the child process
    ATOMIC_COUNT.with(|count| {
        // Modify or access the value of the thread-local variable
        let current_count = count.get();
        count.set(current_count + 1);
        //println!("Atomic count in child thread: {} tid={:?}", count.get(), std::thread::current());
        println!("Atomic count+1 in {who} process: {} pid={:?} tid={:?}", count.get(), std::process::id(), std::thread::current());
    });
    // Accessing the thread-local variable from the main function
    ATOMIC_COUNT.with(|count| {
        // Modify or access the value of the thread-local variable
        //let current_count = count.get();
        //count.set(current_count + 1);
        println!("Atomic count state in {who} process: {} pid={:?} tid={:?}", count.get(), std::process::id(), std::thread::current());
    });
    println!("{who} is done!");
    std::thread::sleep(std::time::Duration::from_millis(1000));
}

use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::AtomicBool;
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
const SHOW_ALLOCS:bool=true;
const SHOW_DEALLOCS:bool=true;

// Implement the GlobalAlloc trait for the custom allocator wrapper
unsafe impl<A: GlobalAlloc> GlobalAlloc for PrintingAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Call the inner allocator's alloc function
        let ptr = self.inner.alloc(layout);

        static BEEN_HERE:AtomicBool=AtomicBool::new(!SHOW_ALLOCS);//inited to false the first time it's
                                                           //encountered!
        //so if it was false set it to true, then do this block:
        //this compare_exchange is atomic (chatgpt confused me before about it not being so)
        //if Ok(false)==BEEN_HERE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        if false==BEEN_HERE.swap(true, Ordering::SeqCst) {
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
            //let _=BEEN_HERE.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
            BEEN_HERE.store(!SHOW_ALLOCS, Ordering::SeqCst);
        }
        //panic!("intentional");
        //let instance = MyStruct;
        //assert!(false, "oh no, '{}' was unexpected", instance);//infinitely nested panics attempt
                                                               //withing alloc

        // Return the allocated pointer
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        static BEEN_HERE:AtomicBool=AtomicBool::new(!SHOW_DEALLOCS);//inited to false the first time it's
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
            BEEN_HERE.store(!SHOW_DEALLOCS, Ordering::SeqCst);
        }
        self.inner.dealloc(ptr, layout);

        // Print a message indicating the deallocation
        //println!("Deallocating {} bytes at {:?}", layout.size(), ptr);
    }
}


// Define a global instance of the printing allocator
#[global_allocator]
static GLOBAL_ALLOCATOR: PrintingAllocator<std::alloc::System> = PrintingAllocator::new(std::alloc::System);
