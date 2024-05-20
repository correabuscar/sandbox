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
        std::alloc::System.dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
//        let _res=FOO_REALLOC.try_with(|_| {
//            std::println!("Reallocating {} bytes", layout.size());
//        });
        let new_ptr = std::alloc::System.realloc(ptr, layout, new_size);
        new_ptr
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyGlobalAllocator = MyGlobalAllocator;

fn main() {
    println!("Hello, world!");
}
