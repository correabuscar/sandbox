#![feature(new_uninit)]

use std::alloc::{GlobalAlloc, Layout, System};
//use std::alloc;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Implement custom allocation logic here
        // Delegating to System allocator for actual allocation
        eprintln!("before alloc, size={}",layout.size());
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Implement custom deallocation logic here
        // Delegating to System allocator for actual deallocation
        eprintln!("before dealloc, size={}",layout.size());
        System.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        eprintln!("before alloc_zeroed, size={}",layout.size());
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        eprintln!("before realloc, oldsize={} newsize={}",layout.size(), new_size);
        System.realloc(ptr, layout, new_size)
    }

/*    // error[E0407]: method `dealloc_excess` is not a member of trait `GlobalAlloc`
    fn dealloc_excess(&self, ptr: *mut u8, layout: Layout, new_size: usize) {
        System.dealloc_excess(ptr, layout, new_size)
    }

    fn alloc_layout(&self, layout: Layout) -> Result<*mut u8, alloc::AllocError> {
        System.alloc_layout(layout)
    }

    fn realloc_layout(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> Result<*mut u8, alloc::AllocError> {
        System.realloc_layout(ptr, layout, new_size)
    }
*/
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyAllocator = MyAllocator;

fn main() {
    // Your program logic here
    println!("sup");//this allocates on first use a buffer for stdout.
    //struct MyStruct {
    //    data: [u8; 1024],
    //}
    //let my_struct = MyStruct {
    //    data: [0; 1024],
    //};
    //let _my_struct: Box<[u8; 1024]> = Box::new_zeroed();
        // Allocate memory for an array of 1024 bytes and zero-initialize it
    //let mut my_struct: Box<[u8; 1024]> = unsafe {
    //    Box::from_raw(MyAllocator::alloc_zeroed(&MyAllocator,Layout::array::<u8>(1024).unwrap()) as *mut [u8; 1024])
    //};

    //// Ensure that the memory is properly initialized to zero
    //assert_eq!(my_struct.iter().filter(|&&x| x == 0).count(), 1024);

    // Create a vector of integers
    //let mut vec: Vec<i32> = Vec::new();
    let mut vec = Vec::<i32>::new(); //turbofish
    //let mut vec: Vec<i32> = Vec::<i32>::new(); //turbofish & type
     // Reserve space for 1024 integers in the vector
    vec.reserve_exact(100);
    vec.reserve_exact(1024);

    unsafe {
    let layout = Layout::new::<u16>();
    let ptr = std::alloc::alloc_zeroed(layout);

    assert_eq!(*(ptr as *mut u16), 0);

    std::alloc::dealloc(ptr, layout);
    }
}
