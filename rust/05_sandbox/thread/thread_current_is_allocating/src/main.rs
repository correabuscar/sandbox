#![feature(thread_id_value)]

use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicBool, Ordering};

thread_local! {
    static FOO_ALLOC:AtomicBool=AtomicBool::new(false);
    static FOO_DEALLOC:AtomicBool=AtomicBool::new(false);
    static FOO_REALLOC:AtomicBool=AtomicBool::new(false);
}

struct MyGlobalAllocator;
unsafe impl GlobalAlloc for MyGlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let _res=FOO_ALLOC.try_with(|s| {
            //let stdout_lock = std::io::stdout().lock();
            //use std::io::{self,Write};
            //let res=stdout_lock.try_lock();
            //if res.is_ok() {
            let res=s.compare_exchange(false,true, Ordering::Release, Ordering::Acquire);
            match res {
                Ok(prev) => {
                    std::eprintln!("Allocating {} bytes", layout.size());
                    let _=s.compare_exchange(true,false, Ordering::Release, Ordering::Acquire);
                }
                Err(prev) => {
                }
            }//match
            //}
        });

        let ptr = std::alloc::System.alloc(layout);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//        let _res=FOO_DEALLOC.try_with(|_| {
//            std::println!("Deallocating {} bytes", layout.size());
//        });
        let _res=FOO_DEALLOC.try_with(|s| {
            //let stdout_lock = std::io::stdout().lock();
            //use std::io::{self,Write};
            //let res=stdout_lock.try_lock();
            //if res.is_ok() {
            let res=s.compare_exchange(false,true, Ordering::Release, Ordering::Acquire);
            match res {
                Ok(prev) => {
                    std::eprintln!("DeAllocating {} bytes", layout.size());
                    let _=s.compare_exchange(true,false, Ordering::Release, Ordering::Acquire);
                }
                Err(prev) => {
                }
            }//match
            //}
        });
        std::alloc::System.dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
//        let _res=FOO_REALLOC.try_with(|_| {
//            std::println!("Reallocating {} bytes", layout.size());
//        });
        let _res=FOO_REALLOC.try_with(|s| {
            //let stdout_lock = std::io::stdout().lock();
            //use std::io::{self,Write};
            //let res=stdout_lock.try_lock();
            //if res.is_ok() {
            let res=s.compare_exchange(false,true, Ordering::Release, Ordering::Acquire);
            match res {
                Ok(prev) => {
                    std::eprintln!("ReAllocating {} bytes", layout.size());
                    let _=s.compare_exchange(true,false, Ordering::Release, Ordering::Acquire);
                }
                Err(prev) => {
                }
            }//match
            //}
        });
        let new_ptr = std::alloc::System.realloc(ptr, layout, new_size);
        new_ptr
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyGlobalAllocator = MyGlobalAllocator;

fn main() {
    println!("Hello, world!");
    //all prev. allocations were done above^
    //
    //XXX: nothing below seems to be allocating anymore, how come!
    println!("Hello, second world!");
    println!("current thread:{:?}", std::thread::current());
    println!("current tid:{:?}", std::thread::current().id());
    println!("current tid:{:?}", std::thread::current().id().as_u64());
    let handle = std::thread::spawn(|| {//that allocated 48+48+24+16
        println!("Hello, from thread world!");//doesn't alloc
        //nothing below allocs either!
        println!("current thread:{:?}", std::thread::current());
        println!("current tid:{:?}", std::thread::current().id());
        println!("current tid:{:?}", std::thread::current().id().as_u64());
    });
    let _=handle.join();
    println!("bye!");
}
