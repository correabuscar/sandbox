#![feature(thread_id_value)]



mod my_mod {
use std::num::NonZeroU64;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::mem::MaybeUninit;
use std::cell::RefCell;

#[derive(Debug)]
pub struct NoHeapAllocThreadLocal<const N: usize, T> {
    //create 3 static arrays of size N, preallocated, presumably on stack, but depends on caller doesn't it!
    before: [AtomicU64; N],
    values: [MaybeUninit<RefCell<T>>; N],//FIXME: ensure that i properly r/w this below
    after: [AtomicU64; N],
}

/* FALSE(chatgpt):" RefCell does use heap allocation internally to manage its borrow checking. It uses dynamic borrowing rules at runtime rather than static borrowing rules enforced by the Rust compiler. This dynamic borrowing is implemented through reference counting and interior mutability, which involves heap allocation for the reference count and the data being managed. This allows RefCell to provide runtime borrow checking and interior mutability without violating Rust's borrowing rules." - chatgpt 3.5
 * ^ apparely a lie because I've found this (and Gemini) that says no heap alloc: https://users.rust-lang.org/t/is-refcell-allocated-in-the-heap/9173/2
 * (true):"RefCell does not allocate, but it contains an additional "borrow state" indicator (one word in size) along with the data.

At runtime each borrow causes a modification/check of the refcount." -src: https://doc.rust-lang.org/1.30.0/book/first-edition/choosing-your-guarantees.html#cost-2
 */

impl<const N: usize, T> Drop for NoHeapAllocThreadLocal<N, T> {
    fn drop(&mut self) {
        //TODO: can this be called concurrently? then we may have a problem.
        //TODO: can this be called by one thread while another one tries to set a value?!
        //FIXME: ensure this is properly implemented!
        for (index, _elem) in &mut self.values.iter().enumerate() {
            //match self.after[index].compare_exchange(what_was,

            //We don't know what thread id was there before, to compare_exchange()
            let was_set=Self::NO_THREAD_ID != self.after[index].load(Ordering::Release);
            self.after[index].store(Self::NO_THREAD_ID, Ordering::Release);
            if was_set {
                // Calling this when the content is not yet fully initialized causes undefined behavior.
                unsafe { self.values[index].assume_init_drop(); }
            }
            self.before[index].store(Self::NO_THREAD_ID, Ordering::Release);
        }
        drop(self.after);
        drop(self.values);
        drop(self.before);
    }
}

//impl<const N: usize, T> Drop for NoHeapAllocThreadLocal<N, T> {
//    fn drop(&self) {
//        self.unset();
//    }
//}

//impl<const N: usize, T/*:std::fmt::Debug + PartialEq + Clone*/> NoHeapAllocThreadLocal<N, T> {
impl<const N: usize, T> NoHeapAllocThreadLocal<N, T> {
    const NO_THREAD_ID: u64 = 0; //aka unused slot/element
    const ARRAY_INITIALIZER_REPEAT_VALUE: AtomicU64 = AtomicU64::new(Self::NO_THREAD_ID);

    // this const fn gets computed at compile time.
    pub const fn new() -> Self {
        let mut index = 0;
        /* "This line initializes each element of the values array with uninitialized memory and then assumes that the uninitialized memory represents valid instances of RefCell<T>. This is done by calling assume_init()." */
        //let mut values: [MaybeUninit<RefCell<T>>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        /* "After this initialization, the values array contains elements that are zeroed out, but they are not valid instances of RefCell<T>. They are simply zeroed memory." */
        let mut values: [MaybeUninit<RefCell<T>>; N] = unsafe { std::mem::zeroed() };

        // Use while loop for initialization
        while index < N {
            // Initialize each element with MaybeUninit::zeroed()
            values[index] = MaybeUninit::uninit();
            index += 1;
        }

        Self {
            before: [Self::ARRAY_INITIALIZER_REPEAT_VALUE; N],
            //In uninitialized state initially, these will never be read before overwriting with valid T instance first! on a per element basis!
            //values: unsafe { std::mem::zeroed() },//good! //[None; N], // that fails needed T:Copy
            values,
            after: [Self::ARRAY_INITIALIZER_REPEAT_VALUE; N],
        }
    }

//    pub fn get_or_set(&self, to_val:T, timeout: Duration) -> Option<T> {
//        let (existed_before, possible_index) = self.acquire_index(timeout);
//        if let Some(index)=possible_index {
//            if existed_before {
//                let current_val=unsafe { self.values[index].assume_init() };
//                return Some(current_val);
//            } else {
//                // it didn't exist before, we must add it
//                self.values[index].write(to_val);
//                return Some(to_val);
//            }
//        } else {
//            //timeout reached when full
//            return None;
//        }
//    }

    //TODO: a fn that drops our value from both fields!
    pub fn unset(&self) {
        if let Some((index,_ref_cell))=self.maybe_get_ref_if_set() {
            let my_tid=get_current_thread_id();
            let expected=my_tid;
            let new_value=Self::NO_THREAD_ID;
            match self.after[index].compare_exchange(expected,new_value,Ordering::Release, Ordering::Acquire) {
                Ok(prev_val) => {
                    assert_eq!(prev_val, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1)");
                    drop(self.values[index]);
                    self.values[index]=MaybeUninit::uninit();
                    match self.before[index].compare_exchange(expected,new_value, Ordering::Release, Ordering::Acquire) {
                        Ok(prev_val) => {
                            assert_eq!(prev_val, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(2)");
                            //ok, successfully unset that, in 3 thread-safe steps.
                            //fall thru
                        },
                        Err(prev_val) => {
                            assert_ne!(prev_val, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(3)");
                            panic!("This should not have happened, something's broken in our code logic(1)");
                        }
                    }//match
                },
                Err(prev_val) => {
                    assert_ne!(prev_val, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(4)");
                    //unless our current thread calls this unset() and a fn. that sets somehow concurrently, this shouldn't happen! like the set fn would happen between the 'maybe' and 'compare' from above, like if get_current_thread_id() panics for example.
                    panic!("This should not have happened, something's broken in our code logic(2)");
                },
            }//match

        }//if
        //else, it was already unset.
    }
    pub fn maybe_get_ref_if_set(&self) -> Option<(usize,&RefCell<T>)> {
        let our_current_tid: u64 = get_current_thread_id();
        for (index, atomic_value) in self.after.iter().enumerate() {
            //TODO: fix the orderings, if they're too strict.
            let thread_id_at_index = atomic_value.load(Ordering::Acquire);
            if our_current_tid == thread_id_at_index {
                //let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
                //let mut_ref_to_value=unsafe { &mut *value_ptr };
                let current_val=unsafe { self.values[index].assume_init_ref() };
                return Some((index,current_val));
            }
        } //for
        None
    }//fn

    /// returns true if it was already set(and thus we just found it again)
    /// returns false if it wasn't already set, and either we found a spot for it or we didn't.
    /// If no slots are available, retry until timeout in which case returns None,
    /// if success returns a mutable ref to the  just set OR prev.  value
    /// Since the value is supposed to be accessible only on current thread, it's not protected or
    /// wrapped into some kind of sync. primitive, so you've direct mutability to it.
    pub fn get_or_set(
        &self,
        to_val:T,
        //thread_id: std::num::NonZeroU64,
        timeout: Duration,
    ) -> (bool,Option<&mut T>) {
        let start_time = std::time::Instant::now();
        //let new_value: u64 = current_thread_id.into();
        //let new_value: u64 = NoHeapAllocThreadLocal::<N, T>::get_current_thread_id();
        let new_value: u64 = get_current_thread_id();
        //if we have already allocated it, return early
        let mut index_of_first_empty: Option<usize> = None;
        for (index, atomic_value) in self.data.iter().enumerate() {
            let thread_id_at_index = atomic_value.load(Ordering::Acquire);
            match thread_id_at_index {
                Self::NO_THREAD_ID => {
                    if index_of_first_empty.is_none() {
                        index_of_first_empty = Some(index);
                    }
                    //break;
                    //fall thru to keep going maybe we find ours later on
                },
                our_tid if our_tid == new_value => {
                    //found one of ours, should be only one anyway!
                    //let current_val=unsafe { self.values[index].assume_init() };
                    let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
                    let mut_ref_to_value=unsafe { &mut *value_ptr };
                    //let current_val=&mut self.values[index];
                    //return (true, Some(current_val));
                    return (true, Some(mut_ref_to_value));
                },
                _any_else => {
                    //owned by another thread
                    //fall thru to try next element
                },
            } //match
        } //for

        let index_of_first_empty:usize=/*shadowed*/ if let Some(index)=index_of_first_empty {
            index
        } else {
            // Sleep for a short duration before retrying
            std::thread::sleep(Duration::from_millis(10));
            //we haven't found any empty, so we try from beginning always
            0
        };
        assert!(index_of_first_empty < N, "we coded it badly");
        //const expected:u64 = Self::NO_THREAD_ID; //XXX: this compile errors! can't use generic parameters from outer item: use of generic parameter from outer item
        let expected: u64 = Self::NO_THREAD_ID;//but this works, odd.
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
                        //FIXME:there's a period of time between the compare_exchange call and until the value
                        //in values[index] is set(below) during which IF the thread now owning it recurses
                        //(or it somehow gets to read that value),
                        //then it can read the unset(or prev.) value!
                        //so maybe we need something like 2 atomics: one saying it's in progress
                        //and one saying it's update(the assignment below) is finished;
                        //and if we encounter the in-progress one before setting it, it means we've recursed
                        //somehow(like this is used in panic handling code and we've panicked
                        //during the period of time aforementioned).
                        //TODO: don't actually panic anywhere? or maybe expect caller to catch_unwind() ?
                        assert_eq!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(5)");
                        // it didn't exist before, we must add it
                        //self.values[index].write(to_val);
                        //self.values[index]=to_val;//cannot assign to `self.values[_]`, which is behind a `&` reference: `self` is a `&` reference, so the data it refers to
                        // Get a raw pointer to the element at the specified index
                        let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
                        //let value_ptr= unsafe { &self.values[index] as *const T as *mut T };
                        // Assign the new value
                        unsafe { *value_ptr = to_val; }
                        //unsafe { *value_ptr = to_val.clone(); }
                        //assert_eq!(self.values[index], to_val,"assignment failed, coded wrongly?");//need to restrict T to Debug, PartialEq and Clone for this assert to compile!
                        //assert_eq!(self.values[index], to_val,"assignment failed, coded wrongly?");
                        //assert!(
                        //    //can't access &val, it's moved!
                        //    Self::unsafe_compare_memory(&self.values[index], &to_val),
                        //    "assignment failed, coded wrongly?");
                        let mut_ref_to_value=unsafe { &mut *value_ptr };
                        //return (false,Some(unsafe { &mut *value_ptr }));//self.values[index]));
                        return (false,Some(mut_ref_to_value));//self.values[index]));
                    }
                    Err(what_was) => {
                        assert_ne!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(6)");
                        if start_time.elapsed() >= timeout {
                            // Timeout reached
                            return (false,None);
                            //} else { // fall thru aka continue;
                            //    continue;
                        };
                    }
                };
            } //for
            if start_time.elapsed() >= timeout {
                return (false,None); // Timeout reached
            };
            // Sleep for a short duration before retrying
            std::thread::sleep(Duration::from_millis(10));
            //TODO: put it to sleep until another thread releases any array element?
        }
    } //fn

//        // Function to unsafely compare the memory contents of two instances of ComplexType
//        unsafe fn unsafe_compare_memory(a: &T, b: &T) -> bool {
//            // Get raw pointers to the memory locations of the instances
//            let a_ptr = a as *const T as *const u8;
//            let b_ptr = b as *const T as *const u8;
//
//            // Get the size of type T in bytes
//            let size = std::mem::size_of::<T>();
//
//            // Compare the memory contents byte by byte
//            for i in 0..size {
//                if *a_ptr.offset(i as isize) != *b_ptr.offset(i as isize) {
//                    eprintln!("Difference at offset {}",i);
//                    return false;
//                }
//            }
//
//            true
//        }
}//impl

    pub fn get_current_thread_id() -> u64 {
        //TODO: here's a question, does this alloc on heap anything, internally?! because that'd be bad.
        let current_thread_id:NonZeroU64 = std::thread::current().id().as_u64();
        let current_thread_id:u64=current_thread_id.get();
        assert!(current_thread_id > 0,"impossible");
        return current_thread_id;
    }

}

pub use my_mod::NoHeapAllocThreadLocal;
pub use my_mod::get_current_thread_id;
