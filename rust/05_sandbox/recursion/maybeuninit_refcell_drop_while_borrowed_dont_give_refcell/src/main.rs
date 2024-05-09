//#![feature(stmt_expr_attributes)]

use std::cell::{RefCell,Ref, RefMut};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::borrow::BorrowMut;
use std::borrow::Borrow;

struct Foo<const N:usize, T> {
    is_about_to_be_set:[AtomicBool;N],
    values:[MaybeUninit<RefCell<T>>;N],
    is_set:[AtomicBool;N],
}

impl<const N:usize, T> Foo<N,T> {
    const fn new() -> Self {
        let mut index=0;
        let mut values:[MaybeUninit<RefCell<T>>;N]= unsafe { std::mem::zeroed() };
        while index < N {
            values[index]=MaybeUninit::uninit();
            index+=1;
        }
        const CONST_INIT:AtomicBool=AtomicBool::new(false);
        Self {
            is_about_to_be_set:[CONST_INIT;N],
            values,
            is_set:[CONST_INIT;N],
        }
    }

    fn try_get_or_set<'a>(&'a self, value:T) -> Option<RefMut<'a,T>> {
        let index=N-1;
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
                    let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut RefCell<T>};
                    unsafe { *value_ptr=RefCell::new(value); }

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
        let ref_to_refcell=unsafe { self.values[index].assume_init_ref() };
        //return Some(unsafe { self.values[index].assume_init_ref() });
        return Some(ref_to_refcell.borrow_mut());
    }//fn

    fn drop_elem(&self) {
        let index=N-1;
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
                let value_ptr = unsafe { self.values.as_ptr().offset(index as isize) as *mut RefCell<T> };
                unsafe { value_ptr.drop_in_place() };
                //self.values[index] = MaybeUninit::uninit();
                let value_ptr_uninit = unsafe { self.values.as_ptr().offset(index as isize) as *mut MaybeUninit<RefCell<T>> };
                unsafe { *value_ptr_uninit=MaybeUninit::<RefCell<T>>::uninit(); }

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
    }
}//impl

#[derive(Debug)]
struct MyType(i32);

fn main() {
    println!("Hello, world!");
    let inst:Foo<10,MyType>=Foo::new();
    let my=MyType(100);
    let mut ref_to_refcell=inst.try_get_or_set(my).unwrap();
    println!("Got {:?}", ref_to_refcell);
    let foo_w=ref_to_refcell.borrow_mut();//no panic, which is ok!
    //let foo_r=ref_to_refcell.borrow();//no panic, which is ok!
    foo_w.0=1;//panic, which is good
    println!("direct access={:?}", foo_w);
    println!("Still got {:?}", ref_to_refcell);
    inst.drop_elem();//FIXME: I shouldn't be able to call this while still having outstanding borrows(ie. given out)
    println!("after dropStill got {:?}", ref_to_refcell);
    let my2=MyType(200);
    let ref_to_refcell2=inst.try_get_or_set(my2).unwrap();//panics, which is good but not enough
    println!("Got2 {:?}", ref_to_refcell2);

    //ref_to_refcell.borrow_mut();//this panics, so it's good
    //ref_to_refcell.borrow();//this doesn't panic, so it's bad
    //FIXME: well this is very bad! the refcell is still alive and sees a ref to a value of 0
    //so this memory location that's being referenced is now uninited!
    println!("Still got {:?}", ref_to_refcell);
    println!("Still Got2 {:?}", ref_to_refcell2);

    let my3=MyType(33);
    let ref_to_refcell3=inst.try_get_or_set(my3).unwrap();
    println!("Got3 {:?}", ref_to_refcell3);
    let my4=MyType(44);
    let ref_to_refcell4=inst.try_get_or_set(my4).unwrap();
    println!("Got4 {:?}", ref_to_refcell4);

    println!("Still got {:?}", ref_to_refcell);
    println!("Still Got2 {:?}", ref_to_refcell2);
}
