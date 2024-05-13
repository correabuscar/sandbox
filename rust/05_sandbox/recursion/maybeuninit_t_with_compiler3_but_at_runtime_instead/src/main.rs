//#![feature(stmt_expr_attributes)]

use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};
//use std::borrow::BorrowMut;
//use std::borrow::Borrow;
use std::mem::ManuallyDrop;
use std::cell::{Ref, RefCell, RefMut};


#[derive(Debug)]
struct Foo<const N:usize, T> {
    //true if it's about to be 'used'
    is_about_to_be_set:[AtomicBool;N],
    values:[ManuallyDrop<RefCell<Option<T>>>;N],
    //values:[MaybeUninit<RefCell<Option<T>>>;N],
    //true if it's 'used'
    is_set:[AtomicBool;N],
}

/// we pinky-promise that we're making sure internally that it's thread safe
// needed to can be used as a type for an immutable 'static' !
unsafe impl<const N: usize, T> Sync for Foo<N, T> {}

impl<const N:usize, T> Foo<N,T> {
    const fn new() -> Self {
        let mut index=0;
        let mut values:[ManuallyDrop<RefCell<Option<T>>>;N]= unsafe { std::mem::zeroed() };
        while index < N {
            //values[index]=MaybeUninit::uninit();
            values[index]=ManuallyDrop::new(RefCell::new(None));
            index+=1;
        }
        const CONST_INIT:AtomicBool=AtomicBool::new(false);
        Self {
            is_about_to_be_set:[CONST_INIT;N],
            values,
            is_set:[CONST_INIT;N],
        }
    }

    fn get_index_for_current_thread(&self) -> Option<usize> {
        let index=N-1;//XXX: hardcoded, but ideally it'd find a spot for the current thread to use, or return the prev. found one.
        #[allow(unused_comparisons)]
        {
            assert!(index>=0);
        }
        assert!(index<N);
        return Some(index);
    }

    //fn try_get_or_set<'a>(&'a self, value:T) -> Option<RefMut<'a,T>> {
    fn try_get_or_set<'a>(&'a self, value:T) -> Option<RefMut<'a,Option<T>>> {
        let index=self.get_index_for_current_thread();
        if index.is_none() {
            return None;
        }
        let index=index.unwrap();//safe
        #[allow(unused_comparisons)]
        {
            assert!(index>=0);
        }
        assert!(index<N);
        if !self.is_set[index].load(Ordering::Acquire) {
            //well it's not set yet.
            //let's mark it in progress first, step1of3
            match self.is_about_to_be_set[index].compare_exchange(false, true, Ordering::Release, Ordering::Acquire) {
                Ok(prev_val) => {
                    assert_eq!(false, prev_val);
                    //now let's set it, step2of3:

                    //this seems to work without needing &mut self:
                    //XXX: it should be ok to mutate this due to self.is_set protecting it from concurrent mutation!
                    //FIXME: the only quesion is, am I doing this mutation right?!
                    //let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
                    //unsafe { *value_ptr=value; }
                    *self.values[index].borrow_mut()=Some(value);

                    //now let's mark it as set step3of3:
                    match self.is_set[index].compare_exchange(false, true, Ordering::Release, Ordering::Acquire) {
                        Ok(prev_val) => {
                            assert_eq!(false, prev_val);
                            //we've set is_set[index] to 'true', so fall thru
                        },
                        Err(prev_val) => {
                            assert_eq!(true, prev_val);
                            panic!("We coded something wrongly, because if we're here, nothing else could've ever set this first, even concurrently");
                        }
                    }//match
                },
                Err(prev_val) => {
                    assert_eq!(true, prev_val);
                    //another thread got to this first, and it's in progress setting it!
                    return None;
                },
            }//match
        } // else
        //let refcell=unsafe { self.values[index].assume_init_ref() };
        //let ref_to_mut_option=unsafe { self.values[index].assume_init_ref() };
        //let ref_to_mut_option=unsafe { self.values[index].assume_init_ref() };
        //let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T};
        //let a_ref=unsafe { &mut *value_ptr };
        //let a_ref=&self.values[index];
        //let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut RefCell<T>};
        //return Some(unsafe { self.values[index].assume_init_ref() });
        //TODO: so are we gonna tell if there was a prev. value and if it was equal or not to the one we tried to set? maybe rename the function to ensure_this_is_set_and_return_it() heh.
        return Some(self.values[index].borrow_mut());
    }//fn

    fn try_drop_elem(&self) -> Result<(), &'static str> {
        let index=self.get_index_for_current_thread();
        if index.is_none() {
            return Err("not yet allocated, no space");
        }
        let index=index.unwrap();//safe
        #[allow(unused_comparisons)]
        {
            assert!(index>=0);
        }
        assert!(index<N);
        if !self.is_set[index].load(Ordering::Acquire) {
            panic!("Bad call");
        }
        //ok so it's set here:
        //step1of3:
        match self.is_set[index].compare_exchange(true, false, Ordering::Release, Ordering::Acquire) {
            Ok(prev_val) => {
                assert_eq!(true, prev_val);
                //step2of3:

                //let old_refcell=unsafe { self.values[index].assume_init_mut() };//can't needs &mut self
                //drop(old_refcell);
//                let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut T };
//                unsafe { value_ptr.drop_in_place() };
                //self.values[index] = MaybeUninit::uninit();
//                let value_ptr_uninit = unsafe { self.values.as_ptr().offset(index as isize) as *mut MaybeUninit<T> };
//                unsafe { *value_ptr_uninit=MaybeUninit::<T>::uninit(); }

                //unless it's already borrowed by caller, then this panics:
                *self.values[index].borrow_mut()=None;//this drops the old value if it was Some(T) automagically

                //step3of3:
                match self.is_about_to_be_set[index].compare_exchange(true, false, Ordering::Release, Ordering::Acquire) {
                    Ok(prev_val) => {
                        assert_eq!(true, prev_val);
                        //all good, fallthru
                    },
                    Err(prev_val) => {
                        assert_eq!(false, prev_val);
                        panic!("We coded something wrongly, because if we're here, nothing else could've ever set this first, even concurrently.(2)");
                    },
                }//match

            },
            Err(prev_val) => {
                assert_eq!(false, prev_val);
                panic!("No drop, some other thread dropped it first!");
            }
        }//match
        Ok(())
    }
}//impl

#[derive(Debug)]
struct MyType(i32);

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping {:?}",self);
    }
}

static INST:Foo<10,MyType>=Foo::new();

fn main() -> Result<(),&'static str> {
    println!("Hello, world!");
    //let mut INST:Foo<10,MyType>=Foo::new();
    let my=MyType(100);
    //let a_ref:&Option<MyType>;
    let a_ref:&MyType;
    {
        let ref_to_mut_mytype=INST.try_get_or_set(my).unwrap();
        println!("Got {:?}", ref_to_mut_mytype);
        //let foo_w=ref_to_mut_mytype.borrow();//no panic, which is ok!
                                              //let foo_r=ref_to_mut_option.borrow();//no panic, which is ok!
                                              //foo_w.0=1;//panic, which is good
        println!("direct access={:?}", *ref_to_mut_mytype);
        println!("Still got {:?}", ref_to_mut_mytype);
        //INST.try_drop_elem()?;//XXX: this panics here due to already borrowed, which is great! at runtime.
        //*ref_to_mut_mytype=None;
        println!("direct access={:?}", *ref_to_mut_mytype);
        println!("Still got {:?}", ref_to_mut_mytype);
        //a_ref=&ref_to_mut_mytype.unwrap();//can't move out.
        //drop(INST);
    }
    //drop(ref_to_mut_option);
    INST.try_drop_elem()?;//XXX: doesn't panic at runtime due to no outstanding borrows. Good.
    //println!("after dropStill got {:?}", ref_to_mut_option);
    //println!("after dropStill got {:?}", a_ref);
    let my2=MyType(200);
    let mut ref_to_mut_option2=INST.try_get_or_set(my2).unwrap();//panics, which is good but not enough
    println!("Got2 {:?}", ref_to_mut_option2);
    *ref_to_mut_option2=None;

    //ref_to_mut_option.borrow_mut();//this panics, so it's good
    //ref_to_mut_option.borrow();//this doesn't panic, so it's bad
    //FIXME: well this is very bad! the refcell is still alive and sees a ref to a value of 0
    //so this memory location that's being referenced is now uninited!
    //println!("Still got {:?}", ref_to_mut_option);
    println!("Still Got2 {:?}", ref_to_mut_option2);
    println!("Foo={:?}", INST);

    let my3=MyType(33);
//    let ref_to_mut_option3=INST.try_get_or_set(my3).unwrap();
  //  println!("Got3 {:?}", ref_to_mut_option3);
    let my4=MyType(44);
//    let ref_to_mut_option4=INST.try_get_or_set(my4).unwrap();
 //   println!("Got4 {:?}", ref_to_mut_option4);

    //println!("Still got {:?}", ref_to_mut_option);
//    println!("Still Got2 {:?}", ref_to_mut_option2);
    Ok(())
}
