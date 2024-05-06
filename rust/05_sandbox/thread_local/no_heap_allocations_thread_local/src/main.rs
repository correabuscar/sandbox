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
    const NO_THREAD_ID: u64 = 0; //aka unused slot/element
    const ARRAY_INITIALIZER_REPEAT_VALUE: AtomicU64 = AtomicU64::new(Self::NO_THREAD_ID);

    const fn new() -> Self {
        //const ARRAY_REPEAT_VALUE: Option<T> = None;
        //const ARRAY_REPEAT_VALUE: Mutex<Option<T>> = Mutex::new(None);
        Self {
            //data: unsafe { std::mem::zeroed() }, //[None; N], // that fails needed T:Copy
            data: [Self::ARRAY_INITIALIZER_REPEAT_VALUE; N],
            values: unsafe { std::mem::zeroed() }, //[None; N], // that fails needed T:Copy
        }
    }

    fn acquire_index(
        &self,
        //thread_id: std::num::NonZeroU64,
        timeout: Duration,
    ) -> Option<usize> {
        let start_time = std::time::Instant::now();
        let current_thread_id:NonZeroU64 = std::thread::current().id().as_u64();
        let current_thread_id:u64=current_thread_id.get();
        assert!(current_thread_id > 0,"impossible");
        //const expected:u64 = Self::NO_THREAD_ID; //this compile errors!
        let expected: u64 = Self::NO_THREAD_ID;//but this works, odd.
        //let new_value: u64 = current_thread_id.into();
        let new_value: u64 = current_thread_id;
        //if we have already allocated it, return early
        let mut index_of_first_empty: Option<usize> = None; //start of index should be 0
        for (index, atomic_value) in self.data.iter().enumerate() {
            let current = atomic_value.load(Ordering::Acquire);
            match current {
                Self::NO_THREAD_ID => {
                    if index_of_first_empty.is_none() {
                        index_of_first_empty = Some(index);
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

        let index_of_first_empty:usize=/*shadowed*/ if let Some(index)=index_of_first_empty {
            index
        } else {
            //we haven't found any empty, so we try from beginning always
            0
        };
        assert!(index_of_first_empty < N, "we coded it badly");
        loop {
            //for (index, atomic_value) in self.data.iter().enumerate() {
            for index in index_of_first_empty..N {
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

const HOW_MANY: usize = 10;
static FOO: NoHeapAllocThreadLocal<HOW_MANY, MyType> = NoHeapAllocThreadLocal::new();

fn main() {
    println!("Hello thread local without any allocations on heap");
    println!("{:?}", FOO);

    let mut handles = Vec::new();
    for i in 1..=HOW_MANY*2 {
        let handle=std::thread::spawn(move || {
            let current_thread_id = std::thread::current().id().as_u64();
            if let Some(index) = FOO.acquire_index(std::time::Duration::from_secs((i/2) as u64)) {
                println!(
                    "Slot allocated for thread {} at index: {}",
                    current_thread_id, index
                );
            } else {
                println!("No available slots found for thread {}", current_thread_id);
            }
        }); //spawn
        handles.push(handle);
    }
     // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", FOO);
    println!("All threads have finished");
}
