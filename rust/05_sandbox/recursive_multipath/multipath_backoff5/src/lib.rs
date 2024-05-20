//#![feature(thread_id_value)]


//pub mod std {
//    pub use crate as std;
//}
//#[cfg(not(feature = "use_std"))]
//mod time {
//    pub use crate::time::*;
//}
//mod std2 {
//    //pub use crate as std2;
//    pub use crate::*;
//    //pub use std::*;
//}

mod my_mod {
//use std::num::NonZeroU64;
//use crate::std2::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
//use std::mem::MaybeUninit;
use std::cell::{RefCell, RefMut};
//use std::cell::{Ref, RefCell, RefMut};
use core::mem::ManuallyDrop;

#[derive(Debug)]
pub struct NoHeapAllocThreadLocal<const MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS: usize, T> {
    //create 3 static arrays of size MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, preallocated, presumably on stack, but depends on type T doesn't it!
    //we need the 2 atomic arrays because a 'values' element is attached to one specific thread which shouldn't outlive it, or may even get manually dropped by caller, to allow for other thread(s) to use that spot
    //if 'before' is set, this means the value setting is in progress.
    before: [AtomicU64; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS],
    //values: [MaybeUninit<RefCell<T>>; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS],//nvmFIXME: ensure that i properly r/w this below
    values: [ManuallyDrop<RefCell<Option<T>>>; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS],
    //if 'after' is set, it means the value has already been set and is thus safe to read
    after: [AtomicU64; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS],
}

/* FALSE(chatgpt):" RefCell does use heap allocation internally to manage its borrow checking. It uses dynamic borrowing rules at runtime rather than static borrowing rules enforced by the Rust compiler. This dynamic borrowing is implemented through reference counting and interior mutability, which involves heap allocation for the reference count and the data being managed. This allows RefCell to provide runtime borrow checking and interior mutability without violating Rust's borrowing rules." - chatgpt 3.5
 * ^ apparely a lie because I've found this (and Gemini) that says no heap alloc: https://users.rust-lang.org/t/is-refcell-allocated-in-the-heap/9173/2
 * (true):"RefCell does not allocate, but it contains an additional "borrow state" indicator (one word in size) along with the data.

At runtime each borrow causes a modification/check of the refcount." -src: https://doc.rust-lang.org/1.30.0/book/first-edition/choosing-your-guarantees.html#cost-2
 */

//FIXME: how to dealloc dead threads, like those that exit()/abort()-ed, since the panic-ed ones would run drop() i guess?! Maybe need a way to find alive threads but there's none, unless directly checking pthreads' tids which are different and who knows if they would be reused unlike the rust tids. Maybe accept an arg with lifetime of the threadlocal in seconds, so if expired can be replaced, maybe if has no outstanding borrows? and so the acquire timeout must be at least that much plus a bit more. Actually the timeout shouldn't be higher, there might be other older threads about to expire so any timeout in acquiring should be valid, not limited to any minimum, besides other lifetimes could've been different anyway. Maybe lifetime should be instead time elapsed since last access but then we'd have to have a custom RefCell that would track access times then, or we consider access only those thru calls to our type, so asking for a mut borrow from the RefCell, but not using that RefCell afterwards. And only if no outstanding borrows would the lifetime expire, hmm but then what happens if RefCell remains in borrowed mode but thread's exited hmmmmm... could replace the RefCell with a new one aka new()? i wonder if dropping it would work in that case, since it's used with active borrow?! and actually I'm not sure I could replace it since it would require &mut self then! Now you may be thinking threads that exit/abort would affect the whole process, but in tests with WIP patch, i catch exit/abort and transform into panic, but point is, process won't exit, but the thread will, however in this particular case i guess since i exit the thread via panic anyway, it will execute drop?! which might just work and thus not need this whole thing hmm

//this is needed to can be shared between threads, and we internally ensure that's true.
unsafe impl<const MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS: usize, T> Sync for NoHeapAllocThreadLocal<MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS,T> {}

impl<const MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS: usize, T> Drop for NoHeapAllocThreadLocal<MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, T> {
    fn drop(&mut self) {
        //dontmatterTODO: can this be called concurrently? then we may have a problem. Apparently can't be.
        //dontmatterTODO: can this be called by one thread while another one tries to set a value?! apparently not.
        //i guess since our type aka Self is not Send, or it's a static(never dropped), then only one thread will be dropping it at most. But what if manual drop? So if it's inside an Arc then only one thread will be dropping it and only when for sure it's not used by others. Can't seem to can otherwise manually drop this which is great.
        //dontmatterFIXME: ensure this is properly implemented! like, if one thread calls drop() and the other makes a new element, this isn't doing it right!
        let mut index=0;
        for elem in &mut self.values { //.iter().enumerate() {
            //let i:i32=elem;//found mutable reference `&mut ManuallyDrop<RefCell<Option<T>>>`
        //for each in &mut self.values {
            //match self.after[index].compare_exchange(what_was,

            //We don't know what thread id was there before, to compare_exchange()
            let existing_tid=self.after[index].load(Ordering::Acquire);
            let was_set=Self::NO_THREAD_ID != existing_tid;
            //step1of3: we say the value in 'values' wasn't set.
            if was_set {
                match self.after[index].compare_exchange(existing_tid, Self::NO_THREAD_ID, Ordering::Release, Ordering::Acquire) {
                    Ok(prev_val) => {
                        assert_eq!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(12)");
                        //fall thru
                    },
                    Err(prev_val) => {
                        assert_ne!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(11)");
                        panic!("this shouldn't have been reached, some kind of race happened, like one thread called drop() and another called to make a new element for itself, but since the type isn't Send this means it's a static that a thread called drop() on manually while another thread was using it to make a new element...");
                    }
                }//match

                //else wasn't set
                //this should've already existed then.
                //self.after[index].store(Self::NO_THREAD_ID, Ordering::Release);
            }//if

            //step2of3: we drop the previously set value
            //ok even if it wasn't set, the RefCell::new(None) still has to be dropped,
            //we've to drop the RefCell, else nothing else will drop it after.
            println!("Dropping element at index '{}'", index);//FIXME: remove this
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
                        assert_eq!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(10)");
                        //fall thru
                    },
                    Err(prev_val) => {
                        assert_ne!(prev_val, existing_tid,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(9)");
                        panic!("this shouldn't have been reached, inconsistency detected there should've been thread id='{}' stored at this index='{}' but it was tid='{}' instead, OR (todo: check if this is possible here:) some kind of race happened, like one thread called drop() and another called to make a new element for itself, but since the type isn't Send this means it's a static that a thread called drop() on manually while another thread was using it to make a new element...", existing_tid, index, prev_val);
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

//impl<const MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS: usize, T> Drop for NoHeapAllocThreadLocal<MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, T> {
//    fn drop(&self) {
//        self.unset();
//    }
//}

//impl<const MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS: usize, T/*:std::fmt::Debug + PartialEq + Clone*/> NoHeapAllocThreadLocal<MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, T> {
impl<const MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS: usize, T> NoHeapAllocThreadLocal<MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, T> {
    const NO_THREAD_ID: u64 = 0; //aka unused slot/element
    const ARRAY_INITIALIZER_REPEAT_VALUE: AtomicU64 = AtomicU64::new(Self::NO_THREAD_ID);
    const SLEEP_TIME_BEFORE_RETRYING: std::time::Duration=std::time::Duration::from_millis(10);

    // this const fn gets computed at compile time.
    pub const fn new() -> Self {
        /* "This line initializes each element of the values array with uninitialized memory and then assumes that the uninitialized memory represents valid instances of RefCell<T>. This is done by calling assume_init()." */
        //let mut values: [MaybeUninit<RefCell<T>>; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS] = unsafe { MaybeUninit::uninit().assume_init() };
        /* "After this initialization, the values array contains elements that are zeroed out, but they are not valid instances of RefCell<T>. They are simply zeroed memory." */
        //let mut values: [MaybeUninit<RefCell<T>>; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS] = unsafe { std::mem::zeroed() };
        let mut values:[ManuallyDrop<RefCell<Option<T>>>; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS]=unsafe { std::mem::zeroed() };
        //let mut before= [Self::ARRAY_INITIALIZER_REPEAT_VALUE; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS];

        // Use while loop for initialization
        let mut index = 0;
        while index < MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS {
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
            before: [Self::ARRAY_INITIALIZER_REPEAT_VALUE; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS],
            //In uninitialized state initially, these will never be read before overwriting with valid T instance first! on a per element basis!
            //values: unsafe { std::mem::zeroed() },//good! //[None; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS], // that fails needed T:Copy
            values,
            after: [Self::ARRAY_INITIALIZER_REPEAT_VALUE; MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS],
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
    /// removes current thread's allocation of the thread local, thus allowing any future thread wanting a spot to take this one.
    /// spots are limited and only MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS number of threads can have occupied spots concurrently
    /// MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS is user setable when you declare your type.
    pub fn unset(&self) {
        //if let Some((index,mut mut_ref_option_t))=self.maybe_get_mut_ref_if_set() {
        if let Some(index)=self.maybe_get_index() {
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
                    let maybe:Result<RefMut<Option<T>>, std::cell::BorrowMutError>=self.values[index].try_borrow_mut();
                    if maybe.is_err() {
                        //XXX: it's recursing, so we undo, don't drop here in the nested call, it will drop in the original call, when it goes back to it.
                        match self.after[index].compare_exchange(new_value, expected,Ordering::Release, Ordering::Acquire) {
                            Ok(prev_val) => {
                                assert_eq!(prev_val, new_value,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(1_1)");
                            },
                            Err(prev_val) => {
                                assert_ne!(prev_val, new_value,"impossible, rust/atomics are broken on this platform, or we coded the logic of our program wrongly(4_1)");
                            },
                        }//match
                        return;//cuz we're recursing!
                    }
                    //if let Ok(mut_ref_option_t)=maybe {
                    //let mut mut_ref_option_t=self.values[index].borrow_mut();
                    let mut mut_ref_option_t=maybe.unwrap();//SAFETY: err variant was just handled above and it exited early
                    if mut_ref_option_t.is_some() {
                        //nvmTODO: does that .borrow_mut() end after the statement? well it's already borrowed from the 'if let'
                        //assert!(self.values[index].borrow_mut().is_some());
                        *mut_ref_option_t=None;//making this None drops the prev value, automagically.
                        //assert!(self.values[index].borrow_mut().is_none());
                    }
                    drop(mut_ref_option_t);
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
    }//fn

    //pub fn maybe_get_mut_ref_if_set<'a>(&'a self) -> Option<(usize,RefMut<'a, Option<T>>)> {
    pub fn maybe_get_mut_ref_if_set<'a>(&'a self) -> Option<RefMut<'a, Option<T>>> {
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
                //let current_val=self.values[index].borrow_mut();
                ////return Some((index,current_val));
                //return Some(current_val);
                let maybe:Result<RefMut<Option<T>>, std::cell::BorrowMutError>=self.values[index].try_borrow_mut();
                if let Ok(mut_ref_to_value)=maybe {
                    //let mut_ref_to_value:RefMut<Option<T>>=self.values[index].borrow_mut();
                    //if ensure_val {
                    //    if let Some(what_was)=mut_ref_to_value.as_mut() {
                    //        //well, we don't have to check at all, thus we won't have to require T traits!
                    //        //if *what_was != to_val { //binary operation `!=` cannot be applied to type `T`: T
                    //        *what_was=to_val;
                    //        //}
                    //    }
                    //}
                    //let current_val=&mut self.values[index];
                    //return (true, Some(current_val));
                    return Some(mut_ref_to_value);
                }//else XXX: fallthru to return None
            }
        } //for
        None
    }//fn

    fn maybe_get_index<'a>(&'a self) -> Option<usize> {
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
                //let current_val=self.values[index].borrow_mut();
                //return Some((index,current_val));
                return Some(index);
            }
        } //for
        None
    }//fn

    pub fn is_recursing(&self) -> bool {
        let our_tid_actual: u64 = get_current_thread_id();
        //if we have already allocated it, return early
        for (index, atomic_value) in self.after.iter().enumerate() {
            let thread_id_at_index = atomic_value.load(Ordering::Acquire);
            if thread_id_at_index == our_tid_actual {
                    //found one of ours, should be only one anyway!
                    let maybe:Result<RefMut<Option<T>>, std::cell::BorrowMutError>=self.values[index].try_borrow_mut();
                    //if can't borrow, then it's recursing, since no other thread could've borrowed (well, if our logic is respected!)
                    return maybe.is_err();
            }
        }
        return false;
    }//fn

    /// returns true if it was already set(and thus we just found it again)
    /// returns false if it wasn't already set, and either we found a spot for it or we didn't.
    /// If no slots are available, retry until timeout in which case returns None,
    /// if success returns a mutable ref to the existing value whether or not it was  just set
    /// Since the value is supposed to be accessible only on current thread, it's not protected or
    /// wrapped into some kind of sync. primitive, so you've direct mutability to it.
    pub fn get_or_set<'a>(
        &'a self,
        //doneFIXME: ensure this value is the one that already exists?
        to_val:T,//TODO: this should probably be a closure so it's not evaluated on each call!
        //thread_id: std::num::NonZeroU64,
        timeout: Duration,
        //ensure_val:bool,
    ) -> (bool,Option<RefMut<'a,Option<T>>>) {
        let start_time = std::time::Instant::now();
        //let new_value: u64 = current_thread_id.into();
        //let new_value: u64 = NoHeapAllocThreadLocal::<MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, T>::get_current_thread_id();
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
                    //TODO: use try_borrow_mut() here:
                    let maybe:Result<RefMut<Option<T>>, std::cell::BorrowMutError>=self.values[index].try_borrow_mut();
                    if let Ok(mut_ref_to_value)=maybe {
                        //let mut_ref_to_value:RefMut<Option<T>>=self.values[index].borrow_mut();
                        //if ensure_val {
                        //    if let Some(what_was)=mut_ref_to_value.as_mut() {
                        //        //well, we don't have to check at all, thus we won't have to require T traits!
                        //        //if *what_was != to_val { //binary operation `!=` cannot be applied to type `T`: T
                        //        *what_was=to_val;
                        //        //}
                        //    }
                        //}
                        //let current_val=&mut self.values[index];
                        //return (true, Some(current_val));
                        return (true, Some(mut_ref_to_value));
                    } else {
                        return (false,None); //FIXME: maybe Result?
                    }
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
        assert!(index_of_first_empty < MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS, "we coded it badly");
        //const expected:u64 = Self::NO_THREAD_ID; //XXX: this compile errors! can't use generic parameters from outer item: use of generic parameter from outer item
        let expected: u64 = Self::NO_THREAD_ID;//but this works, odd.
        loop {
            //for (index, atomic_value) in self.data.iter().enumerate() {
            for index in index_of_first_empty..MAX_CONCURRENTLY_USING_THREADS_AKA_SPOTS {
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
                        //FIXME: especially don't panic on borrows here internally, but return a result?
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

//    fn overwrite_msb_u64_with_i32(u64_value: u64, i32_value: i32) -> u64 {
//        // Shift the i32 value to the left by 32 bits to align it with the MSBs of the u64
//        let u64_value_to_insert:u64 = i32_value as u64;
//        let shifted_u64_value_to_insert:u64 = u64_value_to_insert << 32;
//
//        // Clear the 32 MSBs of the u64 value
//        let cleared_u64_value:u64 = u64_value & ((1u64 << 32) - 1);
//
//        // Combine the cleared u64 value with the shifted u64 value to insert using bitwise OR
//        cleared_u64_value | shifted_u64_value_to_insert
//    }

    fn xor_msb_u64_with_reversed_i32(u64_value: u64, i32_value: i32) -> u64 {
        let reversed_i32: u64= i32_value.reverse_bits() as u64;
        let shifted: u64 = reversed_i32 << 32;
        u64_value ^ shifted
    }

    pub fn get_current_thread_id() -> u64 {
        //itusedtoTODO: here's a question, does this alloc on heap anything, internally?! because that'd be bad.
        //XXX: FAIL std::thread::current() is allocating because uses Arc at some point! here: https://github.com/rust-lang/rust/blob/e8ada6ab253b510ac88edda131021d9878f2984f/library/std/src/thread/mod.rs#L1321-L1349
        //let current_thread_id:NonZeroU64 = std::thread::current().id().as_u64();
        //let current_thread_id:u64=current_thread_id.get();
        #[allow(non_camel_case_types)]
        pub type pid_t = i32;
        #[allow(non_camel_case_types)]
        pub type c_ulong = u64;
        #[allow(non_camel_case_types)]
        pub type pthread_t = c_ulong;
        extern "C" {
            fn pthread_self() -> pthread_t;
            pub fn gettid() -> pid_t;
        }
        //FIXME: pthread_self(or well pthread_create really) doesn't guarantee thread id is unique during the process' lifetime, only during the thread's lifetime.
        let current_thread_id: pthread_t= unsafe { pthread_self() };
        assert!(current_thread_id > 0,"impossible");//XXX: is it still impossible tho?
        let current_tid:pid_t = unsafe { gettid() };
        assert!(current_tid > 0,"impossible");//XXX: is it impossible tho?
        //let mixed:u64 = overwrite_msb_u64_with_i32(current_thread_id, current_tid);
        let mixed:u64 = xor_msb_u64_with_reversed_i32(current_thread_id, current_tid);
        //TODO: keep track of all seen 'mixed', and warn if reuse, but how do we know?! this isn't called only once!
        //TODO: maybe don't mix but make an AtomicU128 type, might require some thinking, but prolly with 2 atomics as guards for before/after.
        //eprintln!("!!! thr={:064b}",current_thread_id);//seems to use lots of bits
        //eprintln!("!!! tid={:064b}",current_tid);//seems to be counter of threads started, starting from 0 since uptime
        //eprintln!("!!! mix={:064b}",mixed);//overwrites some of the pthread_self bits which are very likely used!
        //hmm, first 16-17 msb bits of pthread_self seem 0
        //FIXME: probably need something better than this, to ensure the id isn't reused, altho i'm confident it won't on my current linux gentoo x64.
        return mixed;
    }

}//mod

pub use my_mod::NoHeapAllocThreadLocal;
pub use my_mod::get_current_thread_id;

pub mod my_mod2 {
use std::cell::RefCell;
use std::fmt;

// Helper struct to decrement location's in-use counter on Drop
#[derive(Debug)]
pub struct RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    //this bool is only used to hold the return bool from the macro call.
    //so doesn't have to be part of this struct actually.
    //and is thus only updated due to the call, not afterwards.
    pub is_recursing: bool,

    //this location is used to know which location to unvisit when going out of scope!
    //this is the tracker that we use to update every time we enter/exit the zone
    location_tracker: T,
}

impl<T> fmt::Display for RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "{} {:?}", self.is_recursing, self.location_tracker)
        let type_name = std::any::type_name::<T>();
        //leaveitFIXME: is the _or() even needed?! yes, else it can be None if the separator doesn't exist
        let root_type = type_name.split("<").next().unwrap_or(type_name).split("::").last().unwrap_or(type_name);
        write!(f, "{} {}", self.is_recursing, root_type)
    }
}

pub trait UnvisitTrait {
    fn unvisit(&self);
}



impl UnvisitTrait for RecursionDetectionZoneGuard<&'static HeapAllocsThreadLocalForThisZone> {

    //mustn't call this manually
    fn unvisit(&self) {
        //unvisits
        //if self.can_heap_alloc {
        //TODO: try_with() "This function will still panic!() if the key is uninitialized and the key’s initializer panics."
        //TODO: handle error cases, ie. what if can't borrow, or stuff.
        let res=self.location_tracker.try_with(|refcell| {
            //let i:i32=refcell;//found `&RefCell<Option<...>>`
            let mut res_borrow=refcell.try_borrow_mut();
            if let Ok(ref mut ref_mut_location) = res_borrow {
                //let i:i32=ref_mut_location;//found `RefMut<'_, Option<...>>`
                //println!("!{}",self.location);
                if let Some(sal) = ref_mut_location.as_mut() {
                    //let i:i32=counter;//&mut StuffAboutLocation
                    if *sal> 0 {
                        *sal -= 1;
                    } else {
                        //TODO: return Result<> ? but then rename to try_unvisit() ?
                        eprintln!("!!! counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!(1)", sal);
                    }
                } else {
                    eprintln!("!!! unvisiting found None as the S.A.L., this is pretty bad as it means inconsistency in coding the logic(1)");
                }
            } else {
                eprintln!("!!! unvisiting errored, couldn't borrow(recursion?), this is pretty bad as it means inconsistency in tracking, error='{:?}'(1)",res_borrow);
            }
            drop(res_borrow);//now can be dropped
        });
        if let Err(err)=res {
            //TODO: this is pretty bad, maybe somehow set the is_recursing bool to some default ?
            eprintln!("!!! unvisiting errored, this is pretty bad as it means inconsistency in tracking, error='{}'",err);
        }
    }
}//impl

/// Define the maximum number of threads that are concurrently supported in the same zone,
/// before putting new ones on wait(with a timeout) until the prev. ones exit the zone.
const MAX_NUM_THREADS_AT_ONCE: usize = 10;
//doneTODO: need to rename this type:
pub type NoHeapAllocsThreadLocalForThisZone=super::my_mod::NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE, StuffAboutLocation>;
//type PrivateStuffAboutLocation = super::my_mod2::StuffAboutLocation;
//pub type NoHeapAllocsThreadLocalForThisZone=super::my_mod::NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE, PrivateStuffAboutLocation>;
////failed to wrap it and then delegate:
//pub struct NoHeapAllocsThreadLocalForThisZone(super::my_mod::NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE, StuffAboutLocation>);
//// "If you want to automatically delegate methods of the inner type, you can implement the Deref trait for your wrapper struct. This allows you to access methods of the inner type directly on an instance of your wrapper struct. Here's how you can do it:" - chatgpt 3.5
//impl std::ops::Deref for NoHeapAllocsThreadLocalForThisZone {
//    type Target = super::my_mod::NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE, StuffAboutLocation>;
////| ^^^^^^^^^^^ can't leak private type
////...
////635 | struct StuffAboutLocation {
////    | ------------------------- `StuffAboutLocation` declared as private
////
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//// Implement methods for the public struct, if needed
//impl NoHeapAllocsThreadLocalForThisZone {
//    pub const fn new() -> Self {
//        // You can construct the inner type here
//        let inner = super::my_mod::NoHeapAllocThreadLocal::<MAX_NUM_THREADS_AT_ONCE, StuffAboutLocation>::new();
//        NoHeapAllocsThreadLocalForThisZone(inner)
//    }
//}
impl UnvisitTrait for RecursionDetectionZoneGuard<&NoHeapAllocsThreadLocalForThisZone> {

    //mustn't call this manually
    fn unvisit(&self) {
        //println!("unvisiting self={:?}",self);
        let mut can_dispose:bool=false;
        {
            let mut loc=self.location_tracker.maybe_get_mut_ref_if_set();
            //let i:i32=loc;//`Option<RefMut<'_, Option<LocationWithCounter>>>`
            if let Some(ref mut refmut)=loc {
                //let i:i32=refmut;//`RefMut<'_, Option<LocationWithCounter>>`
                //so it's already being used
                if let Some(sal)=refmut.as_mut() {
                    //let i:i32=sal;//`&mut LocationWithCounter`
                    //let i:i32=sal.counter;//found `StuffAboutLocation`
                    if *sal > 0 {
                        *sal -=1;
                        if *sal == 0 {
                            can_dispose=true;
                        }
                    } else {
                        eprintln!("counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!(2)", sal);
                    }
                } else {
                    eprintln!("!!! unvisiting found None as the S.A.L., this is pretty bad as it means inconsistency in coding the logic(2)");
                }
            } else {
                //it's not used, can drop it:
                can_dispose=true;
            }
            drop(loc);//E0382: use of partially moved value: `loc`
        }//so, is 'loc' dropped here or what? yeFIXME
        if can_dispose {
            //yesTODO: test to see if this is ever called!
            //eprintln!("disposing current tid from noallocthreadlocal {:?}",self.location);
            self.location_tracker.unset();
            //eprintln!("disposed current tid from noallocthreadlocal {:?}",self.location);
        }
    }
}

impl<T> RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    #[allow(dead_code)]
    #[inline(always)]
    pub fn done(self) {
        self.drop();
    }

    #[inline(always)]
    pub fn drop(self) {
        drop(self);
    }

    #[inline(always)]
    pub fn end_zone_aka_drop(self) {
        self.drop();
    }

    pub fn new(is_recursing: bool, location_tracker:T) -> Self {
        Self {
            is_recursing,
            location_tracker,
        }
    }
}

impl<T> Drop for RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    fn drop(&mut self) {
        self.unvisit();
    }
}

/// not meant to be accessible by caller, but can't be helped due to the macro using it as a nested type in the static!
#[derive(Debug)]
pub struct StuffAboutLocation {
    //this is 1 or more while in the zone
    //if it's more than 1 it's currently recursing and recursion started from within the zone
    times_visited_currently: u64,

    //a 1 on this means normal execution
    //a 2+ means recursed this many times minus 1
    max_times_visited_ever: u64,
}

impl PartialEq<u64> for StuffAboutLocation {
    fn eq(&self, other: &u64) -> bool {
        self.times_visited_currently == *other
    }
}

//needed for comparisons like: self.counter > u64
impl PartialOrd<u64> for StuffAboutLocation {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.times_visited_currently.partial_cmp(other)
    }
}

impl fmt::Display for StuffAboutLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.times_visited_currently)
    }
}

impl std::ops::SubAssign<u64> for StuffAboutLocation {
    fn sub_assign(&mut self, rhs: u64) {
        self.times_visited_currently -= rhs;
        self.update_max();
    }
}


impl std::ops::AddAssign<u64> for StuffAboutLocation {
    fn add_assign(&mut self, rhs: u64) {
        self.times_visited_currently += rhs;
        self.update_max();
    }
}

impl StuffAboutLocation {
    //surebutnotinourTLimplFIXME: user can still init the struct with struct initializer syntax and set max to be less
    //than current(if current is >0), then u'd have to call update_max() from below!
    pub fn initial() -> StuffAboutLocation {
        return StuffAboutLocation { times_visited_currently:0, max_times_visited_ever:0 };
    }

    #[inline(always)]
    pub fn update_max(&mut self) {
        if self.times_visited_currently > self.max_times_visited_ever {
            self.max_times_visited_ever=self.times_visited_currently;
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn get_max_seen(&mut self) -> u64 {
        self.update_max();
        self.max_times_visited_ever
    }
}

//TL=for the thread_local declaration
pub type TLHeapAllocsThreadLocalForThisZone = RefCell<Option<StuffAboutLocation>>;
//This is for the reference(&) to what we've declared with thread_local
pub type HeapAllocsThreadLocalForThisZone = std::thread::LocalKey<TLHeapAllocsThreadLocalForThisZone>;
//ohwellTODO: get rid of thread_local!() macro call, and thus use only one type alias here! It won't work, still needs 2 types! so no use.
//cantTODO: actually don't need it to be a RefCell, since we're giving the whole static to the guard! but for the noalloc version we do. Still need RefCell wrapper with thread_local!() else I can't mutate the inner value because .try_with() gives me an immutable ref.

#[inline(always)]
fn got_value(ref_to_static: &NoHeapAllocsThreadLocalForThisZone, timeout: std::time::Duration) -> std::option::Option<bool> {
    if !ref_to_static.is_recursing() {
        let (was_already_set,mut sal_refmut)=ref_to_static.get_or_set(
            StuffAboutLocation::initial(),
            timeout,
        );
        if let std::option::Option::Some(ref mut sal)=sal_refmut {
            let sal=sal.as_mut().unwrap();
            std::assert!(*sal>=0);
            std::assert!(*sal>1);//FIXME: remove this temp., so this causes a 3-depth nested panic in alloc/dealloc/realloc of global allocator in main()
            let was_visited_before= *sal>0;
            *sal+=1;
            std::assert_eq!(was_visited_before, was_already_set, "these two should be in sync");
            //drop(sal);//it's a ref
            std::mem::drop(sal_refmut);
            Some(was_visited_before)
        } else {
            std::assert!(sal_refmut.is_none());
            std::mem::drop(sal_refmut);
            //ie. timeout
            None
        }
    } else {
        //if recursing well...
        None
    }
}

pub fn macro_helper1(ref_to_static: &NoHeapAllocsThreadLocalForThisZone, timeout: std::time::Duration) -> std::option::Option<RecursionDetectionZoneGuard<&NoHeapAllocsThreadLocalForThisZone>> {
    if let Some(was_visited_before)=got_value(ref_to_static, timeout) {
        let guard: RecursionDetectionZoneGuard<&NoHeapAllocsThreadLocalForThisZone> = RecursionDetectionZoneGuard::new(was_visited_before, &ref_to_static);
        std::option::Option::Some(guard) // Return the guard instance
    } else {
        //timed out or is recursing
        std::option::Option::None
    }
}

pub fn macro_helper2(ref_to_static: &NoHeapAllocsThreadLocalForThisZone, timeout: std::time::Duration, default_value_on_timeout:bool) -> RecursionDetectionZoneGuard<&NoHeapAllocsThreadLocalForThisZone> {
    let was_visited_before=if let Some(was_visited_before)=got_value(ref_to_static, timeout) {
        was_visited_before
    } else {
        //timeout or recursing! TODO: rename var to include recursion possibility
        default_value_on_timeout
    };
    return RecursionDetectionZoneGuard::<&NoHeapAllocsThreadLocalForThisZone>::new(was_visited_before, &ref_to_static);
}

//TODO: find out why this needs the 'static lifetime but our noheap type doesn't hmm..
pub fn macro_helper3(ref_to_static:&'static HeapAllocsThreadLocalForThisZone) -> RecursionDetectionZoneGuard<&'static HeapAllocsThreadLocalForThisZone> {
        let was_visited_before=ref_to_static.try_with(|refcell| {
            let mut ref_mut=refcell.borrow_mut();
            //let i:i32=ref_mut;//found `RefMut<'_, Option<...>>`
            if ref_mut.is_none() {
                //first time init:
                *ref_mut=std::option::Option::Some(StuffAboutLocation::initial());
            }
            assert!(ref_mut.is_some(),"code logic is wrong");
            let sal=ref_mut.as_mut().unwrap();
            //let i:i32=sal;//found `&mut StuffAboutLocation`
            *sal += 1;
            *sal > 1 // Return true if is_recursing (counter > 1)
            //assert_eq!(ref_mut.as_mut().unwrap().counter,1,"developer coded it wrongly");
        }).unwrap_or(true);
        //XXX: so we say is_recursing=true if failed to acquire lock which means it's likely due to recursion
        //while inside the try_with() closure, ie. recursion_detection_zone!(start) is called again while inside the
        //above try_with(), how? maybe this is used inside the rust std panic handling code and it
        //panicked inside the try_with() somehow!
        //doneTODO: return the bool and the Option<LocationInSourceCode> so that it can be *counter-=1 later when
        //done; i don't think we can do this on Drop because catch_unwind() would trigger it, hmm,
        //maybe this is a good thing? didn't think this thru.
        let guard:RecursionDetectionZoneGuard<&'static HeapAllocsThreadLocalForThisZone> = RecursionDetectionZoneGuard::new(was_visited_before, &ref_to_static);
        //{
        //    is_recursing: was_visited_before,
        //    location_tracker: &A_STATIC_FOR_THIS_CALL_LOCATION,
        //    //nogoodTODO: maybe don't give ref to the static, but a ref to the inner instead? which means, we'd need the RefCell::borrow_mut() here. Well actually giving a refcell mut ref here would prevent recursive call from modifying the inner because it's already mut borrowed!

        //};
        guard // Return the guard instance
}


// Macro to mark a location as is_recursing
/// aka "am i recursing due to this"
/// or better: "if I'm recursing, has this been done/encountered before?"
/// if I'm not recusing then this is false, even if used in a loop(due to Drop happening after each
/// loop)
// so it's more like, have I seen this in this session,
// and session is the current block. hmm..
/// aka 'recursion guard' or 'recurse guard', in THIS thread!
/// am I recursing in this zone?
/// or has this zone been used as launchpad for this recursion(if guard.is_recursing is true)
//okTODO: should I rename this to something more obvious of what's happening?
#[macro_export]
//#[track_caller] //unused_attributes: `#[track_caller]` is ignored on struct fields, match arms and macro defs
macro_rules! recursion_detection_zone {
    (begin) => {
        $crate::recursion_detection_zone!(start)
    };
    (new) => {
        $crate::recursion_detection_zone!(start)
    };
    (mark_beginning) => {
        $crate::recursion_detection_zone!(start)
    };
    (mark beginning) => {
        $crate::recursion_detection_zone!(start)
    };
    (start) => {{ //double curlies, all the way! else 'let' won't work; single {} expects expression,
             //double {{}} is like a normal {} that returns an expression even if it's () unit.


        // Thread-local storage for the current zone/call-location of this macro
        std::thread_local! {
            //XXX: thread_local itself does heap alloc internally(because pthread_key_create does alloc)!
            //it's gonna be a different static for each location where this macro is called; seems it has same name but internally it's mangled and global, however only visible here.
            static A_STATIC_FOR_THIS_CALL_LOCATION: $crate::my_mod2::TLHeapAllocsThreadLocalForThisZone = $crate::my_mod2::TLHeapAllocsThreadLocalForThisZone::new(std::option::Option::None);
            //doneTODO: keep a max times visited?
        }
        $crate::my_mod2::macro_helper3(&A_STATIC_FOR_THIS_CALL_LOCATION) //returns a guard instance!
    }};
// -----------
    (noheapalloc start, $timeout:expr, $default_value_on_timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout, $default_value_on_timeout)
    };
    (no_heap_alloc start, $timeout:expr, $default_value_on_timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout, $default_value_on_timeout)
    };
    (noalloc begin, $timeout:expr, $default_value_on_timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout, $default_value_on_timeout)
    };
    (noalloc new, $timeout:expr, $default_value_on_timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout, $default_value_on_timeout)
    };
    (noalloc mark_beginning, $timeout:expr, $default_value_on_timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout, $default_value_on_timeout)
    };
    (noalloc mark beginning, $timeout:expr, $default_value_on_timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout, $default_value_on_timeout)
    };
    //doneTODO: code is duplicated in the following 2 macro branches. This is very bad for keeping things in sync when modifying the code in one of them.
    (noalloc start, $timeout:expr, $default_value_on_timeout:expr) => {{
        static LOCATION_VAR: $crate::my_mod2::NoHeapAllocsThreadLocalForThisZone = $crate::my_mod2::NoHeapAllocsThreadLocalForThisZone::new();
        //fn assert_bool(_: bool) {}
        //assert_bool($default_value_on_timeout);
        $crate::my_mod2::macro_helper2(&LOCATION_VAR, $timeout, $default_value_on_timeout)
    }};
// -----------
    (noheapalloc start, $timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout)
    };
    (no_heap_alloc start, $timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout)
    };
    (noalloc begin, $timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout)
    };
    (noalloc new, $timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout)
    };
    (noalloc mark_beginning, $timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout)
    };
    (noalloc mark beginning, $timeout:expr) => {
        $crate::recursion_detection_zone!(noalloc start, $timeout)
    };
    (noalloc start, $timeout:expr) => {{
        //okTODO: code is duplicated in the 2 macro branches (the one above and the one below). This is very bad for keeping things in sync when modifying the code in one of them.
        //doneFIXME: well now need this to be thread_local but without allocating, soo... fixed sized
        //array which would represent only the currently visiting(counter>0) location paired with
        //thread id number, as one of the elements of the array.
        //and have new threads wait if it's full, but with a timeout(5sec?) and if timeout then
        //return what? true that it's recursing or false that it's now? allow user to provide value
        //to be returned if timeout?
        //use no_heap_allocations_thread_local::NoHeapAllocThreadLocal;
        //static LOCATION_VAR: NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE,LocationWithCounter> = NoHeapAllocThreadLocal::new();
        static LOCATION_VAR: $crate::my_mod2::NoHeapAllocsThreadLocalForThisZone = $crate::my_mod2::NoHeapAllocsThreadLocalForThisZone::new();
        //okTODO: the static must remain in the macro, but the rest could be inside a function

        // "Note that, because $crate refers to the current crate, it must be used with a fully qualified module path when referring to non-macro items:" src: https://doc.rust-lang.org/reference/macros-by-example.html#hygiene
        $crate::my_mod2::macro_helper1(&LOCATION_VAR, $timeout)
    }};
    //};
// -----------
    (end_zone, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (end zone, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (end_zone_aka_drop, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (done, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (drop, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (finish, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (mark end, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (mark_end, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (mark_ending, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (mark ending, $guard:ident) => {
        $crate::recursion_detection_zone!(end, $guard)
    };
    (end, $guard:ident) => {
        $guard.end_zone_aka_drop();
    };
}//macro

//pub(super) use been_here;
//pub(crate) use been_here;
//pub(self) use been_here;


}//mod
//pub use my_mod2::TLHeapAllocsThreadLocalForThisZone;
//pub use my_mod2::HeapAllocsThreadLocalForThisZone;
//pub use my_mod2::NoHeapAllocsThreadLocalForThisZone;
//pub use my_mod2::StuffAboutLocation;//semidoneFIXME: shouldn't be pub; well it must be because the type alias is pub and it includes it innerly. So it's pub but inside the my_mod2 module, rather than in the crate root!
pub use my_mod2::RecursionDetectionZoneGuard;//this might wanna be used for explicit typing. This makes it pub in crate root.
//doneTODO: must find a better way here perhaps?
//pub use my_mod2::recursion_detection_zone;//unresolved import `my_mod2::recursion_detection_zone`: no `recursion_detection_zone` in `my_mod2`

//pub use self::recursion_detection_zone;
//    = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
//help: a macro with this name exists at the root of the crate
//    |
//914 | pub use ::recursion_detection_zone;
//    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~

