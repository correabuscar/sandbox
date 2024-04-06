//use std::sync::{Arc, Mutex};
//use std::thread;
//use std::process;

//// Define boolean flags to track fork events
//#[derive(Default)]
//struct ForkFlags {
//    forked: bool,
//}

fn main() {
//    // Shared state for boolean flags
//    let fork_flags = Arc::new(Mutex::new(ForkFlags::default()));

    // Register fork handlers
    unsafe {
        libc::pthread_atfork(
            Some(prepare),
            Some(parent),
            Some(child),
        );
    }

//    // Spawn a thread to perform some tasks
//    let fork_flags_clone = Arc::clone(&fork_flags);
//    let thread_handle = thread::spawn(move || {
//        // Perform some tasks
//        // For example:
//        // println!("Thread doing some work");
//        // Do some computation...
//
//        // Set forked flag to true in the child process
//        let mut flags = fork_flags_clone.lock().unwrap();
//        flags.forked = true;
//    });
//
//    // Wait for the thread to finish its tasks
//    thread_handle.join().unwrap();
//
//    // Check the forked flag
//    let flags = fork_flags.lock().unwrap();
//    if flags.forked {
//        println!("A fork occurred in the child process.");
//    } else {
//        println!("No fork occurred in the child process.");
//    }
    unsafe {
        libc::fork();
        //libc::vfork(); // doesn't use the hooks from pthread_atfork()
        //extern {
        //    fn _exit(code: i32) -> !;
        //}
        ////std::thread::sleep(std::time::Duration::from_secs(1));
        //_exit(0);
    };
}

// Fork handlers
unsafe extern "C" fn prepare() {
    // You can perform any necessary actions before fork() in the parent process
    eprintln!("!! prepare");
}

unsafe extern "C" fn parent() {
    // You can perform any necessary actions after fork() in the parent process
    eprintln!("!! parent");
}

unsafe extern "C" fn child() {
    // You can perform any necessary actions after fork() in the child process
    std::thread::sleep(std::time::Duration::from_secs(1));
    eprintln!("!! child");
}

