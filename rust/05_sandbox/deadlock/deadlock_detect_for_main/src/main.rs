// initially made with chatgpt 3.5
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Shared state for managing timer reset and detection
    let state = Arc::new((Mutex::new(false), Condvar::new()));
    let state_clone = Arc::clone(&state);

    // Timer thread logic
    thread::spawn(move || {
        let (lock, cvar) = &*state_clone;
        let timeout = Duration::from_secs(2); // Adjust timeout duration as needed

        loop {
            let mut guard = lock.lock().unwrap();
            let expired: bool = *guard;

            // Wait for the timeout duration or a notification
            let result = cvar
                .wait_timeout_while(guard, timeout, |expired| !*expired)
                .unwrap();
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
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }

    println!("Main thread finished work without deadlock.");
}
