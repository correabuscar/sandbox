#![feature(internal_output_capture)] //mainly used to assert the output from thread&main is same, during recursion.
#![no_implicit_prelude]

extern crate std;
extern crate multipath_backoff5;

//use multipath_backoff5::recursion_detection_zone;
//use std::string::ToString; // for .to_string() to work.
use std::alloc::{GlobalAlloc, Layout};
//use std::ptr::NonNull;
//use std::sync::atomic::{AtomicUsize, Ordering};

struct MyGlobalAllocator;
unsafe impl GlobalAlloc for MyGlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //okFIXME: infinite recursion below due to std::thread::current() allocating due to using Arc
        //let maybe_zone1_guard = multipath_backoff5::recursion_detection_zone!(noheapalloc start, ONE_SECOND);
        //XXX: this is just another way of doing same thing as we do in dealloc/realloc below
        //multipath_backoff5::recursion_detection_zone!(noheapalloc start, ONE_SECOND, true);//XXX: shows must_use msg!
        let zone1_guard = multipath_backoff5::recursion_detection_zone!(noheapalloc start, ONE_SECOND, true);
        //if let std::option::Option::Some(zone1_guard)=maybe_zone1_guard {
            if !zone1_guard.is_recursing {
                std::eprintln!("Allocating {} bytes", layout.size());
            }
            zone1_guard.drop();
        //}

        let ptr = std::alloc::System.alloc(layout);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //FIXME: restore:
        //let maybe_zone1_guard = multipath_backoff5::recursion_detection_zone!(noheapalloc start, ONE_SECOND);
        //if let std::option::Option::Some(zone1_guard)=maybe_zone1_guard {
        //    if !zone1_guard.is_recursing {
        //        std::eprintln!("Deallocating {} bytes", layout.size());
        //    }
        //    zone1_guard.drop();
        //}
        std::alloc::System.dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        //FIXME: restore:
        //let maybe_zone1_guard = multipath_backoff5::recursion_detection_zone!(noheapalloc start, ONE_SECOND);
        //if let std::option::Option::Some(zone1_guard)=maybe_zone1_guard {
        //    if !zone1_guard.is_recursing {
        //        std::eprintln!("Reallocating {} bytes", layout.size());
        //    }
        //    zone1_guard.drop();
        //}
        let new_ptr = std::alloc::System.realloc(ptr, layout, new_size);
        new_ptr
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyGlobalAllocator = MyGlobalAllocator;

const ONE_SECOND:std::time::Duration = std::time::Duration::from_secs(1);

// Example usage
fn recursive_function(level:usize) {
    const PIPE:char='│';
    let leading_spaces = if level>1 {
        //format!("{:width$}", PIPE, width = level as usize)
        //PIPE.to_string().repeat(level)
        std::string::ToString::to_string(&PIPE).repeat(level)
    } else {
        //"".to_string()
        std::string::ToString::to_string("")
    };

    //begins an action block that's protected from infinite recursion:
    // Mark this location as start of zone that needs protection from recursion and the zone ends when manually dropped or until caller' scope ends!
    // or manually drop()
    //let zone1_guard = multipath_backoff5::recursion_detection_zone!(start);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let zone1_guard = multipath_backoff5::recursion_detection_zone!(noheapalloc start, ONE_SECOND).unwrap();
    std::println!("{}┌zone1, recursing from it? {} level={}", leading_spaces, zone1_guard, level);
    if !zone1_guard.is_recursing {
        multipath_backoff5::recursion_detection_zone!(end, zone1_guard);//end zone manually
        let zone2_guard=multipath_backoff5::recursion_detection_zone!(start);
        //let zone2_guard=recursion_detection_zone!(no_heap_alloc start, ONE_SECOND).unwrap();
        std::println!("{}├zone2, recursing from it? {} level={}", leading_spaces, zone2_guard, level);
        if !zone2_guard.is_recursing {
            std::println!("{}{}zone2, recursion starting from level={}",leading_spaces,PIPE,level);
            recursive_function(level+1);
        }
        multipath_backoff5::recursion_detection_zone!(end_zone, zone2_guard);//explicit tho not needed, if we're relying on end-of-scope drop()
    } else {
        //drop(zone1_guard);
        //zone1_guard.done();
        //zone1_guard.drop();
        multipath_backoff5::recursion_detection_zone!(end zone, zone1_guard);
        //^(any above) ends scope(aka zone) early, because we can say the action that this 'zone1_guard' was
        //protecting, has completed successfully.
        //so then below, any other recursion will allow the above block to execute again as if fresh, because
        //presumably the recursion wasn't triggered by the above block!
    }

    //begin another action block but protects against inf.rec. until the scope ends.
    //let zone3_guard = multipath_backoff5::recursion_detection_zone!(start);
    let zone3_guard = multipath_backoff5::recursion_detection_zone!(noalloc start, ONE_SECOND).unwrap();
    std::println!("{}├zone3, recursing from it? {} level={}", leading_spaces,zone3_guard, level);
    if !zone3_guard.is_recursing {
        std::println!("{}{}zone3, recursion starting from level={}",leading_spaces, PIPE,level);
        recursive_function(level+1);
    }

    std::println!("{}└ending recursion at level={}",leading_spaces, level);
}//zone3_guard unvisits here.

fn main() {
    const CAPTURE:bool=false;//FIXME: set to 'true' again
    const CAPTURE_ASSERT_EQUAL:bool=false;//won't work if u show each allocation
    std::println!("Hello initial println allocation.");//manually call this before anything else to cause allocation to happen.
    let handle = std::thread::Builder::new().name(
        std::string::ToString::to_string("silly goose"))
        .spawn(|| {
        if CAPTURE {
            std::io::set_output_capture(std::option::Option::Some(std::default::Default::default()));
        } else {
            std::io::set_output_capture(std::option::Option::None);
        }
        recursive_function(1); // Call recursive_function in a separate thread
        //display_visited_locations();
        if let std::option::Option::Some(captured) = std::io::set_output_capture(std::option::Option::None) {
        let captured_string = {
          let captured_mutex = captured.lock().unwrap();
          std::string::String::from_utf8_lossy(&captured_mutex[..]).into_owned()
        };
        std::print!("Captured from thread:\n{}", captured_string);
        captured_string
        } else {
            std::string::ToString::to_string("nothing captured")
        }
    }).unwrap();
    // Wait for the spawned thread to finish, else intermixed output. doneFIXME: use temp buffer?
    std::println!("Recursion test starting.........");
    if CAPTURE {
        std::io::set_output_capture(std::option::Option::Some(std::default::Default::default()));
    } else {
        std::io::set_output_capture(std::option::Option::None);
    }
    recursive_function(1);
    let res=handle.join().unwrap();
    if let std::option::Option::Some(captured) = std::io::set_output_capture(std::option::Option::None) {
        let captured_string = {
            let captured_mutex = captured.lock().unwrap();
            std::string::String::from_utf8_lossy(&captured_mutex[..]).into_owned()
        };
        std::print!("Captured from main:\n{}", captured_string);
        if CAPTURE_ASSERT_EQUAL {
            std::assert_eq!(res,captured_string);//same output from thread as from main, even tho they ran concurrently
        }
    }
    std::println!("Starting again.........in main only:");
    recursive_function(1);
    std::println!("Recursion test done.");
    for i in 1..=5 {
        //XXX: Allow this following statement to exist so we're reminded to update the second part of the code that's duplicated in the macro! for keeping the two code copies in sync.
        let _rd_zone_guard=multipath_backoff5::recursion_detection_zone!(noalloc start,std::time::Duration::from_secs(3)).unwrap();
        //let rd_zone_guard=multipath_backoff5::recursion_detection_zone!(noalloc start,3, true);
        let rd_zone_guard=multipath_backoff5::recursion_detection_zone!(noalloc start,std::time::Duration::from_secs(3), true);
        //let rd_zone_guard=multipath_backoff5::recursion_detection_zone!(start);
        if rd_zone_guard.is_recursing {
            std::unreachable!("i={}",i);
        }
    }//rd_zone_guard is dropped here on every cycle!
    let rd_zone_guard=multipath_backoff5::recursion_detection_zone!(start);
    for i in 1..=5 {
        if rd_zone_guard.is_recursing {
            // the value is constant and only changes when the same recursion_detection_zone!() is
            // called again!
            std::unreachable!("i={}",i);
        }
    }
    rd_zone_guard.end_zone_aka_drop();

    // Display the contents of the VisitedLocations hashmap
    //display_visited_locations();
}
