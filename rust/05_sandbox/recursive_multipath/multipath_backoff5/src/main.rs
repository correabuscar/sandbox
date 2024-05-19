#![feature(internal_output_capture)] //mainly used to assert the output from thread&main is same, during recursion.

use multipath_backoff5::recursion_detection_zone;

const ONE_SECOND:std::time::Duration = std::time::Duration::from_secs(1);

// Example usage
fn recursive_function(level:usize) {
    const PIPE:char='│';
    let leading_spaces = if level>1 {
        //format!("{:width$}", PIPE, width = level as usize)
        PIPE.to_string().repeat(level)
    } else { "".to_string() };

    //begins an action block that's protected from infinite recursion:
    // Mark this location as start of zone that needs protection from recursion and the zone ends when manually dropped or until caller' scope ends!
    // or manually drop()
    //let zone1_guard = recursion_detection_zone!(start);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let zone1_guard = recursion_detection_zone!(noheapalloc start, ONE_SECOND).unwrap();
    println!("{}┌zone1, recursing from it? {} level={}", leading_spaces, zone1_guard, level);
    if !zone1_guard.is_recursing {
        recursion_detection_zone!(end, zone1_guard);//end zone manually
        let zone2_guard=recursion_detection_zone!(start);
        //let zone2_guard=recursion_detection_zone!(no_heap_alloc start, ONE_SECOND).unwrap();
        println!("{}├zone2, recursing from it? {} level={}", leading_spaces, zone2_guard, level);
        if !zone2_guard.is_recursing {
            println!("{}{}zone2, recursion starting from level={}",leading_spaces,PIPE,level);
            recursive_function(level+1);
        }
        recursion_detection_zone!(end_zone, zone2_guard);//explicit tho not needed, if we're relying on end-of-scope drop()
    } else {
        //drop(zone1_guard);
        //zone1_guard.done();
        //zone1_guard.drop();
        recursion_detection_zone!(end zone, zone1_guard);
        //^(any above) ends scope(aka zone) early, because we can say the action that this 'zone1_guard' was
        //protecting, has completed successfully.
        //so then below, any other recursion will allow the above block to execute again as if fresh, because
        //presumably the recursion wasn't triggered by the above block!
    }

    //begin another action block but protects against inf.rec. until the scope ends.
    //let zone3_guard = recursion_detection_zone!(start);
    let zone3_guard = recursion_detection_zone!(noalloc start, ONE_SECOND).unwrap();
    println!("{}├zone3, recursing from it? {} level={}", leading_spaces,zone3_guard, level);
    if !zone3_guard.is_recursing {
        println!("{}{}zone3, recursion starting from level={}",leading_spaces, PIPE,level);
        recursive_function(level+1);
    }

    println!("{}└ending recursion at level={}",leading_spaces, level);
}//zone3_guard unvisits here.

fn main() {
    let handle = std::thread::spawn(|| {
        std::io::set_output_capture(Some(Default::default()));
        //std::io::set_output_capture(None);
        recursive_function(1); // Call recursive_function in a separate thread
        //display_visited_locations();
        let captured = std::io::set_output_capture(None).unwrap();
        let captured_string = {
          let captured_mutex = captured.lock().unwrap();
          String::from_utf8_lossy(&captured_mutex[..]).into_owned()
        };
        print!("Captured from thread:\n{}", captured_string);
        captured_string
    });
    // Wait for the spawned thread to finish, else intermixed output. FIXME: use temp buffer?
    println!("Recursion test starting.........");
    std::io::set_output_capture(Some(Default::default()));
    //std::io::set_output_capture(None);
    recursive_function(1);
    let res=handle.join().unwrap();
    let captured = std::io::set_output_capture(None).unwrap();
    let captured_string = {
        let captured_mutex = captured.lock().unwrap();
        String::from_utf8_lossy(&captured_mutex[..]).into_owned()
    };
    print!("Captured from main:\n{}", captured_string);
    assert_eq!(res,captured_string);//same output from thread as from main, even tho they ran concurrently
    println!("Starting again.........in main only:");
    recursive_function(1);
    println!("Recursion test done.");
    for i in 1..=5 {
        //XXX: Allow this following statement to exist so we're reminded to update the second part of the code that's duplicated in the macro! for keeping the two code copies in sync.
        let _rd_zone_guard=recursion_detection_zone!(noalloc start,std::time::Duration::from_secs(3)).unwrap();
        //let rd_zone_guard=recursion_detection_zone!(noalloc start,3, true);
        let rd_zone_guard=recursion_detection_zone!(noalloc start,std::time::Duration::from_secs(3), true);
        //let rd_zone_guard=recursion_detection_zone!(start);
        if rd_zone_guard.is_recursing {
            unreachable!("i={}",i);
        }
    }//rd_zone_guard is dropped here on every cycle!
    let rd_zone_guard=recursion_detection_zone!(start);
    for i in 1..=5 {
        if rd_zone_guard.is_recursing {
            // the value is constant and only changes when the same recursion_detection_zone!() is
            // called again!
            unreachable!("i={}",i);
        }
    }
    rd_zone_guard.end_zone_aka_drop();

    // Display the contents of the VisitedLocations hashmap
    //display_visited_locations();
}
