//use std::cell::Cell;
//use std::thread::LocalKey;
use std::sync::atomic::{AtomicUsize, Ordering};

//static COUNTER_KEY: LocalKey<Cell<usize>> = LocalKey::new(Cell::new(0)); // Allocation happens here (outside function)
thread_local! {
  static COUNTER: AtomicUsize = AtomicUsize::new(0);
  //apparently doesn't alloc, at least not by using global allocator?! but it should be allocating
  //somehow at the C level maybe?
}

fn increment_counter() {
  COUNTER.with(|counter| counter.fetch_add(1, Ordering::Relaxed));
}

fn get_counter_value() -> usize {
  COUNTER.with(|counter| counter.load(Ordering::Relaxed))
}

//src: chatgpt 3.5 generated initial example!
use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::AtomicBool;
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
            //let _=BEEN_HERE.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst);
            BEEN_HERE.store(false, Ordering::SeqCst);
        }

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
        self.inner.dealloc(ptr, layout);

        // Print a message indicating the deallocation
        //println!("Deallocating {} bytes at {:?}", layout.size(), ptr);
    }
}

// Define a global instance of the printing allocator
#[global_allocator]
static GLOBAL_ALLOCATOR: PrintingAllocator<std::alloc::System> = PrintingAllocator::new(std::alloc::System);

//#[test]
//fn test_counter_no_allocations() {
//  with_mock_allocator(|| {
//    increment_counter();
//    get_counter_value();
//  });
//  // Test passes if no panics occurred within the closure
//}
#[test]
fn test_no_allocations_on_counter_access() {
  //with_mock_allocator(|| { // yeah, i wish this was a thing!
    // Code using the thread-local counter (functions like increment_counter and get_counter_value)
    increment_counter();
    let count = get_counter_value();
    println!("Thread local counter: {}", count);

    // Spawn a thread (optional)
    let mut counter_thread = std::thread::spawn(|| {
      increment_counter();
      println!("Thread counter value: {}", get_counter_value());
    });

    counter_thread.join().unwrap();
  //});
}

fn main() {
    let mut o=std::io::stdout();//XXX: allocates 1024 bytes
//  // Example usage
//  increment_counter();
//  let count = get_counter_value();
//  println!("Thread local counter: {}", count);

  // Main thread also increments and prints counter
    eprintln!("Main started");
    let dur=std::time::Duration::from_secs(1);
    std::thread::sleep(dur);
  increment_counter();
  eprintln!("Main thread 0+1 counter value: {}", get_counter_value());
    std::thread::sleep(dur);

  let counter_thread = std::thread::spawn(move|| {
    eprintln!("Thread started(this allocates, apparently)");
    std::thread::sleep(dur);
    increment_counter();
    std::thread::sleep(dur);
    eprintln!("Thread counter +1 value: {}", get_counter_value());
  });
    std::thread::sleep(dur);

  // Main thread also increments and prints counter
  increment_counter();
  eprintln!("Main thread +1 counter value: {}", get_counter_value());

  counter_thread.join().unwrap();
  eprintln!("Main thread counter value after join: {}", get_counter_value());
}

