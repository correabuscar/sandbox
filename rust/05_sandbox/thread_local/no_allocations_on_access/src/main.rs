FAIL;
//use std::cell::Cell;
//use std::thread::LocalKey;
use std::sync::atomic::{AtomicUsize, Ordering};

//static COUNTER_KEY: LocalKey<Cell<usize>> = LocalKey::new(Cell::new(0)); // Allocation happens here (outside function)
thread_local! {
  static COUNTER: AtomicUsize = AtomicUsize::new(0);
}

fn increment_counter() {
  COUNTER.with(|counter| counter.fetch_add(1, Ordering::Relaxed));
}

fn get_counter_value() -> usize {
  COUNTER.with(|counter| counter.load(Ordering::Relaxed))
}


//fn get_thread_local_cell() -> &'static LocalKey<Cell<usize>> {
//  &COUNTER_KEY
//}

//fn increment_counter() {
//  let key = get_thread_local_cell();
//  key.with(|cell| cell.get().set(cell.get() + 1)); // No allocation here (uses existing Cell)
//}
//
//fn get_counter_value() -> usize {
//  let key = get_thread_local_cell();
//  key.with(|cell| cell.get()) // No allocation here (uses existing Cell)
//}

//use std::alloc::{Layout, System, GlobalAlloc};
//
//struct MockAllocator;
//
//unsafe impl System for MockAllocator {
//    fn alloc(self, layout: Layout) -> Result<*mut u8, std::alloc::Error> {
//    // Track allocation attempts (fail the test here for verification)
//    panic!("Allocation attempted during counter access!");
//    // In a real implementation, you might return a placeholder or handle differently
//  }
//
//  fn dealloc(self, _ptr: *mut u8, _layout: Layout) {}
//
//  fn realloc(self, _ptr: *mut u8, _layout: Layout, _new_layout: Layout) -> Result<*mut u8, std::alloc::Error> {
//    panic!("Reallocation attempted during counter access!");
//    // Similar handling as alloc
//  }
//}
//
//fn with_mock_allocator<F>(f: F)
//where
//  F: FnOnce(),
//{
//  let original_alloc = GlobalAlloc::get();
//  GlobalAlloc::set(Box::new(MockAllocator));
//  f();
//  GlobalAlloc::set(original_alloc);
//}

//use std::alloc::System;
use std::alloc::{GlobalAlloc, Layout};

//static ALLOCATION_ATTEMPTED: AtomicBool = AtomicBool::new(false);
pub struct Mockalloc;//<GlobalAlloc>;

unsafe impl GlobalAlloc for Mockalloc {
  unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
     //ALLOCATION_ATTEMPTED.store(true, Ordering::Relaxed);
    panic!("Allocation attempted during counter access!");
  }

  unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}


  unsafe fn realloc(&self, _ptr: *mut u8, _layout: Layout, _new_size: usize) -> *mut u8 {
     //ALLOCATION_ATTEMPTED.store(true, Ordering::Relaxed);
    panic!("Reallocation attempted during counter access!");
  }

  unsafe fn alloc_zeroed(&self, _layout: Layout) -> *mut u8 {
     //ALLOCATION_ATTEMPTED.store(true, Ordering::Relaxed);
    panic!("Zeroed allocation attempted during counter access!");
  }
}


//fn with_mock_allocator<F>(f: F)
//where
//  F: FnOnce(),
//{
//  let old_alloc = <dyn GlobalAlloc>::get();
//  <dyn GlobalAlloc>::set(Box::new(Mockalloc));
//  f();
//  GlobalAlloc::set(old_alloc);
//
//  // Check for allocation attempts after test execution
//  //assert!(!ALLOCATION_ATTEMPTED.load(Ordering::Relaxed), "Allocation attempted during counter access!");
//}
//use std::cell::Cell;
//static ALLOCATOR: Cell<Option<Box<dyn GlobalAlloc>>> = Cell::new(None);
//
//fn set_global_allocator(alloc: Box<dyn GlobalAlloc>) {
//  ALLOCATOR.set(Some(alloc));
//}
//
//fn with_mock_allocator<F>(f: F)
//where
//  F: FnOnce(),
//{
//  let old_alloc = ALLOCATOR.get().take();
//  set_global_allocator(Box::new(Mockalloc));
//  f();
//  if let Some(alloc) = old_alloc {
//    set_global_allocator(alloc);
//  }
//}

fn with_mock_allocator<F>(f: F)
where
  F: FnOnce(),
{
    todo!();
  f();
}
//#[test]
//fn test_counter_no_allocations() {
//  with_mock_allocator(|| {
//    increment_counter();
//    get_counter_value();
//  });
//  // Test passes if no panics occurred within the closure
//}
#[test]
fn test_no_allocations_on_counter_access() {
  with_mock_allocator(|| {
    // Code using the thread-local counter (functions like increment_counter and get_counter_value)
    increment_counter();
    let count = get_counter_value();
    println!("Thread local counter: {}", count);

    // Spawn a thread (optional)
    let mut counter_thread = std::thread::spawn(|| {
      increment_counter();
      println!("Thread counter value: {}", get_counter_value());
    });

    counter_thread.join().unwrap();
  });
}

fn main() {
//  // Example usage
//  increment_counter();
//  let count = get_counter_value();
//  println!("Thread local counter: {}", count);

  let counter_thread = std::thread::spawn(|| {
    increment_counter();
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Thread counter value: {}", get_counter_value());
  });

  // Main thread also increments and prints counter
  increment_counter();
  println!("Main thread counter value: {}", get_counter_value());

  counter_thread.join().unwrap();
  println!("Main thread counter value: {}", get_counter_value());
}

