use std::cell::Cell;

use std::clone::Clone;
use std::sync::atomic::{AtomicUsize, Ordering};

// Define a newtype wrapper around AtomicUsize
struct TrackedAtomic {
    inner: AtomicUsize,
    initialized: bool,
}

impl TrackedAtomic {
    // Constructor function for TrackedAtomic
    //const fn new(value: usize) -> TrackedAtomic {
    fn new(value: usize) -> TrackedAtomic {
        println!("allocation in pid={:?} tid={:?}", std::process::id(), std::thread::current());
        //dbg!("alloc!");
        TrackedAtomic {
            inner: AtomicUsize::new(value),
            initialized: false,
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
    initialized: bool,
}

impl<T: Copy + Clone> TrackedCell<T> {
    // Constructor function for TrackedCell
    fn new(value: T) -> TrackedCell<T> {
        //hmm, only one allocation happened! and thread local var seems to work properly in each
        //forked process
        println!("allocation in pid={:?} tid={:?}", std::process::id(), std::thread::current());
        TrackedCell {
            inner: Cell::new(value),
            initialized: false,
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
    //static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0);
    //static LOCAL_PANIC_COUNT: TrackedCell<usize> = TrackedCell::new(0);
    static LOCAL_PANIC_COUNT: TrackedAtomic = TrackedAtomic::new(0);
    //static LOCAL_PANIC_COUNT: TrackedAtomic = const { TrackedAtomic::new(0) };//TODO: does this
    //one do any new allocations or what? ie. is this true: https://github.com/rust-lang/rust/commit/8e70c82f572be26a9d838e52f451b270160ffdba#diff-88e2a536317b831c2e958b9205fde12f5edaabefba963bdd3a7503bbdedf8da9R300-R315
    //that "Accessing LOCAL_PANIC_COUNT in a child created by `libc::fork` would lead to a memory allocation."
    //even tho there's a 'const' there.
}

fn main() {
    let main=std::process::id();
    // Fork the process before any inits!
    unsafe {
        libc::fork();
    }
    let who=if main!=std::process::id() {
        "fork"
    } else {
        "main"
    };
    // Accessing the thread-local variable from the main function
    LOCAL_PANIC_COUNT.with(|count| {
        // Modify or access the value of the thread-local variable
        let current_count = count.get();
        count.set(current_count + 1);
        println!("Panic count in {who} process: {} pid={:?} tid={:?}", count.get(), std::process::id(), std::thread::current());
    });

    // Fork the process
//    unsafe {
//        libc::fork();
//    }

    // Accessing the thread-local variable from the child process
    LOCAL_PANIC_COUNT.with(|count| {
        // Modify or access the value of the thread-local variable
        let current_count = count.get();
        count.set(current_count + 1);
        //println!("Panic count in child thread: {} tid={:?}", count.get(), std::thread::current());
        println!("Panic count+1 in {who} process: {} pid={:?} tid={:?}", count.get(), std::process::id(), std::thread::current());
    });
    // Accessing the thread-local variable from the main function
    LOCAL_PANIC_COUNT.with(|count| {
        // Modify or access the value of the thread-local variable
        //let current_count = count.get();
        //count.set(current_count + 1);
        println!("Panic count state in {who} process: {} pid={:?} tid={:?}", count.get(), std::process::id(), std::thread::current());
    });
}

