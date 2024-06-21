// initially made with chatgpt 3.5
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    //XXX: ah Arc is heap allocated!
    // Shared state for managing timer reset and detection
    let state = Arc::new((Mutex::new(false), Condvar::new()));
    let state_clone = Arc::clone(&state);

    // Shared flag to signal the timer thread to exit
    let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_clone = Arc::clone(&exit_flag);

    // Timer thread logic
    thread::spawn(move || {
        let (lock, cvar) = &*state_clone;
        let timeout = Duration::from_secs(2); // Adjust timeout duration as needed

        loop {
            let mut guard = lock.lock().unwrap();
            let expired: bool = *guard;//fkin hate derefs now.

            // Wait for the timeout duration or a notification
            let result = cvar.wait_timeout(guard, timeout).unwrap();
            //let result = cvar
                //.wait_timeout_while(guard, timeout, |expired| !*expired)
                //.unwrap();
            guard = result.0;
            let expired = result.1;

            // Check if timer expired without reset
            if expired.timed_out() {
                println!("Timer expired! Main thread did not reset.");
                // Perform action to panic or handle deadlock in main thread
                // For demonstration, we just panic here
                extern "C" {
                    pub fn abort() -> !;
                }
                unsafe {
                    abort();
                }
                //panic!("Main thread deadlocked!");
            }
            // Reset the timer check based on external conditions or exit flag
            if exit_flag_clone.load(Ordering::SeqCst) {
                println!("Timer thread exiting.");
                break;
            }
        }
    });

    // Simulate main thread doing work and periodically resetting the timer
    for i in 1..10 {
        println!("Main thread doing work {}", i);
        thread::sleep(Duration::from_millis(100));

        // Reset the timer in main thread after every 3 iterations
        if i % 3 == 0 {
            let (lock, cvar) = &*state;
            let mut expired = lock.lock().unwrap();
            *expired = false;
            cvar.notify_all();
            // Drop the lock to allow timer thread to acquire it
            drop(expired);
            //XXX: uncomment this to cause deadlock and see what happens!
            //loop {
            //    std::thread::sleep(std::time::Duration::from_secs(1));
            //}
        }
    }

    println!("Main thread finished work without deadlock.");
    // Signal the timer thread to exit
    exit_flag.store(true, Ordering::SeqCst);

    // Notify the condition variable to wake up the timer thread if it's waiting
    {
        let (_, cvar) = &*state;
        cvar.notify_all();
    }

    // Wait for a short time to ensure the timer thread exits
    thread::sleep(Duration::from_secs(1));
}
