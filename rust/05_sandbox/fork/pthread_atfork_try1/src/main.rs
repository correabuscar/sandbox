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

    //#[cfg(not(unix))]
    #[cfg(not(any(unix, target_os = "fuchsia", target_os = "vxworks")))]
    panic!("There's no fork() on your OS: {}", std::env::consts::OS);
    //compile_error!("There's no fork() on your OS!"); //XXX: can't format it! so can't show own OS
    //compile_error!(concat!("There's no fork() on your OS ", env!("CARGO_CFG_TARGET_FAMILY")));
    //XXX: can't format it in compile_error!() and there's no env.var. at build time that's like
    //CARGO_CFG_TARGET_FAMILY(eg. "unix") which is only available in build.rs
    //see: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    eprintln!("Welcome. There's fork() on this OS: {}", std::env::consts::OS);
    // Register fork handlers
    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(
            Some(prepare),
            Some(parent),
            Some(child),
        );
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (1) failed with error code: {}", result);
        }

        //cummulative addition of handlers:
        let result: libc::c_int = libc::pthread_atfork(
            Some(prepare2),
            Some(parent2),
            Some(child2),
        );
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (2) failed with error code: {}", result);
        }
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
//    XXX: fork is part of 'unix' cfg: https://github.com/rust-lang/libc/blob/a0f5b4b21391252fe38b2df9310dc65e37b07d9f/src/lib.rs#L92C5-L97C25
    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    match unsafe {
        libc::fork()
        //libc::vfork(); // doesn't use the hooks from pthread_atfork()
        //extern {
        //    fn _exit(code: i32) -> !;
        //}
        ////std::thread::sleep(std::time::Duration::from_secs(1));
        //_exit(0);
    } {
        -1 => panic!("Fork failed"),
        0 => {
            // Child process
            println!("Child process");
            // Do child process work...
            std::process::exit(0); // Example of child process exiting
        }
        child_pid => {
            // Parent process
            println!("Parent process, child PID: {}", child_pid);
            // Wait for the specific child process to exit, the easy/safe way.
            wait_for_child(child_pid);
            println!("Child process with PID {} exited", child_pid);
        }
    };//match
}

    fn wait_for_child(child_pid: libc::pid_t) {
    let mut status: libc::c_int = 0;
    loop {
        let result = unsafe { libc::waitpid(child_pid, &mut status, 0) };
        if result == -1 {
            panic!("Error waiting for child process");
        }
        if result == child_pid {
            if libc::WIFEXITED(status) {
                println!("Child process exited with status: {}", libc::WEXITSTATUS(status));
                break;
            }
        }
    }
}

//RETURN VALUE
//      On  success,  pthread_atfork()  returns  zero.  On error, it returns an error number.  pthread_atfork() may be
//      called multiple times by a process to register additional handlers.  The handlers for each phase are called in
//      a specified order: the prepare handlers are called in reverse order of registration; the parent and child han‐
//      dlers are called in the order of registration.


use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
static PREPARED:AtomicBool=AtomicBool::new(false);
static PREPARED2:AtomicBool=AtomicBool::new(false);
static PARENT:AtomicBool=AtomicBool::new(false);
static PARENT2:AtomicBool=AtomicBool::new(false);
//static CHILD:AtomicBool=AtomicBool::new(false);
//static CHILD2:AtomicBool=AtomicBool::new(false);
const FNAME_CHILD1:&str = concat!("/tmp/",env!("CARGO_PKG_NAME"),".FNAME_CHILD1");
const FNAME_CHILD2:&str = concat!("/tmp/",env!("CARGO_PKG_NAME"),".FNAME_CHILD2");
//const DELAY_MILLIS:u64=1000; // 1 sec, wait in first child hook

//cfg_if! {
//if #[cfg(unix)] {
// Fork handlers
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare() {
    //std::thread::sleep(std::time::Duration::from_secs(1));
    // You can perform any necessary actions before fork() in the parent process
    PREPARED.store(true, Ordering::SeqCst);
    eprintln!("!! prepare pid={}",std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent() {
    PARENT.store(true, Ordering::SeqCst);
    // You can perform any necessary actions after fork() in the parent process
    eprintln!("!! parent pid={}",std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child() {
    // You can perform any necessary actions after fork() in the child process
    //std::thread::sleep(std::time::Duration::from_millis(DELAY_MILLIS));
    //CHILD.store(true, Ordering::SeqCst);
    let create_result = std::fs::File::create(FNAME_CHILD1);
    if let Err(err) = create_result {
        panic!("Failed to create file {} to signal that fork reached child hook, err={}", FNAME_CHILD1, err);
    }
    eprintln!("!! child pid={}",std::process::id());
}
//}
//} //#cfg

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare2() {
    PREPARED2.store(true, Ordering::SeqCst);
    // You can perform any necessary actions before fork() in the parent process
    eprintln!("!! prepare2 pid={}",std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent2() {
    // You can perform any necessary actions after fork() in the parent process
    PARENT2.store(true, Ordering::SeqCst);
    eprintln!("!! parent2 pid={}",std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child2() {
    // You can perform any necessary actions after fork() in the child process
    //std::thread::sleep(std::time::Duration::from_secs(1));
    //CHILD2.store(true, Ordering::SeqCst);
    let create_result = std::fs::File::create(FNAME_CHILD2);
    if let Err(err) = create_result {
        panic!("Failed to create file {} to signal that fork reached child2 hook, err={}", FNAME_CHILD2, err);
    }
    //TODO: dedup ^
    eprintln!("!! child2 pid={}",std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
#[test]
fn test_that_pthread_atfork_works_as_expected() {
    // ok this is stupid:
    //// Convert the path to a CString
    //let path_c = std::ffi::CString::new(FNAME_CHILD1).expect("CString conversion failed");
    //// Check if the file or directory at the specified path exists
    //let result = unsafe { libc::faccessat(0, path_c.as_ptr(), libc::W_OK , libc::AT_EACCESS) };
    //assert_eq!(result, 0, "path to file {} doesn't already exist", FNAME_CHILD1);

    //let path_c = std::ffi::CString::new(FNAME_CHILD2).expect("CString conversion failed");
    //// Check if the file or directory at the specified path exists
    //let result = unsafe { libc::faccessat(0, path_c.as_ptr(), libc::W_OK , libc::AT_EACCESS) };
    //let errno_value = unsafe { libc::errno() };
    //assert_eq!(result, 0, "path to file {} doesn't already exist", FNAME_CHILD2);
    let _delete_result = std::fs::remove_file(FNAME_CHILD1);
    //if let Err(err) = delete_result {
    //    panic!("Failed to delete file {}, in preparation for the test, err={}", FNAME_CHILD1, err);
    //}
    let _delete_result = std::fs::remove_file(FNAME_CHILD2);
    //if let Err(err) = delete_result {
    //    panic!("Failed to delete file {}, in preparation for the test, err={}", FNAME_CHILD2, err);
    //}
    //mehTODO: dedup ^
    let metadata_result = std::fs::metadata(FNAME_CHILD1);
    if let Ok(ok) = metadata_result {
        panic!("File {} wasn't already deleted by a prev. call to remove_file() which is very odd!, ok={:?}", FNAME_CHILD1, ok);
    }
    let metadata_result = std::fs::metadata(FNAME_CHILD2);
    if let Ok(ok) = metadata_result {
        panic!("File {} wasn't already deleted by a prev. call to remove_file() which is very odd!, ok={:?}", FNAME_CHILD2, ok);
    }
    //TODO: dedup ^
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(
            Some(prepare),
            Some(parent),
            Some(child),
        );
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (1) failed with error code: {}", result);
        }

        //cummulative addition of handlers:
        let result: libc::c_int = libc::pthread_atfork(
            Some(prepare2),
            Some(parent2),
            Some(child2),
        );
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (2) failed with error code: {}", result);
        }
    }
    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    match unsafe {
        libc::fork()
        /* So, to answer your question, after fork() exits, it is indeed guaranteed that the prepare handlers and parent handlers set by a previous pthread_atfork() have completed execution before control is returned to the parent process from fork(). There's no concurrent execution of the handlers during the fork() operation. The fork() operation itself is blocking, ensuring that the atfork handlers are executed in sequence before the child process is created.
         - chatgpt 3.5 */
    } {
        -1 => panic!("Fork failed"),
        0 => {
            // Child process
            println!("Child process");
            // Do child process work...
            std::process::exit(0); // Example of child process exiting
        }
        child_pid => {
            // Parent process
            println!("Parent process, child PID: {}", child_pid);
            // Wait for the specific child process to exit, the easy/safe way.
            wait_for_child(child_pid);
            println!("Child process with PID {} exited", child_pid);
        }
    };//match
    //panic!("uhm");
    //std::thread::sleep(std::time::Duration::from_secs(2));
    assert_eq!(PREPARED.load(Ordering::SeqCst), true);
    assert_eq!(PREPARED2.load(Ordering::SeqCst), true);
    assert_eq!(PARENT.load(Ordering::SeqCst), true);
    assert_eq!(PARENT2.load(Ordering::SeqCst), true);
    //must wait for fork to finish, the hard way:
    //std::thread::sleep(std::time::Duration::from_millis(DELAY_MILLIS+1000)); //add an extra second
                                                                             //to wtw the delay was
    //XXX: can't use statics for this, as it's a different (forked) process:
    //assert_eq!(CHILD.load(Ordering::SeqCst), true);
    //assert_eq!(CHILD2.load(Ordering::SeqCst), true);
    let metadata_result = std::fs::metadata(FNAME_CHILD1);
    if let Err(err) = metadata_result {
        panic!("Fork didn't execute child hook, as file {} doesn't exist already(fork was supposed to create it in child hook), err={}", FNAME_CHILD1, err);
    }
    let metadata_result = std::fs::metadata(FNAME_CHILD2);
    if let Err(err) = metadata_result {
        panic!("Fork didn't execute child hook, as file {} doesn't exist already(fork was supposed to create it in child2 hook), err={}", FNAME_CHILD2, err);
    }
    //TODO: dedup ^
} //test fn
