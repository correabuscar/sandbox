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

fn is_signal_pending(who:&str, signal_to_check: i32) {
    // Create a set to store pending signals
    let mut pending_set: libc::sigset_t = unsafe { std::mem::zeroed() };

    // Call sigpending() to get the set of pending signals
    if unsafe { libc::sigpending(&mut pending_set) } != -1 {
        // Check if a specific signal is pending
        //let signal_to_check = halfway_signal;
        let is_pending = unsafe { libc::sigismember(&pending_set, signal_to_check) } == 1;

        if is_pending {
            println!("{who} Signal {} is pending.", signal_to_check);
        } else {
            println!("{who} Signal {} is not pending.", signal_to_check);
        }
    } else {
        println!("{who} Error checking pending signals.");
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
    if unsafe { libc::kill(pid, signal_to_send) } == 0 {
        println!("Signal {} sent to self", signal_to_send);
    } else {
        println!("Error sending signal to self");
    }

    let mut who="main";
    is_signal_pending(who, halfway_signal);

    let ret=unsafe {
        libc::fork()
    };
    if ret == -1 {
        panic!("failed to fork");
    }else if ret != 0 {
        //sleep in parent
        std::thread::sleep(std::time::Duration::from_secs(1));
    } else {
        who="fork";
        //XXX: "The child’s set of pending signals is initially empty (sigpending(2))." src: man 2 fork
    }
    is_signal_pending(who, halfway_signal);

    println!("{who} is done!");
}
