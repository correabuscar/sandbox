#![feature(thread_id_value)]



mod my_mod {
use std::num::NonZeroU64;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
//use std::mem::MaybeUninit;
use std::cell::{Ref, RefCell, RefMut};
use core::mem::ManuallyDrop;

#[derive(Debug)]
pub struct NoHeapAllocThreadLocal<const N: usize, T> {
    //create 3 static arrays of size N, preallocated, presumably on stack, but depends on type T doesn't it!
    //we need the 2 atomic arrays because a 'values' element is attached to one specific thread which shouldn't outlive it, or may even get manually dropped by caller, to allow for other thread(s) to use that spot
    //if 'before' is set, this means the value setting is in progress.
    before: [AtomicU64; N],
    //values: [MaybeUninit<RefCell<T>>; N],//FIXME: ensure that i properly r/w this below
    values: [ManuallyDrop<RefCell<Option<T>>>; N],
    //if 'after' is set, it means the value has already been set and is thus safe to read
    after: [AtomicU64; N],
}

/* FALSE(chatgpt):" RefCell does use heap allocation internally to manage its borrow checking. It uses dynamic borrowing rules at runtime rather than static borrowing rules enforced by the Rust compiler. This dynamic borrowing is implemented through reference counting and interior mutability, which involves heap allocation for the reference count and the data being managed. This allows RefCell to provide runtime borrow checking and interior mutability without violating Rust's borrowing rules." - chatgpt 3.5
 * ^ apparely a lie because I've found this (and Gemini) that says no heap alloc: https://users.rust-lang.org/t/is-refcell-allocated-in-the-heap/9173/2
 * (true):"RefCell does not allocate, but it contains an additional "borrow state" indicator (one word in size) along with the data.

At runtime each borrow causes a modification/check of the refcount." -src: https://doc.rust-lang.org/1.30.0/book/first-edition/choosing-your-guarantees.html#cost-2
 */

unsafe impl<const N: usize, T> Sync for NoHeapAllocThreadLocal<N,T> {}

impl<const N: usize, T> Drop for NoHeapAllocThreadLocal<N, T> {
    fn drop(&mut self) {
        //TODO: can this be called concurrently? then we may have a problem.
        //TODO: can this be called by one thread while another one tries to set a value?!
        //i guess since our type aka Self is not Send, or it's a static(never dropped), then only one thread will be dropping it at most. But what if manual drop?
        //FIXME: ensure this is properly implemented! like, if one thread calls drop() and the other makes a new element, this isn't doing it right!
        let mut index=0;
        for elem in &mut self.values { //.iter().enumerate() {
        //for each in &mut self.values {
            //match self.after[index].compare_exchange(what_was,

            //We don't know what thread id was there before, to compare_exchange()
            let existing_tid=self.after[index].load(Ordering::Acquire);
            let was_set=Self::NO_THREAD_ID != existing_tid;
            //step1of3: we say the value in 'values' wasn't set.
            if was_set {
                match self.after[index].compare_exchange(existing_tid, Self::NO_THREAD_ID, Ordering::Release, Ordering::Acquire) {
                    Ok(prev_val) => {
                        assert_eq!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1)");
                        //fall thru
                    },
                    Err(prev_val) => {
                        assert_ne!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1)");
                        panic!("this shouldn't have been reached, some kind of race happened, like one thread called drop() and another called to make a new element for itself, but since the type isn't Send this means it's a static that a thread called drop() on manually while another thread was using it to make a new element...");
                    }
                }//match
            //} else {
                //wasn't set
                //this should've already existed then.
                //self.after[index].store(Self::NO_THREAD_ID, Ordering::Release);
            }//if

            //step2of3: we drop the previously set value
            //ok even if it wasn't set, the RefCell::new(None) still has to be dropped,
            //we've to drop the RefCell, else nothing else will drop it after.
            unsafe { ManuallyDrop::drop(elem) }
            //self.values[index]=unsafe { std::mem::zeroed() };
                //XXX: wait, why did I need to manually drop? oh it's because of the init: the const fn new() to be 'const fn' and make a new RefCell must not drop the prev. value which was just a mem::zeroed() RefCell not a real one.
                /* "Manually drops the contained value. This is exactly equivalent to calling ptr::drop_in_place with a pointer to the contained value. As such, unless the contained value is a packed struct, the destructor will be called in-place without moving the value, and thus can be used to safely drop pinned data.

If you have ownership of the value, you can use ManuallyDrop::into_inner instead.
Safety

This function runs the destructor of the contained value. Other than changes made by the destructor itself, the memory is left unchanged, and so as far as the compiler is concerned still holds a bit-pattern which is valid for the type T.

However, this “zombie” value should not be exposed to safe code, and this function should not be called more than once. To use a value after it’s been dropped, or drop a value multiple times, can cause Undefined Behavior (depending on what drop does). This is normally prevented by the type system, but users of ManuallyDrop must uphold those guarantees without assistance from the compiler."
src: https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html#method.drop */
//            if was_set {
//                // Calling this when the content is not yet fully initialized causes undefined behavior.
//                //unsafe { self.values[index].assume_init_drop(); }
//                //unsafe { ManuallyDrop::drop(&mut self.values[index]) }
//
//                //we've to drop the RefCell, else nothing else will after.
//                unsafe { ManuallyDrop::drop(elem) }
//                //unsafe { ManuallyDrop::drop(each) }
//            }//if
            //step3of3
            //self.before[index].store(Self::NO_THREAD_ID, Ordering::Release);
            match self.before[index].compare_exchange(existing_tid,Self::NO_THREAD_ID, Ordering::Release, Ordering::Acquire) {
                    Ok(prev_val) => {
                        assert_eq!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1)");
                        //fall thru
                    },
                    Err(prev_val) => {
                        assert_ne!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1)");
                        panic!("this shouldn't have been reached, inconsistency detected there should've been thread id '{}' stored at this index '{}' but it was '{}' instead, OR (todo: check if this is possible here:) some kind of race happened, like one thread called drop() and another called to make a new element for itself, but since the type isn't Send this means it's a static that a thread called drop() on manually while another thread was using it to make a new element...", existing_tid, index, prev_val);
                    }
            }//match
            index+=1;
        }//for
        //drop(self.after);
        //drop the array itself:
        //drop(self.values);//can't, but the array(s) will get dropped after this fn. is done.
        //drop(self.before);
    }//drop
}//impl Drop

//impl<const N: usize, T> Drop for NoHeapAllocThreadLocal<N, T> {
//    fn drop(&self) {
//        self.unset();
//    }
//}

//impl<const N: usize, T/*:std::fmt::Debug + PartialEq + Clone*/> NoHeapAllocThreadLocal<N, T> {
impl<const N: usize, T> NoHeapAllocThreadLocal<N, T> {
    const NO_THREAD_ID: u64 = 0; //aka unused slot/element
    const ARRAY_INITIALIZER_REPEAT_VALUE: AtomicU64 = AtomicU64::new(Self::NO_THREAD_ID);
    const SLEEP_TIME_BEFORE_RETRYING: std::time::Duration=std::time::Duration::from_millis(10);

    // this const fn gets computed at compile time.
    pub const fn new() -> Self {
        /* "This line initializes each element of the values array with uninitialized memory and then assumes that the uninitialized memory represents valid instances of RefCell<T>. This is done by calling assume_init()." */
        //let mut values: [MaybeUninit<RefCell<T>>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        /* "After this initialization, the values array contains elements that are zeroed out, but they are not valid instances of RefCell<T>. They are simply zeroed memory." */
        //let mut values: [MaybeUninit<RefCell<T>>; N] = unsafe { std::mem::zeroed() };
        let mut values:[ManuallyDrop<RefCell<Option<T>>>; N]=unsafe { std::mem::zeroed() };
        //let mut before= [Self::ARRAY_INITIALIZER_REPEAT_VALUE; N];

        // Use while loop for initialization
        let mut index = 0;
        while index < N {
            //before[index].store(1,Ordering::Relaxed);
            // Initialize each element with MaybeUninit::zeroed()
            //values[index] = MaybeUninit::uninit();

            // E0493: destructor of `RefCell<Option<T>>` cannot be evaluated at compile-time value is dropped here
            // problem is, it thinks it needs to drop() the prev value which is the mem::zeroed() one.
            // this is why we must use ManuallyDrop wrapper to thus tell it to not drop the prev. value.
            values[index]=ManuallyDrop::new(RefCell::new(None));
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

    //doneTODO: a fn that drops our value from all 3 fields!
    pub fn unset(&self) {
        if let Some((index,mut mut_ref_option_t))=self.maybe_get_mut_ref_if_set() {
            let my_tid=get_current_thread_id();
            let expected=my_tid;
            let new_value=Self::NO_THREAD_ID;
            //step1of3
            match self.after[index].compare_exchange(expected,new_value,Ordering::Release, Ordering::Acquire) {
                Ok(prev_val) => {
                    assert_eq!(prev_val, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1)");
                    //step2of3
                    //drop(self.values[index]);
                    //self.values[index]=MaybeUninit::uninit();
                    //self.values[index]=unsafe{std::mem::zeroed()};
                    //ok so we don't remove the RefCell, doh! we only remove the inner value, which will call drop() as needed, even tho the RefCell itself is wrapped into ManuallyDrop, it won't affect its inner held value.
                    if mut_ref_option_t.is_some() {
                        //TODO: does that .borrow_mut() end after the statement?
                        //assert!(self.values[index].borrow_mut().is_some());
                        *mut_ref_option_t=None;
                        //assert!(self.values[index].borrow_mut().is_none());
                    }
                    //step3of3
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
    pub fn maybe_get_mut_ref_if_set<'a>(&'a self) -> Option<(usize,RefMut<'a, Option<T>>)> {
        let our_current_tid: u64 = get_current_thread_id();
        assert_ne!(our_current_tid, Self::NO_THREAD_ID);
        for (index, atomic_value) in self.after.iter().enumerate() {
            //TODO: fix the orderings for atomics, if they're too strict.
            //For example: this here below shouldn't be Acquire because when it was stored it was Release for sure, so Acquire here is re-doing the same thing that was already done, besides, we're not reading the value unless it's our thread.
            let thread_id_at_index = atomic_value.load(Ordering::Acquire);
            if our_current_tid == thread_id_at_index {
                //let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
                //let mut_ref_to_value=unsafe { &mut *value_ptr };
                //let current_val=unsafe { self.values[index].assume_init_ref() };
                let current_val=self.values[index].borrow_mut();
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
    pub fn get_or_set<'a>(
        &'a self,
        //FIXME: ensure this value is the one that already exists?
        to_val:T,
        //thread_id: std::num::NonZeroU64,
        timeout: Duration,
    ) -> (bool,Option<RefMut<'a,Option<T>>>) {
        let start_time = std::time::Instant::now();
        //let new_value: u64 = current_thread_id.into();
        //let new_value: u64 = NoHeapAllocThreadLocal::<N, T>::get_current_thread_id();
        let new_value: u64 = get_current_thread_id();
        //if we have already allocated it, return early
        let mut index_of_first_empty: Option<usize> = None;//if any

        for (index, atomic_value) in self.after.iter().enumerate() {
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
//                    let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
//                    let mut_ref_to_value=unsafe { &mut *value_ptr };
                    let mut_ref_to_value=self.values[index].borrow_mut();
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
            std::thread::sleep(Self::SLEEP_TIME_BEFORE_RETRYING);
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
                //let atomic_value: &AtomicU64 = &self.before[index];
                //step1of3:
                match self.before[index].compare_exchange(
                    expected,
                    new_value,
                    Ordering::Release,
                    Ordering::Acquire,
                ) {
                    Ok(what_was) => {
                        //doneFIXME:there's a period of time between the compare_exchange call and until the value
                        //in values[index] is set(below) during which IF the thread now owning it recurses
                        //(or it somehow gets to read that value),
                        //then it can read the unset(or prev.) value!
                        //so maybe we need something like 2 atomics: one saying it's in progress
                        //and one saying its update(the assignment below) is finished;
                        //and if we encounter the in-progress one before setting it, it means we've recursed
                        //somehow(like this is used in panic handling code and we've panicked
                        //during the period of time aforementioned).
                        //TODO: don't actually panic anywhere? or maybe expect caller to catch_unwind() ? even so, it will call same panic handling code where if we're used we'd cause infinite recursion if we panic anywhere!
                        assert_eq!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(5)");
                        // it didn't exist before, we must add it
                        //self.values[index].write(to_val);
                        //self.values[index]=to_val;//cannot assign to `self.values[_]`, which is behind a `&` reference: `self` is a `&` reference, so the data it refers to
                        // Get a raw pointer to the element at the specified index
                        //let value_ptr= unsafe { &self.values[index] as *const T as *mut T };
                        // Assign the new value
//                        let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
//                        unsafe { *value_ptr = to_val; }
                        //unsafe { *value_ptr = to_val.clone(); }
                        //assert_eq!(self.values[index], to_val,"assignment failed, coded wrongly?");//need to restrict T to Debug, PartialEq and Clone for this assert to compile!
                        //assert_eq!(self.values[index], to_val,"assignment failed, coded wrongly?");
                        //assert!(
                        //    //can't access &val, it's moved!
                        //    Self::unsafe_compare_memory(&self.values[index], &to_val),
                        //    "assignment failed, coded wrongly?");
//                        let mut_ref_to_value=unsafe { &mut *value_ptr };

                        //step2of3
                        let mut mut_ref_to_value=self.values[index].borrow_mut();
                        *mut_ref_to_value=Some(to_val);
                        //step3of3
                        match self.after[index].compare_exchange(
                            expected,
                            new_value,
                            Ordering::Release,
                            Ordering::Acquire,
                            ) {
                            Ok(what_was) => {
                                assert_eq!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(6)");
                            },
                            Err(what_was) => {
                                assert_ne!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(7)");
                            },
                        }//match

                        //return (false,Some(unsafe { &mut *value_ptr }));//self.values[index]));
                        return (false,Some(mut_ref_to_value));//self.values[index]));
                    },//the Ok variant
                    Err(what_was) => {
                        assert_ne!(what_was, expected,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(8)");
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
            std::thread::sleep(Self::SLEEP_TIME_BEFORE_RETRYING);
            //TODO: put it to sleep until another thread releases any array element? or timeout is reached.
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
