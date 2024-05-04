#![feature(rt)]
use std::rt::ForkState;

#[cfg(all(
        any(unix, target_os = "fuchsia", target_os = "vxworks"),
        target_pointer_width = "64",
        ))]
fn fork_syscall() -> i32 {
    let result: i64;
    unsafe {
        std::arch::asm!(
            "syscall",
            inout("rax") 57_i64 => result, // 57 is the syscall number for fork on x86_64
            lateout("rcx") _, // Preserve rcx register
            lateout("r11") _, // Preserve r11 register
            options(nomem, preserves_flags),
        );
    }
    result as i32
}

#[cfg(not(any(unix, target_os = "fuchsia", target_os = "vxworks")))]
#[no_mangle]
pub extern "C" fn my_rust_function() {
    // a main() for other OS-es which aren't supported!
    //compile_error!("hi");
    panic!("There's no fork() on your OS: {}", std::env::consts::OS);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
#[no_mangle]
pub extern "C" fn my_rust_function() {
    println!("Hello from Rust!");

    let fork_state=std::rt::is_this_forked_process(false);
    println!("forkstate={:?}", fork_state);
    assert_eq!(std::rt::ForkState::CanNotBeKnown, fork_state);

    let fork_state=std::rt::is_this_forked_process(false);
    println!("forkstate={:?}", fork_state);
    assert_eq!(std::rt::ForkState::CanNotBeKnown, fork_state);

    let fork_state=std::rt::is_this_forked_process(true);
    println!("forkstate={:?}", fork_state);
    assert_eq!(std::rt::ForkState::CanNotBeKnown, fork_state);

    //add our hooks before!
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(Some(prepare), Some(parent), Some(child));
        assert_eq!(0, result,"hooks failed to be added");
        let result: libc::c_int = libc::pthread_atfork(Some(prepare2), Some(parent2), Some(child2));
        assert_eq!(0, result,"hooks failed to be added");
    }

    let ret=std::rt::init_fork_detection();
    assert!(ret,"unsuccessful init1!");
    let ret=std::rt::init_fork_detection();
    assert!(ret,"unsuccessful init2!");

    //add our hooks after!
    unsafe {
        let result: libc::c_int = libc::pthread_atfork(Some(prepare_aft), Some(parent_aft), Some(child_aft));
        assert_eq!(0, result,"hooks failed to be added");
        let result: libc::c_int = libc::pthread_atfork(Some(prepare_aft2), Some(parent_aft2), Some(child_aft2));
        assert_eq!(0, result,"hooks failed to be added");
    }


    let fork_state=std::rt::is_this_forked_process(false);
    println!("forkstate={:?}", fork_state);
    assert_eq!(std::rt::ForkState::NotForked, fork_state);

    let fork_state=std::rt::is_this_forked_process(true);
    println!("forkstate={:?}", fork_state);
    assert_eq!(std::rt::ForkState::NotForked, fork_state);

    for i in 1..=2 {
        //do this 2 times:
        match unsafe {
            libc::fork() //WORKS!
        } {
            -1 => panic!("Fork failed"),
            0 => {
                // Child process
                println!("This is child process");
                let fork_state=std::rt::is_this_forked_process(true);
                println!("in child forkstate={:?}", fork_state);
                assert_eq!(std::rt::ForkState::Forked, fork_state);
                // Do child process work...
                std::process::exit(0); // Example of child process exiting
            }
            child_pid => {
                // Parent process
                println!("This is parent process noting that the child PID is: {}", child_pid);
                let fork_state=std::rt::is_this_forked_process(true);
                println!("in parent forkstate={:?}", fork_state);
                assert_eq!(std::rt::ForkState::NotForked, fork_state);
                // Wait for the specific child process to exit, the easy/safe way.
                let has_exit_code = wait_for_child_or_panic(child_pid);
                println!(
                    "This is parent noting that child process with PID {} exited with exit code: {}",
                    child_pid, has_exit_code
                    );
                let fork_state=std::rt::is_this_forked_process(true);
                println!("still in parent forkstate={:?}", fork_state);
                assert_eq!(std::rt::ForkState::NotForked, fork_state);
            }
        }//match
    }//for

    //TODO:
/*    match unsafe {
        fork_syscall()
        //libc::fork() //WORKS!
    } {
        -1 => panic!("Fork failed"),
        0 => {
            // Child process
            println!("This is child process");
            let fork_state=std::rt::is_this_forked_process(true);
            println!("in child forkstate={:?}", fork_state);
            assert_eq!(std::rt::ForkState::Forked, fork_state);
            // Do child process work...
            std::process::exit(0); // Example of child process exiting
        }
        child_pid => {
            // Parent process
            println!("This is parent process noting that the child PID is: {}", child_pid);
            let fork_state=std::rt::is_this_forked_process(true);
            println!("in parent forkstate={:?}", fork_state);
            assert_eq!(std::rt::ForkState::NotForked, fork_state);
            // Wait for the specific child process to exit, the easy/safe way.
            let has_exit_code = wait_for_child_or_panic(child_pid);
            println!(
                "This is parent noting that child process with PID {} exited with exit code: {}",
                child_pid, has_exit_code
            );
            let fork_state=std::rt::is_this_forked_process(true);
            println!("still in parent forkstate={:?}", fork_state);
            assert_eq!(std::rt::ForkState::NotForked, fork_state);
        }
    }//match
    */
}//my func


#[inline(always)]
fn is_this_forked_process_no_getpid() -> std::rt::ForkState {
    std::rt::is_this_forked_process(/*trust_pid==*/false)
}

#[inline(always)]
fn is_this_forked_process_yes_getpid() -> std::rt::ForkState {
    std::rt::is_this_forked_process(/*trust_pid==*/true)
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare() {
    let fork_state=is_this_forked_process_no_getpid();
    eprintln!("!! prepare pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::InProgressInsideTheForkHooks);//XXX
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent() {
    let fork_state=is_this_forked_process_no_getpid();
    eprintln!("!! parent pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::InProgressInsideTheForkHooks);//XXX
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child() {
    let fork_state=is_this_forked_process_no_getpid();
    eprintln!("!! child pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::InProgressInsideTheForkHooks);//XXX
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare_aft() {
    let fork_state=is_this_forked_process_no_getpid();
    eprintln!("!! prepare_aft pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::NotForked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent_aft() {
    let fork_state=is_this_forked_process_no_getpid();
    eprintln!("!! parent_aft pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::NotForked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child_aft() {
    let fork_state=is_this_forked_process_no_getpid();
    eprintln!("!! child_aft pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::Forked);
}

//oh my, the duppenning!
#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare2() {
    let fork_state=is_this_forked_process_yes_getpid();
    eprintln!("!! prepare2 pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::NotForked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent2() {
    let fork_state=is_this_forked_process_yes_getpid();
    eprintln!("!! parent2 pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::NotForked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child2() {
    let fork_state=is_this_forked_process_yes_getpid();
    eprintln!("!! child2 pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::Forked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn prepare_aft2() {
    let fork_state=is_this_forked_process_yes_getpid();
    eprintln!("!! prepare_aft2 pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::NotForked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn parent_aft2() {
    let fork_state=is_this_forked_process_yes_getpid();
    eprintln!("!! parent_aft2 pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::NotForked);
}

#[cfg(any(unix, target_os = "fuchsia", target_os = "vxworks"))]
unsafe extern "C" fn child_aft2() {
    let fork_state=is_this_forked_process_yes_getpid();
    eprintln!("!! child_aft2 pid={} {:?}", std::process::id(), fork_state);
    assert_eq!(fork_state,ForkState::Forked);
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
                println!("This is parent, noting that child process exited with status: {}", status);
                return status; //Some(status);
            }
        }

        // Sleep for a short duration before checking again
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
