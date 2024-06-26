use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
//initial code via chatgpt 3.5

/// Set it once and read it from anywhere(thread safe, i think).
/// Meant to be static global
/// with an initial None value
/// Holds Option<u32> which I need to keep a std::process::id() in.
struct AtomicOnceOptionU32 {
    has_value: AtomicBool,
    data: AtomicU32,
}

//unsafe impl Sync for AtomicOnceOptionU32 {} //FIXME: it's not thread safe atm., needs mutex

//FIXME: needs mutex or another atomic bool that says init is in progress (and then have threads wait until it's not)
//Instead, it's better to use OnceLock but it has MSRV 1.70.0
impl AtomicOnceOptionU32 {
    const fn none() -> Self {
        AtomicOnceOptionU32 {
            has_value: AtomicBool::new(false),
            data: AtomicU32::new(0),
        }
    }

    fn store(&self, value: Option<u32>) {
        //using 'const's here will show the hidden relationship between the relevant bools below.
        const IS_VALUE_INITIALIZED: bool = false;
        const VALUE_IS_ALREADY_INITIALIZED: bool = !IS_VALUE_INITIALIZED;
        assert_eq!(false, IS_VALUE_INITIALIZED);
        assert_eq!(true, VALUE_IS_ALREADY_INITIALIZED);
        // Use compare_exchange to set the has_value flag to true only if it's currently false
        // If compare_exchange returns Ok(false), it means that the comparison was successful (i.e., has_value was false before) and the value has been successfully set to true atomically by the current thread.
        // If compare_exchange returns Err(true), it means that the comparison failed because has_value was already true (i.e., it was set to true by another thread before the current thread attempted to set it). This indicates that the value has already been set, and attempting to set it again would be an error.
        // In both cases, compare_exchange will never return Ok(true) or Err(false). These situations don't occur in our specific usage of the function for ensuring atomic initialization of the static value. Therefore, we handle these two possible outcomes (Ok(false) and Err(true)) appropriately in our implementation of the store function.
        match self.has_value.compare_exchange(IS_VALUE_INITIALIZED, VALUE_IS_ALREADY_INITIALIZED, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(IS_VALUE_INITIALIZED) => {
                //XXX: manual delay to catch a race, FIXME: obv. needs mutex! or another atomic bool.
                std::thread::sleep(std::time::Duration::from_millis(1000));
                // If the flag was false before and successfully set to true, set the data value
                if let Some(val) = value {
                    self.data.store(val, Ordering::SeqCst);
                }
            }
            Err(VALUE_IS_ALREADY_INITIALIZED) => {
                // If the flag was already true, panic to indicate that the value has already been set
                panic!("AtomicOnceOptionU32 already set");
            }
            wicked_result => {
                //if wicked_result == Ok(true) || wicked_result == Err(false) => {
                panic!("This should never happen, unless AtomicBool::compare_exchange() if bugged or we didn't understand how it works! Got: '{:?}' but we expected only '{:?}' or '{:?}'",wicked_result, Ok::<bool,bool>(IS_VALUE_INITIALIZED), Err::<bool,bool>(VALUE_IS_ALREADY_INITIALIZED));
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


#[test]
fn test_race1() {
    static ATOMIC_ONCE_OPTION: AtomicOnceOptionU32 = AtomicOnceOptionU32::none();
    let handle = std::thread::spawn(move || {
        println!("!! Thread starts to store the value.");
        ATOMIC_ONCE_OPTION.store(Some(44));
        println!("!! Thread set the value");
    });
    std::thread::sleep(std::time::Duration::from_millis(200));
    let value = ATOMIC_ONCE_OPTION.load();
    println!("!! Main got: {:?}", value);
    handle.join().expect("Failed to join thread");
    assert_eq!(Some(44), value);
}

