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

// Define a type alias for the HashMap to simplify the code
type VisitedLocations = HashMap<Location, bool>;

// Thread-local storage for the visited locations
thread_local! {
    static VISITED_LOCATIONS: RefCell<VisitedLocations> = RefCell::new(VisitedLocations::new());
}
/*
macro_rules! been_here {
    () => {{
        let location = Location {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        VISITED_LOCATIONS.with(|locations| {
            let mut visited_locations = locations.borrow_mut();
            let visited = visited_locations.get(&location).cloned().unwrap_or(false);
            if !visited {
                visited_locations.insert(location, true);
            }
            visited
        })
    }};
}*/
macro_rules! been_here {
    () => {{
        let location = Location {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        VISITED_LOCATIONS.with(|locations| {
            locations.borrow_mut().insert(location, true).map_or(false, |visited| visited)
        })
    }};
}


// Function to check if a location has been visited
fn is_visited(file: &'static str, line: u32, column: u32) -> bool {
    let location = Location { file, line, column };
    VISITED_LOCATIONS.with(|locations| locations.borrow().get(&location).cloned().unwrap_or(false))
}

// Function to display the contents of the VisitedLocations hashmap
fn display_visited_locations() {
    VISITED_LOCATIONS.with(|locations| {
        println!("Visited Locations:");
        for (location, visited) in locations.borrow().iter() {
            println!("{:?}: {}", location, visited);
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
//    let location = Location {
//        file: file!(),
//        line: line!(),
//        column: column!(),
//    };
//    println!(
//        "Is main() visited? {}",
//        is_visited(location.file, location.line, location.column)
//    );

    // Display the contents of the VisitedLocations hashmap
    display_visited_locations();
}


