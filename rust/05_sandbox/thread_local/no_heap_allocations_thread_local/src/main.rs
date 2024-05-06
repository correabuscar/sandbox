#![feature(thread_id_value)]

//use std::sync::Mutex;
//use std::mem;
use std::mem::MaybeUninit;
use std::num::NonZeroU64;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;

#[derive(Debug)]
struct NoHeapAllocThreadLocal<const N: usize, T> {
    //data: [Option<Mutex<T>>; N],
    //data: [Mutex<Option<T>>; N],
    //data: [Option<T>; N],
    data: [AtomicU64; N],
    values: [MaybeUninit<T>; N],
}

impl<const N: usize, T> NoHeapAllocThreadLocal<N, T> {
    const ARRAY_INITIALIZER_REPEAT_VALUE: AtomicU64 = AtomicU64::new(0);

    const fn new() -> Self {
        //const ARRAY_REPEAT_VALUE: Option<T> = None;
        //const ARRAY_REPEAT_VALUE: Mutex<Option<T>> = Mutex::new(None);
        Self {
            //data: unsafe { std::mem::zeroed() }, //[None; N], // that fails needed T:Copy
            data: [Self::ARRAY_INITIALIZER_REPEAT_VALUE; N],
            values: unsafe { std::mem::zeroed() }, //[None; N], // that fails needed T:Copy
        }
    }

    fn acquire(
        &self,
        thread_id: std::num::NonZeroU64,
        timeout: Duration,
    ) -> Option<usize> {
        let start_time = std::time::Instant::now();
        let expected = 0; //aka unused
        let new_value: u64 = thread_id.into();
        //if we have already allocated it, return early
        let mut first_empty: Option<usize> = None; //start of index should be 0
        for (index, atomic_value) in self.data.iter().enumerate() {
            let current = atomic_value.load(Ordering::Acquire);
            match current {
                0 => {
                    if first_empty.is_none() {
                        first_empty = Some(index);
                    }
                    break;
                }
                our_tid if our_tid == new_value => {
                    //found one of ours, should be only one anyway!
                    return Some(index);
                }
                _any_else => {
                    //owned by another thread
                    //fall thru
                }
            } //match
        } //for

        let first_empty:usize=/*shadowed*/ if let Some(index)=first_empty {
            index
        } else {
            //we haven't found any empty, so we try from beginning always
            0
        };
        assert!(first_empty < N, "we coded it badly");
        loop {
            //for (index, atomic_value) in self.data.iter().enumerate() {
            for index in first_empty..N {
                //attempts to acquire the first one that was found empty,
                //unless another thread got it already, then we keep trying
                //TODO: don't get starved
                let atomic_value: &AtomicU64 = &self.data[index];
                match atomic_value.compare_exchange(
                    expected,
                    new_value,
                    Ordering::Release,
                    Ordering::Acquire,
                ) {
                    Ok(what_was) => {
                        assert_eq!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly");
                        return Some(index);
                    }
                    Err(what_was) => {
                        assert_ne!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly");
                        if start_time.elapsed() >= timeout {
                            // Timeout reached
                            return None;
                            //} else { // fall thru aka continue;
                            //    continue;
                        };
                    }
                };
            } //for
            if start_time.elapsed() >= timeout {
                return None; // Timeout reached
            };
            // Sleep for a short duration before retrying
            std::thread::sleep(Duration::from_millis(10));
            //TODO: put it to sleep until another thread releases any array element?
        }
    } //fn
}

#[derive(Debug)]
struct MyType(i32);

fn main() {
    println!("Hello thread local without any allocations on heap");
    const HOW_MANY: usize = 10;
    let foo: NoHeapAllocThreadLocal<HOW_MANY, MyType> = NoHeapAllocThreadLocal::new();
    println!("{:?}", foo);

    //let current_thread_id = std::thread::current().id().as_u64();
    let mut current_thread_id: NonZeroU64 = NonZeroU64::new(1).unwrap();
    for _i in 1..=HOW_MANY {
        if let Some(index) =
            foo.acquire(current_thread_id, std::time::Duration::from_secs(1))
        {
            println!(
                "Slot allocated for thread {} at index: {}",
                current_thread_id, index
            );
        } else {
            println!("No available slots found for thread {}", current_thread_id);
        }
        current_thread_id = NonZeroU64::new(1 + current_thread_id.get()).unwrap();
    }
    if let Some(index) =
        foo.acquire(current_thread_id, std::time::Duration::from_secs(1))
    {
        println!("Slot allocated for thread at index: {}", index);
    } else {
        println!("No available slots found for thread {}", current_thread_id);
    }
}
