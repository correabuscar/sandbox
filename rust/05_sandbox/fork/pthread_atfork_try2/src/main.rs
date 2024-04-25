#![feature(lazy_cell)]
//use std::sync::{Arc, Mutex};
//use std::thread;
//use std::process;

//// Define boolean flags to track fork events
//#[derive(Default)]
//struct ForkFlags {
//    forked: bool,
//}

#[cfg(not(any(unix, target_os = "fuchsia", target_os = "vxworks")))]
fn main() {
    // a main() for other OS-es which aren't supported!
    //compile_error!("hi");
    panic!("There's no fork() on your OS: {}", std::env::consts::OS);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
fn main() {
    //    // Shared state for boolean flags
    //    let fork_flags = Arc::new(Mutex::new(ForkFlags::default()));

    //#[cfg(not(unix))]
    //compile_error!("There's no fork() on your OS!"); //XXX: can't format it! so can't show own OS
    //compile_error!(concat!("There's no fork() on your OS ", env!("CARGO_CFG_TARGET_FAMILY")));
    //XXX: can't format it in compile_error!() and there's no env.var. at build time that's like
    //CARGO_CFG_TARGET_FAMILY(eg. "unix") which is only available in build.rs
    //see: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    //{ //block to not have to repeat the 'cfg' but this runs drop() sooner than main() exit!ie. at
    //end of this block! doneFIXME: fixed by placing cfg on main()
    eprintln!(
        "Welcome. There's fork() and pthread_atfork() on this OS: {}",
        std::env::consts::OS
    );

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
        //vfork got deprecated&removed: https://github.com/rust-lang/libc/pull/3624
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
            let has_exit_code = wait_for_child_or_panic(child_pid);
            println!(
                "Child process with PID {} exited with exit code: {}",
                child_pid, has_exit_code
            );
        }
    }; //match
} //main

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
fn wait_for_child_or_panic(child_pid: libc::pid_t) -> libc::c_int {
    const TIMEOUT_SECS: u64 = 5;
    let start_time = std::time::Instant::now();
    loop {
        //TODO: uncertain why the loop is needed, apparently child receiving a signal and not
        //exiting can make waitpid still exit somehow? uncertain! added 5 sec timeout anyway,
        //safer than forever loop.
        let elapsed_time = start_time.elapsed().as_secs();
        if elapsed_time >= TIMEOUT_SECS {
            // Timeout reached
            panic!(
                "Timeout {} seconds while waiting for child process with pid={} to exit.",
                TIMEOUT_SECS, child_pid
            );
            //break;
            //return None;
        }
        //The waitpid() system call suspends execution of the calling thread until a child specified by pid argument has changed  state.
        let mut status: libc::c_int = 0;
        let result = unsafe { libc::waitpid(child_pid, &mut status, 0) };
        if result == -1 {
            panic!("Error waiting for child process, waitpid returned -1");
        }
        if result == child_pid {
            if libc::WIFEXITED(status) {
                let status = libc::WEXITSTATUS(status);
                println!("Child process exited with status: {}", status);
                return status; //Some(status);
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

//use std::sync::atomic::AtomicBool;
//use std::sync::atomic::Ordering;
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
static PREPARED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
static PREPARED2: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
static PARENT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
static PARENT2: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
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
    PREPARED.store(true, std::sync::atomic::Ordering::SeqCst);
    eprintln!("!! prepare pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent() {
    HOOK_TRACKER.started_executing("parent");
    PARENT.store(true, std::sync::atomic::Ordering::SeqCst);
    // You can perform any necessary actions after fork() in the parent process
    eprintln!("!! parent pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child() {
    HOOK_TRACKER.started_executing("child");
    // You can perform any necessary actions after fork() in the child process
    //std::thread::sleep(std::time::Duration::from_millis(DELAY_MILLIS));
    eprintln!("!! child pid={}", std::process::id());
}
//}
//} //#cfg

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare2() {
    HOOK_TRACKER.started_executing("prepare2");
    PREPARED2.store(true, std::sync::atomic::Ordering::SeqCst);
    // You can perform any necessary actions before fork() in the parent process
    eprintln!("!! prepare2 pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent2() {
    HOOK_TRACKER.started_executing("parent2");
    // You can perform any necessary actions after fork() in the parent process
    PARENT2.store(true, std::sync::atomic::Ordering::SeqCst);
    eprintln!("!! parent2 pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child2() {
    HOOK_TRACKER.started_executing("child2");
    // You can perform any necessary actions after fork() in the child process
    //std::thread::sleep(std::time::Duration::from_secs(1));
    eprintln!("!! child2 pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
#[test]
fn test_that_pthread_atfork_works_as_expected() {
    println!(); //a new line

    //doneTODO: dedup ^
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

    //#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
    match unsafe {
        libc::fork()
        /* So, to answer your question, after fork() exits, it is indeed guaranteed that the prepare handlers and parent handlers set by a previous pthread_atfork() have completed execution before control is returned to the parent process from fork(). There's no concurrent execution of the handlers during the fork() operation. The fork() operation itself is blocking, ensuring that the atfork handlers are executed in sequence before the child process is created.
        - chatgpt 3.5 */
    } {
        // match
        -1 => panic!("Fork failed"),
        0 => {
            // Child process
            println!("Child process");
            let hooks_order_seen_in_child = HOOK_TRACKER.get_executed_hooks();
            println!(
                "inchild:{:?}",
                hooks_order_seen_in_child //HookTracker::get().lock().get_executed_hooks()
                                          //HOOK_TRACKER.get_or_init(|| HookTracker::init()).get_executed_hooks()
            );
            const EXPECTED_ORDER: [&str; 4] = ["prepare2", "prepare", "child", "child2"];
            if hooks_order_seen_in_child == EXPECTED_ORDER {
                //success
                std::process::exit(200);
            } else {
                //failed order, unexpected.
                println!("!! Fork process reports unexpected order or missing execution of hooks: (actual) '{:?}' != '{:?}' (expected)", hooks_order_seen_in_child, EXPECTED_ORDER);
                std::process::exit(201);
            }
            //std::process::exit(0); // Example of child process exiting
        }
        child_pid => {
            // Parent process
            println!(
                "This is the Parent process, but we know forked process aka child PID: {}",
                child_pid
            );
            // Wait for the specific child process to exit, the easy/safe way.
            let exit_code = wait_for_child_or_panic(child_pid);
            println!(
                "Parent sees the Child process with PID '{}' exited with exit code: '{}'",
                child_pid, exit_code
            );
            //let exit_code = has_exit_code.expect("forked process didn't exit successfully");
            assert_eq!(200, exit_code, "Fork process didn't execute one or more child hooks, or if it did, they weren't in expected order!");
            //XXX: 200 means child's seen this (correct)hook execution order: ["prepare2", "prepare", "child", "child2"]
        }
    }; //match

    //panic!("uhm");
    //std::thread::sleep(std::time::Duration::from_secs(2));
    assert_eq!(PREPARED.load(std::sync::atomic::Ordering::SeqCst), true);
    assert_eq!(PREPARED2.load(std::sync::atomic::Ordering::SeqCst), true);
    assert_eq!(PARENT.load(std::sync::atomic::Ordering::SeqCst), true);
    assert_eq!(PARENT2.load(std::sync::atomic::Ordering::SeqCst), true);
    //must wait for fork to finish, the hard way:
    //std::thread::sleep(std::time::Duration::from_millis(DELAY_MILLIS+1000)); //add an extra second
    //to wtw the delay was
    //XXX: can't use statics for this, as it's a different (forked) process:
    //assert_eq!(CHILD.load(std::sync::atomic::Ordering::SeqCst), true);
    //assert_eq!(CHILD2.load(std::sync::atomic::Ordering::SeqCst), true);

    //doneFIXME: this test doesn't test order of execution of the handlers
    //doneTODO: get rid of external crate for lazy_static!() macro.
    //doneTODO: since we keep a list of executed hooks, we know which ones (if any) didn't execute,
    //don't thus need the creation of files in /tmp/ by the forked process too! this was left
    //undone in previous project aka ../pthread_atfork_try1/
    //XXX: the second part of this list is already verified in child aka forked process.
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
//lazy_static! { //doneFIXME: external crate should not be needed, find another way.
//    static ref HOOK_TRACKER: HookTracker = HookTracker {
//        //list: Arc::new(Mutex::new(Vec::new())),
//        list: Mutex::new(Vec::new()),
//    };
//}
//use std::sync::OnceLock;
//static HOOK_TRACKER: OnceLock<HookTracker> = OnceLock::new();
//use std::sync::LazyLock;
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
static HOOK_TRACKER: std::sync::LazyLock<HookTracker> = std::sync::LazyLock::new(|| {
    //HookTracker { list:Mutex::new(Vec::new()) }
    HookTracker::init()
});

//use std::sync::Mutex;
//use lazy_static::lazy_static;

// Define a struct to hold your hook data
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
struct HookTracker {
    //list: Arc<Mutex<Vec<&'static str>>>, //nopTODO: is Arc really needed tho?! other than futureproofing the code
    list: std::sync::Mutex<Vec<&'static str>>,
    //TODO: do I need RwLock instead of Mutex here?! think.
} // struct

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
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
        // TODO: figure out what happens when panic happens during held lock, ie. poisoning
        // Lock the list to ensure exclusive access
        if let Ok(mut guard) = self.list.lock() {
            //use if let to keep lock for minimal block
            //.expect("Unexpected concurrent execution attempted") {

            // Add the name of the function to the list
            guard.push(func_name);//TODO: what if this panics? lock poisoning?
        }//lock released here.
        else {
            // This function might panic when called if the lock is already held by the current thread.
            panic!("Impossible recursive execution attempted. Couldn't acquire lock!");
        }

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
