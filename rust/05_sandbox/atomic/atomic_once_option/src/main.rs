use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
//initial code via chatgpt 3.5

/// Set it once and read it from anywhere.
/// Meant to be static global
/// with an initial None value
/// Holds Option<u32> which I need to keep a std::process::id() in.
struct AtomicOnceOptionU32 {
    has_value: AtomicBool,
    data: AtomicU32,
}

unsafe impl Sync for AtomicOnceOptionU32 {}

impl AtomicOnceOptionU32 {
    const fn none() -> Self {
        AtomicOnceOptionU32 {
            has_value: AtomicBool::new(false),
            data: AtomicU32::new(0),
        }
    }

    fn store(&self, value: Option<u32>) {
        // Use compare_exchange to set the has_value flag to true only if it's currently false
        match self.has_value.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                // If the flag was false before and successfully set to true, set the data value
                if let Some(val) = value {
                    self.data.store(val, Ordering::SeqCst);
                }
            }
            Err(_) => {
                // If the flag was already true, panic to indicate that the value has already been set
                panic!("AtomicOnceOptionU32 already set");
            }
        }
    }

    pub fn load(&self) -> Option<u32> {
        if self.has_value.load(Ordering::SeqCst) {
            Some(self.data.load(Ordering::SeqCst))
        } else {
            None
        }
    }
}

static ATOMIC_ONCE_OPTION: AtomicOnceOptionU32 = AtomicOnceOptionU32::none();

fn main() {
    ATOMIC_ONCE_OPTION.store(Some(42));
    // Spawn a new thread
    let handle = std::thread::spawn(move || {
        ////didn't test, shouldn't matter
        //#[cfg(target_os = "linux")]
        //{
        //    use libc::{cpu_set_t, CPU_ZERO, CPU_SET, sched_setaffinity, getpid};
        //    let mut cpuset: cpu_set_t = unsafe { std::mem::zeroed() };
        //    unsafe { CPU_SET(0, &mut cpuset) }; // Set affinity to CPU 0
        //    unsafe { sched_setaffinity(getpid(), std::mem::size_of_val(&cpuset), &cpuset) };
        //}
        // Access the value of the static inside the thread and print it
        if let Some(value) = ATOMIC_ONCE_OPTION.load() {
            println!("Value inside thread: {}", value);
        } else {
            println!("Value inside thread: None");
        }
    });
     // Wait for the thread to finish
    handle.join().expect("Failed to join thread");
    let value = ATOMIC_ONCE_OPTION.load();
    println!("value in main: {:?}", value);
    println!("Bye from main!");
}

#[test]
fn test_none() {
    static ATOMIC_ONCE_OPTION: AtomicOnceOptionU32 = AtomicOnceOptionU32::none();
    let value = ATOMIC_ONCE_OPTION.load();
    assert_eq!(None, value);

    ATOMIC_ONCE_OPTION.store(Some(41));
    let value = ATOMIC_ONCE_OPTION.load();
    assert_eq!(Some(41), value);
}
#[test]
//#[should_panic]
fn test_double_set() {
    static ATOMIC_ONCE_OPTION: AtomicOnceOptionU32 = AtomicOnceOptionU32::none();
    //ATOMIC_ONCE_OPTION = AtomicOnceOptionU32::none(); // E0594: cannot assign to immutable static item `ATOMIC_ONCE_OPTION` cannot assign


    ATOMIC_ONCE_OPTION.store(Some(42));
    let value = ATOMIC_ONCE_OPTION.load();
    assert_eq!(Some(42), value);
    let ret=std::panic::catch_unwind(|| {
        ATOMIC_ONCE_OPTION.store(Some(41));
    });
    //println!("{:?}",ret);
    assert!(ret.is_err(), "should've panicked1");
    let value = ATOMIC_ONCE_OPTION.load();
    assert_eq!(Some(42), value);
    let ret=std::panic::catch_unwind(|| {
    ATOMIC_ONCE_OPTION.store(None);
    });
    assert!(ret.is_err(), "should've panicked2");
    let value = ATOMIC_ONCE_OPTION.load();
    assert_eq!(Some(42), value);
}


