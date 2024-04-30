#![feature(panic_always_abort)]
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
fn test_aaok1() {
    set_exit_hook();
    thread::sleep(Duration::from_millis(1000));
    println!("Hello from the invisible test* named test_aaok1! *unless you used 1 test thread");
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
    std::process::abort()//uncaught! well, caught with patch.
    //std::process::exit(6);
}
#[test]
fn test_abort2() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(2));
    //set_exit_hook();
    println!("Hello from the invisible test* named test_abort2!");
    std::process::abort()//uncaught! well, caught with patch.
    //std::process::exit(6);
}

#[test]
fn test_abort_due_to_double_panic() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(2));
    //set_exit_hook();
    println!("Hello from the invisible test* named test_abort_due_to_double_panic!");

    use std::fmt::{Display, self};

    struct MyStruct;
    impl Display for MyStruct {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            //todo!()
            panic!("oh no panic in Display");
        }
    }
    let instance = MyStruct;

    println!("{}", instance);
}

#[test]
#[ignore] //TODO: not yet ready to test this!
fn test_abort_due_to_double_panic_when_always_abort_is_set() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(2));
    //set_exit_hook();
    println!("Hello from the invisible test* named test_abort_due_to_double_panic_when_always_abort_is_set!");

    use std::fmt::{Display, self};

    struct MyStruct;
    impl Display for MyStruct {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            //todo!()
            panic!("oh no panic in Display");
        }
    }
    let instance = MyStruct;

    //FIXME: if always_abort() then this causes an exit(128+6), probably due to missing one more atexit handler in test harness!
    std::panic::always_abort();//issue: https://github.com/rust-lang/rust/issues/122940
    println!("{}", instance);
}

#[test]
#[ignore] //TODO: not yet ready to test this!
fn test_abort_due_to_infinitely_nested_panics() {
    set_exit_hook();
    thread::sleep(Duration::from_secs(2));
    //set_exit_hook();
    println!("Hello from the invisible test* named test_abort_due_to_infinitely_nested_panics!");

    use std::fmt::{Display, self};

    struct MyStruct2;
    impl Display for MyStruct2 {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            //todo!()
            let instance = MyStruct;
            panic!("oh1 no, '{}' was unexpected", instance);
        }
    }
    struct MyStruct;
    impl Display for MyStruct {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            let instance2 = MyStruct2;
            panic!("oh2 no, '{}' was unexpected", instance2);
            //todo!()
        }
    }
    let instance = MyStruct;

    std::panic::always_abort();//issue: https://github.com/rust-lang/rust/issues/122940
    //actually attempts to do infinite recurios of panics
    println!("{}", instance);
}

#[test]
fn test_ok2() {
    set_exit_hook();
    thread::sleep(Duration::from_millis(1000));
    println!("Hello from the invisible test* named test_ok2! *unless you used 1 test thread");
}
