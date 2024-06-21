use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    //XXX: the logic here with just AtomicBool(s) assumes one thread only; don't use this for
    // multithreads, u'd need the AtomicBools to be threadlocals or something!
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        static ALREADY_BEING_HERE: AtomicBool = AtomicBool::new(false);
        //eprintln!("!! before alloc, size={}",layout.size());
        if !ALREADY_BEING_HERE.load(Ordering::Relaxed) {
            eprintln!("!! before alloc, size={}", layout.size());
            if PANIC_ON_ALLOC.load(Ordering::Relaxed) {
                // since panic!() will alloc
                match ALREADY_BEING_HERE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(prev) => {
                        assert_eq!(false, prev);
                        //using std::process::abort() or std::alloc::handle_alloc_error(layout ) here would be
                        // preferred but it creates coredumps which I might want to avoid.
                        eprintln!("allocation detected when it shouldn't have allocated anymore!");
                        std::alloc::handle_alloc_error(layout); //never returns, see: https://doc.rust-lang.org/alloc/alloc/fn.handle_alloc_error.html

                        //panic!("allocation detected when it shouldn't have allocated anymore!");
                        // this panic deadlocks in cleanup() of stdio due to STDOUT.get_or_init() ah, it's because the
                        // realloc below gets triggered and we didn't also panic in it! which would detect a double
                        // panic and abort instead of deadlock. sure maybe panic shouldn't be
                        // called from the allocator, but still, the type of STDOUT seems off.
                        // put it back, in case we decide to comment out the panic!() call!
                        #[allow(unreachable_code)]
                        {
                            let _ =
                                ALREADY_BEING_HERE.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed);
                        }
                    }
                    Err(prev) => {
                        assert_eq!(true, prev);
                    }
                }
            }
        }
        if layout.size() == 1024 {
            // XXX: hacky!, likely the stdout buffer being allocated on first use of print!() or println!(), we
            // emulate allocation failure
            eprintln!("1024");
            //std::ptr::null_mut() // return null ptr to signal allocation failure! won't deadlock!
            System.alloc(layout)
        } else {
            // Delegating to System allocator for actual allocation
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        static HAPPENED_ONCE_ALREADY: AtomicBool = AtomicBool::new(false);
        if !HAPPENED_ONCE_ALREADY.load(Ordering::Relaxed) {
            // Implement custom deallocation logic here
            // Delegating to System allocator for actual deallocation
            eprintln!("!! before dealloc, size={}", layout.size());
            match HAPPENED_ONCE_ALREADY.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(prev) => {
                    assert_eq!(false, prev);
                    eprintln!("!! further deallocs ignored to avoid spam");
                }
                Err(prev) => {
                    assert_eq!(true, prev);
                }
            }
        }
        System.dealloc(ptr, layout)
    }

    //not hit by any code below, currently.
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        static ALREADY_BEING_HERE: AtomicBool = AtomicBool::new(false);
        if !ALREADY_BEING_HERE.load(Ordering::Relaxed) {
            eprintln!("!! before alloc_zeroed, size={}", layout.size());
            if PANIC_ON_ALLOC.load(Ordering::Relaxed) {
                // since panic!() will alloc
                match ALREADY_BEING_HERE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(prev) => {
                        assert_eq!(false, prev);
                        panic!("allocation(zeroed) detected when it shouldn't have allocated anymore!");
                        //put it back, in case we decide to comment out the panic!() call!
                        #[allow(unreachable_code)]
                        {
                            let _ =
                                ALREADY_BEING_HERE.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed);
                        }
                    }
                    Err(prev) => {
                        assert_eq!(true, prev);
                    }
                }
            }
        }
        // Delegating to System allocator for actual allocation
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        static ALREADY_BEING_HERE: AtomicBool = AtomicBool::new(false);
        //eprintln!("!! before realloc, oldsize={} newsize={}",layout.size(), new_size);
        if !ALREADY_BEING_HERE.load(Ordering::Relaxed) {
            eprintln!("!! before realloc, oldsize={} newsize={}", layout.size(), new_size);
            match ALREADY_BEING_HERE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(prev) => {
                    assert_eq!(false, prev);
                    eprintln!(
                        "REallocation detected when it shouldn't have allocated anymore! Further reallocs are ignored to avoid spam."
                    );
                    //panic!("REallocation detected when it shouldn't have allocated anymore!");

                    //put it back (to enable spam)
                    //let _=ALREADY_BEING_HERE.compare_exchange(true,false,Ordering::Relaxed,
                    // Ordering::Relaxed);
                }
                Err(prev) => {
                    assert_eq!(true, prev);
                }
            }
        }
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyAllocator = MyAllocator;

static PANIC_ON_ALLOC: AtomicBool = AtomicBool::new(false);

//#[deny(unused_must_use)] // works ofc, because it applies to call sites!
//fn main() {
fn main() {
    //XXX: so stdout deadlocks on cleanup only if a panic happens on first use of stdout, ie. when it
    // tries to init it by allocating 1024 bytes, it panics within the alloc triggering the cleanup thus
    // seeing it not having been inited, so it tries to do another init, but since we're within the
    // first init, deadlocks. But as long as the buffer is already inited, it won't deadlock later!
    // println!("supnewline");//on first print to stdout it allocates 1k buffer
    //print!("yes this new lined part will be flushed\nbut this no new line part won't be flushed
    // supNOnewline");//on first print to stdout it allocates 1k buffer
    PANIC_ON_ALLOC.store(true, Ordering::Relaxed); //from now on, panic on any memory allocations!
    print!(
        "!!!! this line won't be seen anyway because it panics within this\nsup!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
    );
    //this allocates on first use a buffer(of 1k) for stdout.
    // that print without newline at end won't be flushed on exit when deadlocking (or when avoiding
    // the deadlock)

    //let mut vec = Vec::<i32>::with_capacity(200); //another way to alloc without needing stdout,
    // won't deadlock! vec.reserve_exact(200);//causes reallocation!
}
