use std::cell::RefCell;
use std::collections::HashMap;
use std::thread_local;
use std::fmt;
include!(concat!(env!("OUT_DIR"), "/project_dir.rs")); //gets me 'PROJECT_DIR'


// Struct to store location information
#[derive(PartialEq, Eq, Hash, Clone)]
struct LocationInSourceCode {
    file: &'static str,
    line: u32,
    column: u32,
}

impl fmt::Debug for LocationInSourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Remove the prefix of PROJECT_DIR from the file field
        let file_without_prefix = match self.file.strip_prefix(PROJECT_DIR) {
            Some(suffix) => suffix,
            None => self.file,
        };

        f.debug_struct("LocationInSourceCode")
            .field("file", &file_without_prefix)
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}


impl fmt::Display for LocationInSourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Remove the prefix of PROJECT_DIR from the file field
        let file_without_prefix = match self.file.strip_prefix(PROJECT_DIR) {
            Some(suffix) => suffix,
            None => self.file,
        };

        write!(f, "{}:{}:{}", file_without_prefix, self.line, self.column)
    }
}

// Helper struct to decrement location's in-use counter on Drop
#[derive(Debug)]
struct RecursionDetectionZoneGuard {
    //this bool is only used to hold the return bool
    //so doesn't have to be part of this struct actually.
    is_recursing: bool,

    //this location is used to know which location to unvisit when going out of scope!
    location: LocationInSourceCode,
}

impl fmt::Display for RecursionDetectionZoneGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {})", self.is_recursing, self.location)
    }
}

impl RecursionDetectionZoneGuard {
    fn unvisit(&self) {
        //unvisits
        //TODO: try_with() "This function will still panic!() if the key is uninitialized and the key’s initializer panics."
        //TODO: handle error cases, ie. what if can't borrow, or stuff.
        let res=PER_THREAD_VISITED_LOCATIONS.try_with(|locations| {
            //TODO: can this drop() be called again if this panics here? or in some other cases?
            //I think it's more likely drop() won't be called at all in some cases like exit()
            if let Ok(mut locations) = locations.try_borrow_mut() {
                //println!("!{}",self.location);
                //display_visited_locations();//this does cause borrow error
                if let Some(counter) = locations.get_mut(&self.location) {
                    if *counter > 0 {
                        *counter -= 1;
                    }
                }
            }
        });
        if let Err(err)=res {
            eprintln!("unvisiting errored, error={}",err);
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    fn done(self) {
        self.drop();
    }

    #[inline(always)]
    fn drop(self) {
        drop(self);
    }

    #[inline(always)]
    fn end_zone_aka_drop(self) {
        self.drop();
    }
}

impl Drop for RecursionDetectionZoneGuard {
    fn drop(&mut self) {
        self.unvisit();
    }
}

// Thread-local storage for the is_recursing locations
thread_local! {
    static PER_THREAD_VISITED_LOCATIONS: RefCell<HashMap<LocationInSourceCode, u64>> = RefCell::new(HashMap::new());
    //TODO: unclear why using RefCell instead of Cell
}

// Macro to mark a location as is_recursing
/// aka "am i recursing due to this"
/// or better: "if I'm recursing, has this been done/encountered before?"
/// if I'm not recusing then this is false, even if used in a loop(due to Drop happening after each
/// loop)
// so it's more like, have I seen this in this session,
// and session is the current block. hmm..
/// aka 'recursion guard' or 'recurse guard', in THIS thread!
/// am I recursing in this zone?
/// or has this zone been used as launchpad for this recursion(if guard.is_recursing is true)
//okTODO: should I rename this to something more obvious of what's happening?
macro_rules! recursion_detection_zone {
    (start) => {
        been_here!()
    };
    (begin) => {
        been_here!()
    };
    (new) => {
        been_here!()
    };
    (mark_beginning) => {
        been_here!()
    };
    (end, $guard:ident) => {
        been_here_end!($guard)
    };
    (done, $guard:ident) => {
        been_here_end!($guard)
    };
    (finish, $guard:ident) => {
        been_here_end!($guard)
    };
    (mark_end, $guard:ident) => {
        been_here_end!($guard)
    };
}
macro_rules! been_here_end {
    ($guard:ident) => {
        $guard.end_zone_aka_drop();
    };
}
macro_rules! been_here {
    () => {{ //double curlies, all the way! else 'let' won't work; single {} expects expression,
             //double {{}} is like a normal {} that returns an expression even if it's () unit.
        let location = LocationInSourceCode {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        let was_visited_before=PER_THREAD_VISITED_LOCATIONS.try_with(|locations| {
            let mut visited_locations = locations.borrow_mut();
            let counter = visited_locations.entry(location.clone()).or_insert(0);
            *counter += 1;
            *counter > 1 // Return true if location was previously is_recursing (counter > 1)
        }).unwrap_or(true);
        //XXX: so we say is_recursing=true if failed to acquire lock which means it's likely due to recursion
        //while inside the try_with() closure, ie. recursion_detection_zone!() is called again while inside the
        //above try_with(), how? maybe this is used inside the rust std panic handling code and it
        //panicked inside the try_with() somehow!
        //doneTODO: return the bool and the Option<LocationInSourceCode> so that it can be *counter-=1 later when
        //done; i don't think we can do this on Drop because catch_unwind() would trigger it, hmm,
        //maybe this is a good thing? didn't think this thru.
        let guard = RecursionDetectionZoneGuard {
            is_recursing: was_visited_before,
            location: location,

        };
        guard // Return the guard instance
    }};
}

// Function to display the contents of the VisitedLocations hashmap
fn display_visited_locations() {
    PER_THREAD_VISITED_LOCATIONS.with(|locations| {
        println!("Visited Locations in thread id='{:?}':", std::thread::current().id());
        for (location, count) in locations.borrow().iter() {
            println!("{} (Visited {} times)", location, count);
        }
    });
}

// Example usage
fn recursive_function(level:usize) {
    const PIPE:char='│';
    let leading_spaces = if level>1 {
        //format!("{:width$}", PIPE, width = level as usize)
        PIPE.to_string().repeat(level)
    } else { "".to_string() };

    //begins an action block that's protected from infinite recursion:
    let zone1_guard = recursion_detection_zone!(start); // Mark this location as is_recursing, XXX: until caller' scope ends!
                                // or manually drop()
    println!("{}┌zone1, visited? {} level={}", leading_spaces, zone1_guard, level);
    if !zone1_guard.is_recursing {
        println!("{}{}zone1, recursion starting from level={}",leading_spaces,PIPE,level);
        recursive_function(level+1);
    }
    //drop(zone1_guard);
    //zone1_guard.done();
    //zone1_guard.drop();
    recursion_detection_zone!(end, zone1_guard);
    //^(any above) ends scope early, because we can say the action that this 'zone1_guard' was
    //protecting, has completed successfully.
    //so then below, any other recursion will allow the above block to execute again as if fresh.

    //begin another action block but protects against inf.rec. until the scope ends.
    let zone2_guard = recursion_detection_zone!(start); // Mark this location as is_recursing, XXX: until caller' scope ends!
    println!("{}├zone2, visited? {} level={}", leading_spaces,zone2_guard, level);
    if !zone2_guard.is_recursing {
        println!("{}{}zone2, recursion starting from level={}",leading_spaces, PIPE,level);
        recursive_function(level+1);
    }

    println!("{}└ending recursion at level={}",leading_spaces, level);
}//zone2_guard unvisits here.

fn main() {
//    let handle = std::thread::spawn(|| {
//        recursive_function(); // Call recursive_function in a separate thread
//        display_visited_locations();
//    });
    println!("Recursion test starting.........");
    recursive_function(1);
    println!("Starting again.........");
    recursive_function(1);
    println!("Recursion test done.");
    // Wait for the spawned thread to finish
//    handle.join().unwrap();
    for i in 1..=5 {
        let rd_zone_guard=recursion_detection_zone!(start);
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
    display_visited_locations();
}
