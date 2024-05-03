use std::sync::{Mutex, MutexGuard};
use std::thread;

// Define your function
fn my_function() {
    // Initialize a static mutex to guard the critical section and a boolean flag
    static MUTEX: Mutex<bool> = Mutex::new(false);

    // Acquire the lock before accessing/modifying the boolean flag
    let mut flag: MutexGuard<bool> = match MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            println!("Ignoring poisoning!");
            poisoned.into_inner()
        }
    };

    // Check if the function has already been executed
    if *flag {
        println!("Function has already been executed before, returning early.");
        return;
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=100);

    println!("Function is now running");

    // Simulate work
    thread::sleep(std::time::Duration::from_secs(2));
    if random_number % 2 == 0 {
        panic!("teehee {}",random_number);
    }

    println!("Function completed execution");
    // Set the flag to true to indicate that the function has completed init
    *flag = true;
}

fn main() {
    // Spawn multiple threads to call the function concurrently
    let handles: Vec<_> = (0..5)
        .map(|_| {
            thread::spawn(|| {
                my_function();
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        let _=handle.join();
    }
}

