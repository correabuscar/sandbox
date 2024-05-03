use std::ffi::CStr;
use std::os::raw::c_char;
use std::thread;

fn foo() {
    // Call pthread_atfork
    let result = unsafe { libc::pthread_atfork(None, None, None) };
    //XXX: I can't believe it still adds them if they're None :) https://github.com/bminor/glibc/blob/4bbca1a44691a6e9adcee5c6798a707b626bc331/posix/register-atfork.c#L47

    // Check if an error occurred
    if result != 0 {
        // Get the value of errno
        let errno_value = unsafe { *libc::__errno_location() };
        assert_eq!(result, errno_value,"these were supposed to be same, or 'result' should be ENOMEM");

        // Convert errno value to a human-readable error message
        let c_err_msg: *const c_char = unsafe { libc::strerror(errno_value) };
        let err_msg = unsafe { CStr::from_ptr(c_err_msg) };
        let err_msg_str = err_msg.to_str().unwrap_or("Unknown error");

        // Display an error message
        panic!(
            "Error: pthread_atfork returned non-zero value. Errno: {}. Message: {}",
            errno_value, err_msg_str
        );
    }
//    foo();
}
fn main() {
    // Create a thread with a small stack size
    let handle = thread::Builder::new()
        .spawn(|| {
            // Loop to exhaust memory
            loop {
                foo();
            }
        })
        .expect("Failed to create thread");

    // Wait for the thread to finish
    let _ = handle.join();
}

