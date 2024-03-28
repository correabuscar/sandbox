use libc::{cpu_set_t, sched_setaffinity, sched_getaffinity};
use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::thread;
use std::cell::Cell;

// Define a thread-local variable
thread_local! {
    static THREAD_LOCAL_VAR: Cell<usize> = Cell::new(0);
}

fn main() {
    // Function to pin thread to a specific CPU core
    fn pin_thread_to_core(core_id: usize) -> Result<(), String> {
        // Create CPU set with a single CPU core
        let mut cpu_set: cpu_set_t = unsafe { std::mem::zeroed() };
        unsafe { libc::CPU_ZERO(&mut cpu_set) };
        unsafe { libc::CPU_SET((core_id as libc::c_uint).try_into().unwrap(), &mut cpu_set) };

        // Set CPU affinity for current thread
        let result = unsafe { sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &cpu_set as *const _) };
        if result == 0 {
            Ok(())
        } else {
            Err(format!("Failed to set CPU affinity: {}", std::io::Error::last_os_error()))
        }
    }

    // Spawn a thread and pin it to CPU core 1
    let handle = thread::spawn(|| {
        if let Err(err) = pin_thread_to_core(1) {
            eprintln!("{}", err);
            return;
        }

        // Store a value in the thread-local variable
        THREAD_LOCAL_VAR.with(|var| {
            var.set(42);
            //*var.borrow_mut() = &42;
            //*var=42;
        });

        // Print the value stored in the thread-local variable
        THREAD_LOCAL_VAR.with(|var| {
            //println!("Thread pinned to CPU core 1: Value = {}", *var.borrow());
            println!("Thread pinned to CPU core 1: initial Value = {}", var.get());
        });

        // Migrate the thread to CPU core 2
        if let Err(err) = pin_thread_to_core(2) {
            eprintln!("{}", err);
            return;
        }

        // Print the value loaded from the thread-local variable
        THREAD_LOCAL_VAR.with(|var| {
            //println!("Thread migrated to CPU core 2: Value = {}", *var.borrow());
            println!("Thread migrated to CPU core 2: Value is still = {}", var.get());
        });

        // Update the value in the thread-local variable
        THREAD_LOCAL_VAR.with(|var| {
            //*var.borrow_mut() = &99;
            //var=&99;
            println!("Thread on CPU core 2: set value to 99 but didn't read it");
            var.set(99);
        });

        // Migrate the thread back to CPU core 1
        if let Err(err) = pin_thread_to_core(1) {
            eprintln!("{}", err);
            return;
        }

        // Print the value stored in the thread-local variable after migration
        THREAD_LOCAL_VAR.with(|var| {
            //println!("Thread migrated back to CPU core 1: Value = {}", *var.borrow());
            println!("Thread migrated back to CPU core 1: should still see new Value = {}", var.get());
        });
    });

    // Wait for the thread to finish
    handle.join().unwrap();
}

