use std::cell::RefCell;
use std::collections::HashMap;
use std::thread_local;

// Struct to store location information
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location {
    file: &'static str,
    line: u32,
    column: u32,
}

// Helper struct to decrement counter on Drop
#[derive(Debug)]
struct LocationGuard {
    //this bool is only used to hold the return bool
    //so doesn't have to be part of this struct actually.
    visited: bool,

    //this location is used to know which location to unvisit when going out of scope!
    location: Location,
}

//if it's a trait, the funcs within can be overridden by user(s) by implementing trait extension(s)
//AND still be able to call the original functions here via like UnVisit::unvisit(self)
//ah crap, nevermind this, I can still use LocationGuard::unvisit(self) even if I don't have a
//trait, which in retrospect makes sense, heh.
//trait UnVisit {
//    fn unvisit(&self);
//}

//impl UnVisit for LocationGuard {
impl LocationGuard {
    fn unvisit(&self) {
        //unvisits
        //TODO: handle error cases, ie. what if can't borrow, or stuff.
        let res=VISITED_LOCATIONS.try_with(|locations| {
            //TODO: can this drop() be called again if this panics here? or in some other cases?
            //I think it's more likely drop() won't be called at all in some cases like exit()
            if let Ok(mut locations) = locations.try_borrow_mut() {
                println!("!{:?}",locations);
                if let Some(counter) = locations.get_mut(&self.location) {
                    if *counter > 0 {
                        *counter -= 1;
                    }
                }
            }
        });
        eprintln!("unvisiting, res={:?}",res);
    }
}

impl Drop for LocationGuard {
    fn drop(&mut self) {
        self.unvisit();
    }
}

// Thread-local storage for the visited locations
thread_local! {
    static VISITED_LOCATIONS: RefCell<HashMap<Location, u64>> = RefCell::new(HashMap::new());
}

// Macro to mark a location as visited
/// aka "am i recursing due to this"
/// or better: "if I'm recursing, has this been done/encountered before?"
/// if I'm not recusing then this is false, even if used in a loop(due to Drop happening after each
/// loop)
// so it's more like, have I seen this in this session,
// and session is the current block. hmm..
/// aka 'recursion guard' or 'recurse guard', in THIS thread!
//TODO: should I rename this to something more obvious of what's happening?
macro_rules! been_here {
    () => {{
        let location = Location {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        let was_visited_before=VISITED_LOCATIONS.try_with(|locations| {
            let mut visited_locations = locations.borrow_mut();
            let counter = visited_locations.entry(location.clone()).or_insert(0);
            *counter += 1;
            *counter > 1 // Return true if location was previously visited (counter > 1)
        }).unwrap_or(true);
        //XXX: so we say visited=true if failed to acquire lock which means it's likely due to recursion
        //while inside the try_with() closure, ie. been_here!() is called again while inside the
        //above try_with(), how? maybe this is used inside the rust std panic handling code and it
        //panicked inside the try_with() somehow!
        //doneTODO: return the bool and the Option<Location> so that it can be *counter-=1 later when
        //done; i don't think we can do this on Drop because catch_unwind() would trigger it, hmm,
        //maybe this is a good thing? didn't think this thru.
        let guard = LocationGuard {
            visited: was_visited_before,
            location: location,

        };
        guard // Return the guard instance
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
fn recursive_function(level:usize) {
    let leading_spaces = format!("{:width$}", "", width = level as usize);

    //begins an action block that's protected from infinite recursion:
    let visited = been_here!(); // Mark this location as visited, XXX: until caller' scope ends!
                                // or manually drop()
    println!("{}Visited? {:?} level={}", leading_spaces,visited, level);
    if !visited.visited {
        println!("{}!!! recursing1 level={}",leading_spaces,level);
        recursive_function(level+1);
    }
    drop(visited);
    //^ ends scope early, because we can say the action that this 'visited' was
    //protecting, has completed successfully.
    //so then below, any other recursion will allow the above block to execute again as if fresh.

    //begin another action block but protects against inf.rec. until the scope ends.
    let visited = been_here!(); // Mark this location as visited, XXX: until caller' scope ends!
    println!("{}Visited2? {:?} level={}", leading_spaces,visited, level);
    if !visited.visited {
        println!("{}!!! recursing2 level={}",leading_spaces,level);
        recursive_function(level+1);
    }

    // Your recursive logic here...
}//unvisits here.

fn main() {
//    let handle = std::thread::spawn(|| {
//        recursive_function(); // Call recursive_function in a separate thread
//        display_visited_locations();
//    });
    recursive_function(1);
    println!("Starting again.........");
    recursive_function(1);
    // Wait for the spawned thread to finish
//    handle.join().unwrap();
    for i in 1..=5 {
        let bh=been_here!();
        if bh.visited {
            unreachable!("i={}",i);
        }
    }//bh is dropped here on every cycle!

    // Display the contents of the VisitedLocations hashmap
    display_visited_locations();
}
