#![feature(thread_id_value)]

use std::alloc::{GlobalAlloc, Layout};

struct MyGlobalAllocator;
unsafe impl GlobalAlloc for MyGlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _=std::thread::current();
        //XXX: ^ infinite recursion because it's allocating, so Segmentation fault      (core dumped)
        //https://github.com/rust-lang/rust/blob/e8ada6ab253b510ac88edda131021d9878f2984f/library/std/src/thread/mod.rs#L1321-L1349
        let ptr = std::alloc::System.alloc(layout);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::System.dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = std::alloc::System.realloc(ptr, layout, new_size);
        new_ptr
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyGlobalAllocator = MyGlobalAllocator;

fn main() {
}
