use parking_lot::ReentrantMutex;
use std::sync::Arc;
use std::thread;

fn main() {
    // Create an Arc containing a ReentrantMutex with an integer value.
    let mutex = Arc::new(ReentrantMutex::new(0));

    // Clone the Arc to share it with the recursive function.
    let mutex_clone = Arc::clone(&mutex);

    // Spawn a new thread to run the recursive function.
    let handle = thread::spawn(move || {
        recursive_lock(&mutex_clone, 3);
    });

    // Wait for the thread to finish.
    handle.join().unwrap();
}

fn recursive_lock(mutex: &Arc<ReentrantMutex<i32>>, depth: usize) {
    if depth == 0 {
        return;
    }

    // Attempt to lock the mutex.
    let guard = mutex.lock();
    println!("Locked at depth {} {}", depth, guard);

    // Recursive call - this will lock the mutex again without deadlock.
    recursive_lock(mutex, depth - 1);
println!("still Locked at depth {} {}", depth, guard);


    // Mutex will be unlocked when `_guard` goes out of scope.
}

