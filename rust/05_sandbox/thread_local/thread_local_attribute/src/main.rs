#![feature(thread_id_value)]
#![feature(thread_local)]
use std::cell::UnsafeCell;
use std::thread;

/* "fork requires all functions called after it to be async-signal safe, which both pthread_key_* and #[thread_local] are not documented to be, so all TLS accesses after fork risk running into deadlocks if the platform has not considered this extremely niche use-case. "
src: https://github.com/rust-lang/rust/issues/122940#issuecomment-2016600046

"An evaluation is signal-safe unless it includes one of the following:
an access to an object with thread storage duration;"
src: https://eel.is/c++draft/support.runtime#support.signal-3.2  via https://github.com/rust-lang/rust/issues/122940#issuecomment-2016611311
*/
//Presumably this #[thread_local] attribute is non-heap allocating thread_local, which is unlike thread_local!() macro which uses pthread_key_create which is allocating.
#[thread_local]
static RECURSION_FLAG: UnsafeCell<bool> = UnsafeCell::new(false);
//XXX: yes each thread has a different address for this!
/*
 * Unclear how the following apply:
 *
 * rustc -Z tls-model=
    -Z                                     tls-model=val -- choose the TLS model to use (`rustc --print tls-models` for details)

 * $ rustc --print tls-models
Available TLS models:
    global-dynamic
    local-dynamic
    initial-exec
    local-exec
    emulated

see: https://github.com/rust-lang/rust/issues/29594#issuecomment-1874468467

*/

fn my_non_allocating_function() {
    // Access the thread-local recursion flag
    let already_in_function = unsafe {
        let flag = &mut *RECURSION_FLAG.get();
        let was_set = *flag;
        *flag = true;
        was_set
    };

    if already_in_function {
        println!("Already in the function, detected recursion! in thread {:?}", thread::current().id());
        return;
    }

    // Function logic goes here
    println!("Function logic executed in thread {:?}", thread::current().id());
    std::thread::sleep(std::time::Duration::from_secs(4-u64::try_from(thread::current().id().as_u64()).unwrap() ));
    println!("About to re-execute function in thread {:?}", thread::current().id());
    my_non_allocating_function();
    // Reset the flag before returning
    unsafe {
        *RECURSION_FLAG.get() = false;
    }
    let flag_address = &RECURSION_FLAG as *const UnsafeCell<bool>;
    let bool_address = RECURSION_FLAG.get() as *const bool;
    //assert_eq!(bool_address, flag_address);
    assert_eq!(flag_address as usize, bool_address as usize);
    assert_eq!(bool_address, RECURSION_FLAG.get());
    println!("Done with function in thread {:?} {:?}", thread::current().id(), RECURSION_FLAG.get());
}

fn thread_function() {
    println!("Thread {:?} starting", thread::current().id());
    std::thread::sleep(std::time::Duration::from_secs(3-u64::try_from(thread::current().id().as_u64()).unwrap() ));
    my_non_allocating_function(); // This will detect recursion
}

fn main() {
    let thread1 = thread::spawn(thread_function);
    let thread2 = thread::spawn(thread_function);

    thread1.join().unwrap();
    thread2.join().unwrap();
}

