use std::cell::RefCell;
use std::collections::HashMap;
use std::thread_local;

// Struct to store location information
#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    file: &'static str,
    line: u32,
    column: u32,
}

// Thread-local storage for the visited locations
thread_local! {
    static VISITED_LOCATIONS: RefCell<HashMap<Location, u64>> = RefCell::new(HashMap::new());
}

// Macro to mark a location as visited
macro_rules! been_here {
    () => {{
        let location = Location {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        VISITED_LOCATIONS.with(|locations| {
            let mut visited_locations = locations.borrow_mut();
            let counter = visited_locations.entry(location).or_insert(0);
            *counter += 1;

            //I know it's u64 now, so can't be negative, but what if type changes in the future!
            assert!(*counter > 0, "Counter was somehow negative '{}'", counter);

            *counter > 1 // Return true if location was previously visited (counter > 1)
        })
    }};
}

// Function to display the contents of the VisitedLocations hashmap
fn display_visited_locations() {
    VISITED_LOCATIONS.with(|locations| {
        println!("Visited Locations in thread id='{:?}':", std::thread::current().id());
        for (location, count) in locations.borrow().iter() {
            println!("{:?} (Visited {} times)", location, count);
        }
    });
}

// Example usage
fn recursive_function() {
    let visited = been_here!(); // Mark this location as visited
    println!("Visited? {}", visited);
    if !visited {
        recursive_function();
    }

    // Your recursive logic here...
}

fn main() {
    let handle = std::thread::spawn(|| {
        recursive_function(); // Call recursive_function in a separate thread
        display_visited_locations();
    });
    recursive_function();
    // Wait for the spawned thread to finish
    handle.join().unwrap();

    // Display the contents of the VisitedLocations hashmap
    display_visited_locations();
}
