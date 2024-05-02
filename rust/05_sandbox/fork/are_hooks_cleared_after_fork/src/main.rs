#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
fn forky(refork:bool) {
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
            if refork {
                forky(false);
            }
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
}

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

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
fn main() {
    println!("Hello, world!");
    // Register fork handlers
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(Some(prepare), Some(parent), Some(child));
        if result != 0 {
            // Error handling: Handle the case where pthread_atfork fails
            panic!("pthread_atfork (1) failed with error code: {}", result);
        }
    }
    forky(false);
    forky(true);
    forky(true);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare() {
    eprintln!("!! prepare pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent() {
    eprintln!("!! parent pid={}", std::process::id());
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child() {
    eprintln!("!! child pid={}", std::process::id());
}
