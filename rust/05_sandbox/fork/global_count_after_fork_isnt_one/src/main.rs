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

struct MyPanickyStruct;
impl std::fmt::Display for MyPanickyStruct {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!() // double panic, will abort() normally, but we should catch that with sig handler
        //let instance = MyPanickyStruct;;
        //panic!("oh1 no, '{}' was unexpected", instance);
    }
}

  // Custom signal handler for SIGABRT
  //extern "C" fn handle_abort(signal: libc::c_int) {
  extern "C" fn handle_abort(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void) {
      println!("Custom abort handler called");

      // Add your custom handling logic here
//    let inst = MyStruct;
    //panic!("panic from within my custom abort handler");//can't catch this from caller
    //panic!("from within my custom abort handler {}", inst); // infinite recursion panic

    // Terminate the process
    unsafe { libc::_exit(128+6) };
  }

fn set_sig_handler() {
      // Set up signal handling for SIGABRT
      // way1:
      let mut sigset: libc::sigset_t = unsafe { std::mem::zeroed() }; // sets it to 0
      unsafe {
          libc::sigemptyset(&mut sigset as *mut libc::sigset_t); //redundant since this set it to 0
      }
      let sig_action = libc::sigaction {
          sa_sigaction: handle_abort as usize, // when you cast a function to usize you're "getting" the memory address   of the function; it's common to use usize for function pointers to maintain compatibility with C's function pointer re  presentation.
          //sa_mask: 0, //whoops?
          //sa_mask: sigset,
          sa_mask: unsafe { std::mem::zeroed() }, // No signals blocked during handler execution
          sa_flags: libc::SA_SIGINFO,
          sa_restorer: None,
      };
      //way2: (less clear which fields and why are zeroed) XXX: also assumes None is first variant in Option which if I   recompile rust with them swapped would break this! and anything that assumes None is 0 in memory, so other projects, m  aybe even within rust itself, TODO: for fun, at some point, if ever. Set as todo in: https://github.com/correabuscar/k  nowhow_and_TODO/blob/main/rust/todo_rust.wofl
      //let mut sig_action: libc::sigaction = unsafe { std::mem::zeroed() }; //sa_mask and sa_restorer are to be 0 and N  one
      //sig_action.sa_sigaction = handle_abort as usize;
      //sig_action.sa_flags = libc::SA_SIGINFO;
      // Install a signal handler for SIGABRT
      unsafe {
          //libc::signal(libc::SIGABRT, handle_abort as usize);
          // WARNING: the behavior of signal() varies across UNIX versions, and has also varied historically across diff  erent versions of Linux.  Avoid its use: use sigaction(2) instead.  See Portability below.
		  use std::ptr;
          libc::sigaction(libc::SIGABRT, &sig_action, ptr::null_mut());
      }
}

fn set_atexit() {
    extern "C" {
        pub fn atexit(callback: extern "C" fn()) -> std::os::raw::c_int;
    }

    #[allow(dead_code)]
    extern "C" fn cleanup() {
        eprintln!("In atexit handler..");
        panic!("panic from atexit handler");
    }
    unsafe {
        let result = atexit(cleanup);
        if result != 0 {
            panic!("failed to register atexit handler");
        } else {
            eprintln!("Registered atexit handler");
        }
    }

}

fn main() {
    set_atexit();
    //std::process::exit(2);
//    let _=catch_unwind(|| {
//
//        panic!("panic1");
//    });
//    let _=catch_unwind(|| {
//        panic!("panic2");
//    });
    let o = std::io::stdout();//allocate 1024 bytes before fork. (println!() does this first time)
    let main=std::process::id();
	set_sig_handler();
    std::panic::set_hook(Box::new(|_panic_info| {
        // This closure will be called when a panic occurs
        eprintln!("Custom Panic Hook, panic occurred");//: {}", panic_info);
        eprintln!("sleeping 600 sec");
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("done sleeping 600 sec");


    }));


    for i in 2..=2-1 +3 {//spawns +3 threads, so main being +1, we're at 4 after this
    let h=std::thread::spawn(|| {
    //mess with global panic count before the fork!
    let _=
        std::panic::catch_unwind(|| {
        //    let inst=MyPanickyStruct;
            panic!("simple main panic");
    });
    });
    std::thread::sleep(std::time::Duration::from_millis(200));//wait before spawning next thread
    }//for

    let h5=std::thread::spawn(move || { //spawn a 5th thread from main, then fork within the thread
    // Fork the process before any inits!
    unsafe {
        libc::fork();
    }
    let who=if main!=std::process::id() {
        std::thread::sleep(std::time::Duration::from_millis(2200));
        "fork"
    } else {
        //std::thread::sleep(std::time::Duration::from_millis(1200));
        "main(but in a 5th thread)"
    };
    println!("@@@ {who} says hi!");
//    let _=if who=="main" {
//        std::panic::catch_unwind(|| {
//            let inst=MyPanickyStruct;
//        panic!("on purpose in main {}",inst);//TODO: find out why if we panic for real in main, the child
//                                     //seems to die or is it it's stdout/stderr no longer point to
//                                     //same place? seemed so from 'mc', not completely sure tho.
//    })
//    }else{
        panic!("panic on purpose in fork");
//    };
    });//spawn

    let who="main process";
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

const SHOW_ALLOCS:bool=false;
const SHOW_DEALLOCS:bool=false;

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
