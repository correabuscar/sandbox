//use std::sync::Arc;
use std::thread;
use std::time::Duration;
//use std::sync::atomic::{AtomicUsize, Ordering};
//use std::sync::{Mutex, MutexGuard};
//use std::convert::TryInto;
//use std::ptr::null_mut;
use libc::{cpu_set_t, sched_setaffinity, CPU_ZERO, CPU_SET};

// Shared mutable value
static mut SHARED_VALUE: usize = 0;

// Function to pin thread to a specific CPU core
fn pin_thread_to_core(core_id: usize) -> Result<(), String> {
    // Create CPU set with a single CPU core
    let mut cpu_set: cpu_set_t = unsafe { std::mem::zeroed() };
    unsafe { CPU_ZERO(&mut cpu_set) };
    unsafe { CPU_SET((core_id as libc::c_uint).try_into().unwrap(), &mut cpu_set) };

    // Set CPU affinity for current thread
    let result = unsafe { sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &cpu_set as *const _) };
    if result == 0 {
        Ok(())
    } else {
        Err(format!("Failed to set CPU affinity: {}", std::io::Error::last_os_error()))
    }
}

fn main() {
    // Pin thread 1 to CPU core 0
    let thread1 = thread::spawn(|| {
        // Pin thread to CPU core 0
        if let Err(err) = pin_thread_to_core(0) {
            eprintln!("{}", err);
            return;
        }

        // Modify the shared value
        unsafe {
            SHARED_VALUE = 42;
        }
    });

    // Pin thread 2 to CPU core 1
    let thread2 = thread::spawn(|| {
        // Pin thread to CPU core 1
        if let Err(err) = pin_thread_to_core(1) {
            eprintln!("{}", err);
            return;
        }

        // Add a sleep to introduce timing variation
        thread::sleep(Duration::from_millis(100));

        // Access the shared value
        let value;
        unsafe {
            value = SHARED_VALUE;
        }
        println!("Thread 2: Shared value = {}", value);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}

