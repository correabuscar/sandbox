//mod ffi;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let value = access_thread_local_var();
    println!("Thread-local value: {}", value);
    set_thread_local_var2(20);
    let value = access_thread_local_var();
    println!("Thread-local value: {}", value);

    // Spawn a new thread
    let handle = thread::spawn(|| {
        // This closure represents the code that will run in the new thread
        for i in 1..=5 {
            let value = access_thread_local_var();
            println!("Hello from the spawned thread! Message {} ltvar={}", i,value);
            thread::sleep(Duration::from_millis(500)); // Sleep for 500 milliseconds
            set_thread_local_var2(10*i);
        }
    });

    // Main thread continues executing concurrently with the spawned thread
    for i in 1..=3 {
        let value = access_thread_local_var();
        println!("Hello from the main thread! Message {} ltvar={}", i,value);
        thread::sleep(Duration::from_millis(1000)); // Sleep for 1 second
        set_thread_local_var2(i);
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    println!("Main thread exiting.");
}
//
// Link against the C shared library
//#[link(name = "tl")] //looks like this isn't needed!
//extern {}

// Rust code (lib.rs)
extern {
    // Define an external function to access the thread-local variable
    fn get_thread_local_var() -> *mut i32;
    fn set_thread_local_var(value: i32);
}

// Function to access the thread-local variable from Rust
pub fn access_thread_local_var() -> i32 {
    unsafe {
        // Call the external function to get a pointer to the thread-local variable
        let ptr = get_thread_local_var();
        // Dereference the pointer to access the value of the thread-local variable
        *ptr
    }
}

// Function to set the value of the thread-local variable from Rust
pub fn set_thread_local_var2(value: i32) {
    unsafe {
        // Call the external function to set the value of the thread-local variable
        set_thread_local_var(value);
    }
}



