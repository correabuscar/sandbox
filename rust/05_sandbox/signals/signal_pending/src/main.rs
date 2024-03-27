extern crate libc;

//made with the help of chatgpt 3.5


//#[cfg(not(debug_assertions))]

use std::ptr;

// Signal handler function
extern "C" fn signal_handler(sig: libc::c_int) {
    //This won't run, because we mask it, but it's needed or else the default handler will say:
    //"Real-time signal 49" and exit with code 177 aka 128+49
    println!("Signal {} received", sig);
    panic!("unreachable");
}

fn is_signal_pending(signal_to_check: i32) -> Result<bool, &'static str> {
    // Create a set to store pending signals
    let mut pending_set: libc::sigset_t = unsafe { std::mem::zeroed() };
    // "While sigemptyset initializes the signal set structure to an empty state, relying on
    // uninitialized memory before calling sigemptyset can be risky and may lead to unexpected
    // behavior or bugs in your program." src: chatgpt 3.5
    // but in my case it's libc::sigpending that empties it internally.

    // Check if the signal is pending with a timeout
    // Call sigpending() to get the set of pending signals
    if unsafe { libc::sigpending(&mut pending_set) } != -1 {
        // Check if a specific signal is pending
        //let signal_to_check = halfway_signal;
        let is_pending = unsafe { libc::sigismember(&pending_set, signal_to_check) } == 1;
        return Ok(is_pending);
    } else {
        return Err("Error checking pending signals.");
    }
}

fn is_signal_pending_with_timeout(who:&str, signal_to_check: i32, timeout_msec:u64) {
    // Create a set to store pending signals
    let mut pending_set: libc::sigset_t = unsafe { std::mem::zeroed() };
    // "While sigemptyset initializes the signal set structure to an empty state, relying on
    // uninitialized memory before calling sigemptyset can be risky and may lead to unexpected
    // behavior or bugs in your program." src: chatgpt 3.5
    // but in my case it's libc::sigpending that empties it internally.

    // Start time for timeout calculation
    let start_time = std::time::Instant::now();
    // Changes to your system clock, whether manual or through NTP updates, won't affect the accuracy of Instant::now() and .elapsed() in Rust, as they rely on the system's monotonic clock, which isn't subject to adjustments or updates from the system clock.
    const RECHECK_SLEEP_MS:u64=100;
    let recheck_sleep_ms=std::time::Duration::from_millis(RECHECK_SLEEP_MS);
    // Check if the signal is pending with a timeout
    let timeout_dur=std::time::Duration::from_secs(timeout_msec);
    while start_time.elapsed() < timeout_dur {
        // Call sigpending() to get the set of pending signals
        if unsafe { libc::sigpending(&mut pending_set) } != -1 {
            // Check if a specific signal is pending
            //let signal_to_check = halfway_signal;
            let is_pending = unsafe { libc::sigismember(&pending_set, signal_to_check) } == 1;

            match is_signal_pending(signal_to_check) {
                Ok(ret) => {
                    if ret {
                        println!("{who} Signal {} is pending.", signal_to_check);
                        break;
                    } else {
                        println!("{who} Signal {} is not pending.", signal_to_check);
                    }
                } // ok
                Err(e) => {
                    println!("{who} Error checking pending signals, '{e}'");
                }//err
            }//match
        }
        // Sleep briefly before rechecking
        std::thread::sleep(recheck_sleep_ms);
    }
}

#[test]
fn test_assertions_are_enabled_in_cargo_toml_for_test_profile() {
    assert!(cfg!(debug_assertions));
}

fn main() {
    println!("Hello, world!");
    if cfg!(debug_assertions) {
        println!("Debugging enabled");
    } else {
        panic!("Debugging disabled");
    }
    assert!(cfg!(debug_assertions));

    //TODO: use static globals wrapped in std::sync::OnceLock ?
    let rtmin =  libc::SIGRTMIN();
    let rtmax =  libc::SIGRTMAX();
    assert!(rtmin <= rtmax); // this gets removed on 'cargo run --release' unless... Cargo.toml change ... 

    let halfway_signal = rtmin + ((rtmax - rtmin) / 2);
    println!("Halfway Signal Number: {}", halfway_signal);

    let signal_number=halfway_signal;
    unsafe {
        let mut act: libc::sigaction = std::mem::zeroed();
        act.sa_sigaction = signal_handler as usize;
        act.sa_flags |= libc::SA_SIGINFO;
        libc::sigemptyset(&mut act.sa_mask as *mut libc::sigset_t);
        libc::sigaction(signal_number, &act, ptr::null_mut());
    }

    // Block signal 49
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut mask);
        libc::sigaddset(&mut mask, signal_number);
        libc::sigprocmask(libc::SIG_BLOCK, &mask, ptr::null_mut());
    }


    let signal_to_send = halfway_signal;
    //on Gentoo kernel 6.7.9-gentoo-x86_64, I'm not allowed to set it higher than 4,194,304 ie. cat /proc/sys/kernel/pid_max
    let pid=std::process::id() as i32; //so this i32 should be enough to hold any PID ranges
    // Send the signal to yourself
    // "Return from kill: As mentioned, the libc::kill function returns immediately after sending the signal. It does not wait for the signal to be received or processed by the target process.
    // Signal Queuing: Once the signal is sent, it is typically queued for delivery to the target process. However, the actual delivery time depends on various factors, including the scheduler's decisions and the current state of the receiving process. The operating system handles signal delivery and scheduling.
    // Signal Handling: If the signal is blocked or ignored by the receiving process, it will remain pending until it is unblocked or until the process changes its signal handling behavior. This could result in the behavior you described, where the signal appears to be pending but is not being handled by the process.
    // Signal Delivery Time: There is no specific guarantee on how quickly the signal will be delivered to the target process after it is sent. The operating system's scheduler determines when the receiving process will handle the signal, based on its scheduling policies and the state of the system."
    // FIXME: soo, some kind of delay or timeout-based for loop would be needed to ensure the
    // signal was indeed sent and received, before moving on, OR when checking for it.
    if unsafe { libc::kill(pid, signal_to_send) } == 0 {
        // "the libc::kill function returns immediately after sending the signal, as it's a non-blocking call."
        // "it will indeed remain pending until the receiving process decides to handle it or unblock it."
        println!("Signal {} sent to self", signal_to_send);
    } else {
        println!("Error sending signal to self");
    }

    let mut who="main";
    const TIMEOUT_SECS:u64=1;
    //wait 1 sec until 'kill' is sure to have sent it.
    is_signal_pending_with_timeout(who, halfway_signal, TIMEOUT_SECS);
    let r=is_signal_pending(halfway_signal);//non-blocking
    assert!(r.is_ok());

    let ret=unsafe {
        libc::fork()
    };
    if ret == -1 {
        panic!("failed to fork");
    }else if ret != 0 {
        //sleep in parent
        //not needed but hey
        std::thread::sleep(std::time::Duration::from_secs(1));
    } else {
        who="fork";
        //XXX: "The child’s set of pending signals is initially empty (sigpending(2))." src: man 2 fork
    }
    let r=is_signal_pending(halfway_signal).expect("should not have errored");
    if r == true {
        assert_eq!(who,"main");
        println!("{who} Signal is pending.");
    } else {
        assert_eq!(who,"fork");
        println!("{who} Signal is NOT pending.");
    }


    println!("{who} is done!");
}
