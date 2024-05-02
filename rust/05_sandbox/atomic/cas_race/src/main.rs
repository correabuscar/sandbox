/// works as expected, no races here!

use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    // Shared flag to indicate if both threads succeeded
    // ignore the name of this var:
    let both_succeeded = Arc::new(AtomicBool::new(false));

    let exit_all=Arc::new(AtomicBool::new(false));
    const SUCCESS_ORDERING:Ordering=Ordering::Acquire;
    //const SUCCESS_ORDERING:Ordering=Ordering::Relaxed;
    const FAILURE_ORDERING:Ordering=Ordering::Relaxed;

    // Create 12 threads
    let mut threads = vec![];
    for i in 1..=12 {
        let both_succeeded_clone = Arc::clone(&both_succeeded);
        let exit_all_clone=Arc::clone(&exit_all);

        // Define a closure to be used by each thread
        let thread_closure = move || {
            loop {
                // Perform the CAS operation
                for _j in 1..=5*i {
                    let result = both_succeeded_clone.compare_exchange(false, true, SUCCESS_ORDERING, FAILURE_ORDERING);
                    match result {
                        Ok(false) | Err(true) => {
                            //got to next CAS
                        },
                        Ok(true) => {
                            println!("Thread {}: Ok(true)", i);
                            exit_all_clone.store(true, Ordering::Relaxed);
                            break;
                        },
                        Err(false) => {
                            println!("Thread {}: Err(false)", i);
                            exit_all_clone.store(true, Ordering::Relaxed);
                            break;
                        },
                    }
                    //thread::sleep_ms(1);
                    let result = both_succeeded_clone.compare_exchange(true, false, SUCCESS_ORDERING, FAILURE_ORDERING);
                    match result {
                        Ok(true) | Err(false) => {
                            continue;
                        },
                        Ok(false) => {
                            println!("Thread {}: Ok(false)", i);
                            exit_all_clone.store(true, Ordering::Relaxed);
                            break;
                        },
                        Err(true) => {
                            println!("Thread {}: Err(true)", i);
                            exit_all_clone.store(true, Ordering::Relaxed);
                            break;
                        },
                    }
                } //for 5 times
                if exit_all_clone.load(Ordering::Relaxed) {
                    break;
                }
            } //infinite loop
        };

        let thread_name = format!("Thread {}", i);
        let thread = thread::Builder::new()
            .name(thread_name.clone())
            .spawn(thread_closure)
            .expect("Failed to create thread");
        threads.push(thread);
    }

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    // Check if both threads succeeded
    if both_succeeded.load(Ordering::SeqCst) {
        println!("Both threads succeeded!");
    } else {
        println!("At least one thread failed.");
    }
}

