use std::cell::RefCell;
use std::collections::HashSet;
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
    static VISITED_LOCATIONS: RefCell<HashSet<Location>> = RefCell::new(HashSet::new());
}

// Macro to mark a location as visited
macro_rules! been_here {
    () => {{
        let location = Location {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        VISITED_LOCATIONS.with(|locations| !locations.borrow_mut().insert(location))
    }};
}

// Function to display the contents of the VisitedLocations hashset
fn display_visited_locations() {
    VISITED_LOCATIONS.with(|locations| {
        println!("Visited Locations:");
        for location in locations.borrow().iter() {
            println!("{:?}", location);
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
    recursive_function();

    // Display the contents of the VisitedLocations hashset
    display_visited_locations();
}

