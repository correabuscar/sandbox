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
static mut ORDERING1: bool = false;
static mut ORDERING2: bool = false;

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
    if !cfg!(debug_assertions) {
        panic!("assertions are disabled");
    }
    // Set up a flag to indicate whether Ctrl+C has been pressed
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    let must_quit = Arc::new(AtomicBool::new(false));
    let r = must_quit.clone();

    // Register Ctrl+C handler
    ctrlc::set_handler(move || {
        // Set the flag when Ctrl+C is pressed
        r.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    use std::sync::Barrier;
    // Create a barrier that waits for 2 threads to reach it
    let barrier = Arc::new(Barrier::new(2));

    // Clone the barrier for each thread
    let barrier_clone = Arc::clone(&barrier);
    // Create an unbuffered channel for synchronization of flow between threads
    //let five_sec=Duration::from_secs(5);
    //use std::sync::mpsc;
    //XXX: need two receivers because can't clone receiver! so 2 channels needed, because mpmc is
    //private module!
    //let (sender1, receiver1) = mpsc::sync_channel::<()>(0);
    //let (sender2, receiver2) = mpsc::sync_channel::<()>(0);
    //sender1.send_timeout((), five_sec).unwrap();//tf: method `send_timeout` is private: private method
    //receiver1.recv_timeout(five_sec).unwrap();//at least this exists!
    //use std::sync::mpmc;//tf: module `mpmc` is private: private module
    //let (sender1, receiver1) = mpsc::sync_channel::<()>(0);

    #[inline]
    fn should_show(reset_to:Option<bool>) -> bool {
        lazy_static::lazy_static! {
            static ref DEBUG_FLAG: AtomicBool = AtomicBool::new(std::env::var("DEBUG").map_or(false, |v| v != "0"));
        }
        if let Some(new_val)=reset_to {
            DEBUG_FLAG.store(new_val, std::sync::atomic::Ordering::SeqCst);
        }
        return DEBUG_FLAG.load(std::sync::atomic::Ordering::SeqCst);
    }

    fn set_val(val:usize, thread_num:u8, step_num:u8) {
        unsafe {
            SHARED_VALUE = val;
        }
        if should_show(None) {
            println!("step{step_num}:Thread {thread_num}: set value to {val} without re-reading it");
        }
    }

    fn read_val(expected:usize,thread_num:u8, step_num:u8) -> usize {
        let value;
        unsafe {
            value = SHARED_VALUE;
        }
        if should_show(None) {
            println!("step{step_num}:Thread {thread_num}: reads shared value = {value} expected={expected}");
        }
        assert_eq!(value, expected);
        return value;
    }

    //#[inline(always)]
    let should_quit=move || {
            if must_quit.load(Ordering::SeqCst) {
                if should_show(None) {
                    println!("Quitting true");
                    return true;
                } else {
                    //one more run with debug on, then quit!
                    //DEBUG_FLAG.store(true,std::sync::atomic::Ordering::SeqCst);
                    println!("Should quit detected, trying to enable DEBUG=1 before actually quitting next");
                    should_show(Some(true));
                    println!();
                }
            }
            return false;
    };
    let should_quit2=should_quit.clone();

    // Pin thread 1 to CPU core 0
    let thread1 = thread::Builder::new()
        .name("thread1".to_owned())
        .spawn(move || {
        // Pin thread to CPU core 0
        if let Err(err) = pin_thread_to_core(0) {
            eprintln!("{}", err);
            return;
        }
        // Wait for all threads to reach the barrier
        unsafe { ORDERING2=true; }
        barrier_clone.wait();
        //while false == unsafe { ORDERING1 } {
        //    thread::sleep(Duration::from_millis(20));
        //}
        println!("start: Thread 1: ready");
        let mut set_to=42;
        let mut expected=920;
        let mut sq;
        'mainl: loop {
            // Modify the shared value
            //barrier_clone.wait();
            set_val(set_to,1,1);
            unsafe { ORDERING2=false; }
            while true == unsafe { ORDERING1 } {
                thread::sleep(Duration::from_millis(20));
                sq=should_quit();
                if sq {
                    println!("Thread 1 must quit detected!");
                    break 'mainl;
                }
                //print!("1");
                //use std::io::Write;
                //std::io::stdout().flush().expect("Failed to flush stdout");
            }
            unsafe { ORDERING1=true; }
            //if let Err(e)=sender1.send(()) {
            //    println!("Thread1: Gave up trying to send, err: '{e}'");
            //    break;
            //}
            //if let Err(e) = receiver2.recv_timeout(five_sec) {
            //    println!("Thread1: Gave up trying to receive, err: '{e}'");
            //    break;
            //}
            //barrier_clone.wait();
            read_val(expected, 1,4);
            //barrier_clone.wait();
            //if let Err(e)=sender1.send(()) {
            //    println!("Thread1: Gave up trying to send, err: '{e}'");
            //    break;
            //}
            //if let Err(e) = receiver2.recv_timeout(five_sec) {
            //    println!("Thread1: Gave up trying to receive, err: '{e}'");
            //    break;
            //}
            //while ! unsafe { ORDERING } {}
            expected+=1;
            set_to+=1;
            sq=should_quit();
            if sq {
                break;
            }
        }
        println!("end:Thread 1: waits 2sec");
        thread::sleep(Duration::from_millis(2000));
    }).unwrap();

    // Pin thread 2 to CPU core 1
    let thread2 = thread::Builder::new()
        .name("thread2".to_owned())
   // let thread2 = thread::
    .spawn(move || {
        // Pin thread to CPU core 1
        if let Err(err) = pin_thread_to_core(1) {
            eprintln!("{}", err);
            return;
        }
        // Wait for all threads to reach the barrier
        unsafe { ORDERING1=true; }
        barrier.wait();

        //thread::sleep(Duration::from_millis(200));
//        while false == unsafe { ORDERING2 } {
//            thread::sleep(Duration::from_millis(20));
//        }
        println!("start: Thread 2: ready");
        let mut expected=42;
        let mut set_to=920;
        let mut sq;
        'mainl: loop {
            // Add a sleep to introduce timing variation
            //thread::sleep(Duration::from_millis(100));

            //barrier.wait();
            //if let Err(e) = receiver1.recv_timeout(five_sec) {
            //    println!("Thread2: Gave up trying to receive, err: '{e}'");
            //    break;
            //}
            while true == unsafe { ORDERING2 } {
                thread::sleep(Duration::from_millis(20));
                sq=should_quit2();
                if sq {
                    println!("Thread 2 must quit detected!");
                    break 'mainl;
                }
                //print!("2");
                //use std::io::Write;
                //std::io::stdout().flush().expect("Failed to flush stdout");
            }
            read_val(expected, 2,2);
            set_val(set_to, 2,3);
            unsafe { ORDERING1=false; }
            unsafe { ORDERING2=true; }
            //if let Err(e)=sender2.send(()) {
            //    println!("Thread2: Gave up trying to send, err: '{e}'");
            //    break;
            //}
            expected+=1;
            set_to+=1;
            sq=should_quit2();
            if sq {
                break;
            }
        }
        println!("end:Thread 2: waits 2sec");
        thread::sleep(Duration::from_millis(2000));
    }).unwrap();

    thread1.join().unwrap();
    thread2.join().unwrap();
}

