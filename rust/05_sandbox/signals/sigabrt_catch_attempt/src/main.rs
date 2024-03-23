FAIL; thx chatgpt 3.5 
extern crate nix;

use nix::libc;
use crate::libc::sigaction;
use std::sync::atomic::{AtomicBool, Ordering};

// Atomic boolean to indicate whether the signal has been caught
static SIGNAL_CAUGHT: AtomicBool = AtomicBool::new(false);

fn main() {
    // Set up a signal handler for SIGABRT
    setup_signal_handler();

    // Simulate an abort
    unsafe {
        libc::abort();
    }

    // Check if the signal has been caught
    if SIGNAL_CAUGHT.load(Ordering::Relaxed) {
        println!("SIGABRT signal caught!");
    } else {
        println!("SIGABRT signal not caught!");
    }
}

// Signal handler function
extern "C" fn signal_handler(_: libc::c_int) {
    // Set the atomic boolean to indicate that the signal has been caught
    SIGNAL_CAUGHT.store(true, Ordering::Relaxed);
}

// Function to set up signal handler for SIGABRT
fn setup_signal_handler() {
    // Set up signal handler using nix
    let sig_action = nix::sys::signal::SigAction::new(
        nix::sys::signal::SigHandler::Handler(signal_handler),
        nix::sys::signal::SaFlags::empty(),
        nix::sys::signal::SigSet::empty(),
    );

    // Register the signal handler for SIGABRT
    unsafe {
        sigaction(libc::SIGABRT, &sig_action).expect("Failed to set up signal handler");
    }
}

