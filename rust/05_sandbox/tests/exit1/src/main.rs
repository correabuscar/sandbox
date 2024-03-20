use libc::atexit;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU64,Ordering};

static HOOKS: AtomicU64 = AtomicU64::new(0);

#[allow(dead_code)]
extern "C" fn cleanup() {
//fn cleanup() {
    let hooks = HOOKS.load(Ordering::SeqCst);
    println!("! project's Cleaning up resources before exit... hooks registered={}",hooks);
}

fn set_exit_hook() {
    // Register the exit handler
    unsafe {
        let result = atexit(cleanup);// as extern "C" fn());
        if result != 0 {
            panic!("Failed to register project's exit handler");
        } else {
            HOOKS.fetch_add(1, Ordering::SeqCst);
        }
    }
}

fn main() {
    // Your program logic here

    // Exit normally
    std::process::exit(4);
}


#[test]
fn test_ok1() {
    set_exit_hook();
    thread::sleep(Duration::from_millis(1000));
    println!("Hello from the invisible test* named test_ok1! *unless you used 1 test thread");
}

#[test]
fn test_exit0() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(1));
    println!("Hello from the invisible test* named test_exit0! *unless you used 1 test thread");
    std::process::exit(0); // this does.
    //panic!("on purpose"); // doesn't affect the test harness
}

#[test]
fn test_that_panics() {
    set_exit_hook();
    thread::sleep(Duration::from_millis(1100));
    println!("Hello from the invisible test* named test_that_panics! *unless you used 1 test thread");
    panic!("on purpose"); // doesn't affect the test harness
}

#[test]
fn test_exit4() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(1));
    //set_exit_hook();
    println!("Hello from the invisible test* named test_exit4!");
    thread::sleep(Duration::from_secs(3));
    std::process::exit(4);
}
#[test]
fn test_abort() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(2));
    //set_exit_hook();
    println!("Hello from the invisible test* named test_abort!");
    //std::process::abort()//uncaught!
    std::process::exit(6);
}
#[test]
fn test_ok2() {
    set_exit_hook();
    thread::sleep(Duration::from_millis(1000));
    println!("Hello from the invisible test* named test_ok2! *unless you used 1 test thread");
}
