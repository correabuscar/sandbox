#![feature(lazy_cell)]
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
    { //block to not have to repeat the 'cfg' but this runs drop() sooner than main() exit!ie. at
      //end of this block!
    eprintln!(
        "Welcome. There's fork() and pthread_atfork() on this OS: {}",
        std::env::consts::OS
    );
    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    //wipe_tempfiles();
    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    //ensure_files_are_deleted();
    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    let mut deferrer: Defer<_> = Defer::new(|| {
        wipe_tempfiles();
        ensure_files_are_deleted();
    });
    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    deferrer.execute();

    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    //let mut _deferred_execution: Option<Defer<_>> = None;

    // Register fork handlers
    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(Some(prepare), Some(parent), Some(child));
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (1) failed with error code: {}", result);
        }

        //cummulative addition of handlers:
        let result: libc::c_int = libc::pthread_atfork(Some(prepare2), Some(parent2), Some(child2));
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
    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
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
            deferrer.cancel();
            // Child process
            println!("Child process");
            // Do child process work...
            //std::mem::drop(deferrer);//manually drop
            //XXX: this exit() bypasses all(?) drop()s; so deferrer.cancel() isn't needed really
            std::process::exit(0); // Example of child process exiting
        }
        child_pid => {
            //_deferred_execution = Some(Defer::new(|| {
            //    wipe_tempfiles();
            //    ensure_files_are_deleted();
            //}));
            // Parent process
            println!("Parent process, child PID: {}", child_pid);
            // Wait for the specific child process to exit, the easy/safe way.
            let has_exit_code = wait_for_child(child_pid);
            println!(
                "Child process with PID {} exited with exit code: {:?}",
                child_pid, has_exit_code
            );
        }
    }; //match
    } // end block
} //main

fn wait_for_child(child_pid: libc::pid_t) -> Option<libc::c_int> {
    const TIMEOUT_SECS: u64 = 5;
    let start_time = std::time::Instant::now();
    loop {
        //TODO: uncertain why the loop is needed, apparently child receiving a signal and not
        //exiting can make waitpid still exit somehow? uncertain! added 5 sec timeout anyway,
        //safer than forever loop.
        let elapsed_time = start_time.elapsed().as_secs();
        if elapsed_time >= TIMEOUT_SECS {
            // Timeout reached
            return None;
        }
        //The waitpid() system call suspends execution of the calling thread until a child specified by pid argument has changed  state.
        let mut status: libc::c_int = 0;
        let result = unsafe { libc::waitpid(child_pid, &mut status, 0) };
        if result == -1 {
            panic!("Error waiting for child process");
        }
        if result == child_pid {
            if libc::WIFEXITED(status) {
                let status = libc::WEXITSTATUS(status);
                println!("Child process exited with status: {}", status);
                return Some(status);
            }
        }

        // Sleep for a short duration before checking again
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

//RETURN VALUE
//      On  success,  pthread_atfork()  returns  zero.  On error, it returns an error number.  pthread_atfork() may be
//      called multiple times by a process to register additional handlers.  The handlers for each phase are called in
//      a specified order: the prepare handlers are called in reverse order of registration; the parent and child han‐
//      dlers are called in the order of registration.

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
static PREPARED: AtomicBool = AtomicBool::new(false);
static PREPARED2: AtomicBool = AtomicBool::new(false);
static PARENT: AtomicBool = AtomicBool::new(false);
static PARENT2: AtomicBool = AtomicBool::new(false);
//static CHILD:AtomicBool=AtomicBool::new(false);
//static CHILD2:AtomicBool=AtomicBool::new(false);
const FNAME_CHILD1: &str = concat!("/tmp/", env!("CARGO_PKG_NAME"), ".FNAME_CHILD1");
const FNAME_CHILD2: &str = concat!("/tmp/", env!("CARGO_PKG_NAME"), ".FNAME_CHILD2");
//const DELAY_MILLIS:u64=1000; // 1 sec, wait in first child hook

//cfg_if! {
//if #[cfg(unix)] {
// Fork handlers
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare() {
    //HOOK_TRACKER.get_or_init(HookTracker::init()).started_executing("prepare");
    HOOK_TRACKER.started_executing("prepare");
    //std::thread::sleep(std::time::Duration::from_secs(1));
    // You can perform any necessary actions before fork() in the parent process
    PREPARED.store(true, Ordering::SeqCst);
    eprintln!("!! prepare pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent() {
    HOOK_TRACKER.started_executing("parent");
    PARENT.store(true, Ordering::SeqCst);
    // You can perform any necessary actions after fork() in the parent process
    eprintln!("!! parent pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child() {
    HOOK_TRACKER.started_executing("child");
    // You can perform any necessary actions after fork() in the child process
    //std::thread::sleep(std::time::Duration::from_millis(DELAY_MILLIS));
    //CHILD.store(true, Ordering::SeqCst);
    let create_result = std::fs::File::create(FNAME_CHILD1);
    if let Err(err) = create_result {
        panic!(
            "Failed to create file {} to signal that fork reached child hook, err={}",
            FNAME_CHILD1, err
        );
    }
    eprintln!("!! child pid={}", std::process::id());
}
//}
//} //#cfg

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare2() {
    HOOK_TRACKER.started_executing("prepare2");
    PREPARED2.store(true, Ordering::SeqCst);
    // You can perform any necessary actions before fork() in the parent process
    eprintln!("!! prepare2 pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent2() {
    HOOK_TRACKER.started_executing("parent2");
    // You can perform any necessary actions after fork() in the parent process
    PARENT2.store(true, Ordering::SeqCst);
    eprintln!("!! parent2 pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child2() {
    HOOK_TRACKER.started_executing("child2");
    // You can perform any necessary actions after fork() in the child process
    //std::thread::sleep(std::time::Duration::from_secs(1));
    //CHILD2.store(true, Ordering::SeqCst);
    let create_result = std::fs::File::create(FNAME_CHILD2);
    if let Err(err) = create_result {
        panic!(
            "Failed to create file {} to signal that fork reached child2 hook, err={}",
            FNAME_CHILD2, err
        );
    }
    //TODO: dedup ^
    eprintln!("!! child2 pid={}", std::process::id());
}

fn wipe_tempfiles() {
    eprintln!("Deleting temp files... pid={}", std::process::id());
    let _ = std::fs::remove_file(FNAME_CHILD1);
    //if let Err(err) = delete_result {
    //    panic!("Failed to delete file {}, in preparation for the test, err={}", FNAME_CHILD1, err);
    //}
    let _ = std::fs::remove_file(FNAME_CHILD2);
    //if let Err(err) = delete_result {
    //    panic!("Failed to delete file {}, in preparation for the test, err={}", FNAME_CHILD2, err);
    //}
}

fn ensure_files_are_deleted() {
    let metadata_result = std::fs::metadata(FNAME_CHILD1);
    if let Ok(ok) = metadata_result {
        panic!("File {} wasn't already deleted by a prev. call to remove_file() which is very odd!, ok={:?}", FNAME_CHILD1, ok);
    }
    let metadata_result = std::fs::metadata(FNAME_CHILD2);
    if let Ok(ok) = metadata_result {
        panic!("File {} wasn't already deleted by a prev. call to remove_file() which is very odd!, ok={:?}", FNAME_CHILD2, ok);
    }
    eprintln!("Ensure temp files are deleted...");
}

//struct Defer<F: FnOnce()>(Option<F>);
//
//impl<F: FnOnce()> Defer<F> {
//    fn new(f: F) -> Self {
//        Defer(Some(f))
//    }
//}
//
//impl<F: FnOnce()> Drop for Defer<F> {
//    fn drop(&mut self) {
//        // If the closure is present, execute it
//        if let Some(f) = self.0.take() {
//            f();
//        }
//    }
//}
struct Defer<F: Fn()>(Option<F>);

impl<F: Fn()> Defer<F> {
    fn new(f: F) -> Self {
        Defer(Some(f))
    }

    fn execute(&mut self) {
        if let Some(ref f) = self.0 {
            f();
        }
    }

    fn cancel(&mut self) {
        self.0 = None;
    }
}

impl<F: Fn()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(ref f) = self.0 {
            f();
        }
    }
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
#[test]
fn test_that_pthread_atfork_works_as_expected() {
    println!(); //a new line

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
    //    let _delete_result = std::fs::remove_file(FNAME_CHILD1);
    //    //if let Err(err) = delete_result {
    //    //    panic!("Failed to delete file {}, in preparation for the test, err={}", FNAME_CHILD1, err);
    //    //}
    //    let _delete_result = std::fs::remove_file(FNAME_CHILD2);
    //if let Err(err) = delete_result {
    //    panic!("Failed to delete file {}, in preparation for the test, err={}", FNAME_CHILD2, err);
    //}
    //mehTODO: dedup ^
    //wipe_tempfiles();
    //ensure_files_are_deleted();
    let mut deferrer: Defer<_> = Defer::new(|| {
        wipe_tempfiles();
        ensure_files_are_deleted();
    });
    deferrer.execute();

    //TODO: dedup ^
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(Some(prepare), Some(parent), Some(child));
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (1) failed with error code: {}", result);
        }

        //cummulative addition of handlers:
        let result: libc::c_int = libc::pthread_atfork(Some(prepare2), Some(parent2), Some(child2));
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (2) failed with error code: {}", result);
        }
    }

    //    // Create a pipe for communication between parent and child
    //    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    //    let (mut parent_read, mut child_write) = match std::io::pipe() {
    //        Ok((r, w)) => (r, w),
    //        Err(err) => {
    //            panic!("Failed to create pipe: {}", err);
    //        }
    //    };

    #[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    match unsafe {
        libc::fork()
        /* So, to answer your question, after fork() exits, it is indeed guaranteed that the prepare handlers and parent handlers set by a previous pthread_atfork() have completed execution before control is returned to the parent process from fork(). There's no concurrent execution of the handlers during the fork() operation. The fork() operation itself is blocking, ensuring that the atfork handlers are executed in sequence before the child process is created.
        - chatgpt 3.5 */
    } {
        // match
        -1 => panic!("Fork failed"),
        0 => {
            deferrer.cancel();
            // Child process
            println!("Child process");
            let hooks_order_seen_in_child=
                //HookTracker::get().lock()
                HOOK_TRACKER.get_executed_hooks();
            println!(
                "inchild:{:?}",
                hooks_order_seen_in_child //HookTracker::get().lock().get_executed_hooks()
                                          //HOOK_TRACKER.get_or_init(|| HookTracker::init()).get_executed_hooks()
            );
            if hooks_order_seen_in_child == ["prepare2", "prepare", "child", "child2"] {
                //success
                std::process::exit(200);
            } else {
                //failed order, unexpected.
                std::process::exit(201);
            }
            //std::process::exit(0); // Example of child process exiting
        }
        child_pid => {
            // Parent process
            println!("Parent process, child PID: {}", child_pid);
            // Wait for the specific child process to exit, the easy/safe way.
            let has_exit_code = wait_for_child(child_pid);
            println!(
                "Child process with PID {} exited with exit code: {:?}",
                child_pid, has_exit_code
            );
            let exit_code = has_exit_code.expect("forked process didn't exit successfully");
            assert_eq!(200, exit_code);
            //XXX: 200 means child's seen this (correct)hook execution order: ["prepare2", "prepare", "child", "child2"]
        }
    }; //match

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
    let values = vec![(FNAME_CHILD1, ""), (FNAME_CHILD2, "2")];
    for &(fname, childnum) in &values {
        let metadata_result = std::fs::metadata(fname);
        if let Err(err) = metadata_result {
            panic!("Fork didn't execute child hook, as file {} doesn't exist already(fork was supposed to create it in child{} hook), err={}", fname, childnum, err);
        }
    } //for

    //let metadata_result = std::fs::metadata(FNAME_CHILD1);
    //if let Err(err) = metadata_result {
    //    panic!("Fork didn't execute child hook, as file {} doesn't exist already(fork was supposed to create it in child hook), err={}", FNAME_CHILD1, err);
    //}
    //let metadata_result = std::fs::metadata(FNAME_CHILD2);
    //if let Err(err) = metadata_result {
    //    panic!("Fork didn't execute child hook, as file {} doesn't exist already(fork was supposed to create it in child2 hook), err={}", FNAME_CHILD2, err);
    //}
    //done1TODO: dedup ^
    //doneFIXME: this test doesn't test order of execution of the handlers
    //TODO: get rid of external crate for lazy_static!() macro.
    let expected_order = vec!["prepare2", "prepare", "parent", "parent2"];
    assert_eq!(
        HOOK_TRACKER //.get_or_init(HookTracker::init())
            //HookTracker::get()
            .get_executed_hooks(),
        expected_order
    );
} //test fn

//use std::fs::{self, File};
////use std::io;
//
//struct TemporaryFile {
//    path: String,
//    should_cleanup: bool,
//}
//
//impl TemporaryFile {
//    fn new(path: &str) -> Self {
//        //-> io::Result<Self> {
//        let _ = fs::remove_file(path);
//
//        return TemporaryFile {
//            path: String::from(path),
//            should_cleanup: false, // Set to false until explicitly created
//        };
//    }
//
//    fn create(&mut self) {
//        // Create the file
//        match File::create(&self.path) {
//            Ok(_) => {
//                // Set should_cleanup to true if creation was successful
//                self.should_cleanup = true;
//            }
//            Err(err) => {
//                panic!("Failed to create file '{}': {}", self.path, err);
//            }
//        }
//    }
//
//    fn path(&self) -> &str {
//        &self.path
//    }
//
//    fn delete(&self) {
//        // Unconditionally delete the file
//        if let Err(err) = fs::remove_file(&self.path) {
//            panic!("Failed to delete file '{}': {}", self.path, err);
//        }
//    }
//}
//
//impl Drop for TemporaryFile {
//    fn drop(&mut self) {
//        // Perform cleanup actions if needed
//        if self.should_cleanup {
//            // Panic if failed to delete the file
//            self.delete();
//        }
//    }
//}
//
////    let mut temp_file = TemporaryFile::new(path);
////    temp_file.create();
////    temp_file.delete();

//// Create a static instance of HookTracker using lazy_static
//lazy_static! { //FIXME: external crate should not be needed, find another way.
//    static ref HOOK_TRACKER: HookTracker = HookTracker {
//        //list: Arc::new(Mutex::new(Vec::new())),
//        list: Mutex::new(Vec::new()),
//    };
//}
//use std::sync::OnceLock;
//static HOOK_TRACKER: OnceLock<HookTracker> = OnceLock::new();
//use std::sync::LazyLock;
static HOOK_TRACKER: std::sync::LazyLock<HookTracker> = std::sync::LazyLock::new(|| {
    //HookTracker { list:Mutex::new(Vec::new()) }
    HookTracker::init()
});

//use std::sync::Mutex;
//use lazy_static::lazy_static;

// Define a struct to hold your hook data
struct HookTracker {
    //list: Arc<Mutex<Vec<&'static str>>>, //nopTODO: is Arc really needed tho?! other than futureproofing the code
    list: std::sync::Mutex<Vec<&'static str>>,
} // struct

impl HookTracker {
    //fn get() -> HookTracker { //Mutex<Vec<&'static str>> {
    //    HOOK_TRACKER.get_or_init(|| HookTracker::init())
    //}
    #[inline]
    fn init() -> HookTracker {
        //Mutex<Vec<&'static str>>{
        HookTracker {
            list: std::sync::Mutex::new(Vec::new()),
        }
    }
    // Function to register that a hook has started executing
    fn started_executing(&self, func_name: &'static str)
    //    fn started_executing<F>(&self, _func: F)
    //    where
    //        F: Fn(),
    {
        // Lock the list to ensure exclusive access
        let mut guard = self
            .list
            .lock()
            .expect("Unexpected concurrent execution attempted");

        // Add the name of the function to the list
        guard.push(func_name);

        // Print the name of the function
        println!("Executing {}", func_name);
    }

    //// Function to print the names of executed functions
    //fn print_executed_hooks(&self) {
    //    // Lock the list to ensure exclusive access
    //    let guard = self.list.lock().expect("Failed to acquire lock.");
    //    println!("Here's what executed so far:");
    //    // Print the names of executed functions
    //    for (index, func_name) in guard.iter().enumerate() {
    //        println!("Function {}: {}", index + 1, func_name);
    //    }
    //}
    #[allow(dead_code)] //it's used in #[test] at least!
    fn get_executed_hooks(&self) -> Vec<&str> {
        // Lock the list to ensure exclusive access
        let guard = self.list.lock().expect("Failed to acquire lock.");

        // Create a Vec<String> to hold the executed hook names
        let mut executed_hooks = Vec::new();

        // Iterate over the guard and collect the hook names into the executed_hooks Vec
        for func_name in guard.iter() {
            executed_hooks.push(*func_name); //XXX: well...
        }

        // Return the Vec<String> containing the executed hook names
        executed_hooks
    } //fn
} //impl
