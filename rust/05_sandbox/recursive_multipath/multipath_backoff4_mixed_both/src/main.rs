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
    //mustn't call this manually
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
                        //XXX: on purpose not removing from the static list! we might wanna know
                        //which points were hit at all. And maybe even add a max-times-hit.
                    } else {
                        //TODO: return Result<> ? but then rename to try_unvisit() ?
                        panic!("counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!", *counter);
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
    pub fn done(self) {
        self.drop();
    }

    #[inline(always)]
    pub fn drop(self) {
        drop(self);
    }

    #[inline(always)]
    pub fn end_zone_aka_drop(self) {
        self.drop();
    }
}

impl Drop for RecursionDetectionZoneGuard {
    fn drop(&mut self) {
        self.unvisit();
    }
}

/// not meant to be accessible by caller
#[derive(Debug)]
struct StuffAboutLocation {
    //this is 1 or more while in the zone
    //if it's more than 1 it's currently recursing and recursion started from within the zone
    times_visited_currently: u64,

    //a 1 on this means normal execution
    //a 2+ means recursed this many times minus 1
    max_times_visited_ever: u64,
}

//impl PartialEq for StuffAboutLocation {
//    fn eq(&self, other: &Self) -> bool {
//        self.times_visited_currently == other.times_visited_currently
//    }
//}
//
//impl PartialOrd for StuffAboutLocation {
//    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//        self.times_visited_currently.partial_cmp(&other.times_visited_currently)
//    }
//}

impl PartialEq<u64> for StuffAboutLocation {
    fn eq(&self, other: &u64) -> bool {
        self.times_visited_currently == *other
    }
}

impl PartialOrd<u64> for StuffAboutLocation {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.times_visited_currently.partial_cmp(other)
    }
}

impl fmt::Display for StuffAboutLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.times_visited_currently)
    }
}

impl std::ops::SubAssign<u64> for StuffAboutLocation {
    fn sub_assign(&mut self, rhs: u64) {
        self.times_visited_currently -= rhs;
        self.update_max();
    }
}


impl std::ops::AddAssign<u64> for StuffAboutLocation {
    fn add_assign(&mut self, rhs: u64) {
        self.times_visited_currently += rhs;
        self.update_max();
    }
}

impl StuffAboutLocation {
    //FIXME: user can still init the struct with struct initializer syntax and set max to be less
    //than current(if current is >0), then u'd have to call update_max() from below!
    pub fn initial() -> StuffAboutLocation {
        return StuffAboutLocation { times_visited_currently:0, max_times_visited_ever:0 };
    }

    #[inline(always)]
    pub fn update_max(&mut self) {
        if self.times_visited_currently > self.max_times_visited_ever {
            self.max_times_visited_ever=self.times_visited_currently;
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn get_max_seen(&mut self) -> u64 {
        self.update_max();
        self.max_times_visited_ever
    }
}

// Thread-local storage for the is_recursing locations
thread_local! {
    static PER_THREAD_VISITED_LOCATIONS: RefCell<HashMap<LocationInSourceCode, StuffAboutLocation>> = RefCell::new(HashMap::new());
    //TODO: unclear why using RefCell instead of Cell
    //doneTODO: keep a max times visited?
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
    (mark beginning) => {
        been_here!()
    };
    (noalloc_start, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here_without_allocating!($timeout, $default_value_on_timeout);
    };
    (noalloc_begin, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here_without_allocating!($timeout, $default_value_on_timeout);
    };
    (noalloc_new, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here_without_allocating!($timeout, $default_value_on_timeout);
    };
    (noalloc_mark_beginning, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here_without_allocating!($timeout, $default_value_on_timeout);
    };
    (noalloc mark beginning, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here_without_allocating!($timeout, $default_value_on_timeout);
    };
// -----------
    (end, $guard:ident) => {
        been_here_end!($guard)
    };
    (end_zone_aka_drop, $guard:ident) => {
        been_here_end!($guard)
    };
    (done, $guard:ident) => {
        been_here_end!($guard)
    };
    (drop, $guard:ident) => {
        been_here_end!($guard)
    };
    (finish, $guard:ident) => {
        been_here_end!($guard)
    };
    (mark end, $guard:ident) => {
        been_here_end!($guard)
    };
    (mark_end, $guard:ident) => {
        been_here_end!($guard)
    };
    (mark_ending, $guard:ident) => {
        been_here_end!($guard)
    };
    (mark ending, $guard:ident) => {
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

        //XXX: saves the call location and the number of times it was encountered, into a static hashmap,
        //but the zone in which, if recursing, the number of times encountered is increased can be ended
        //via dropping the returned guard, which decreases the num.times encountered.
        let location = LocationInSourceCode {
            file: file!(),
            line: line!(),
            column: column!(),
        };
        let was_visited_before=PER_THREAD_VISITED_LOCATIONS.try_with(|locations| {
            let mut visited_locations = locations.borrow_mut();
            let counter = visited_locations.entry(location.clone()).or_insert(StuffAboutLocation::initial());
            *counter += 1;
            *counter > 1 // Return true if location was previously is_recursing (counter > 1)
        }).unwrap_or(true);
        //XXX: so we say is_recursing=true if failed to acquire lock which means it's likely due to recursion
        //while inside the try_with() closure, ie. recursion_detection_zone!(start) is called again while inside the
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

struct LocationWithCounter {
    location: LocationInSourceCode,
    counter: StuffAboutLocation,
}

// Define the maximum number of threads that are concurrently supported in the same zone,
// before putting new ones on wait(with a timeout) until the prev. ones exit the zone.
const MAX_NUM_THREADS_AT_ONCE: usize = 10;

macro_rules! been_here_without_allocating {
    ($timeout:expr, $default_value_on_timeout:expr) => {{
        //FIXME: well now need this to be thread_local but without allocating, soo... fixed sized
        //array which would represent only the currently visiting(counter>0) location paired with
        //thread id number, as one of the elements of the array.
        //and have new threads wait if it's full, but with a timeout(5sec?) and if timeout then
        //return what? true that it's recursing or false that it's now? allow user to provide value
        //to be returned if timeout?
        static mut LOCATION_VAR: LocationWithCounter = LocationWithCounter {
            location: LocationInSourceCode {
                file: file!(),
                line: line!(),
                column: column!(),
            },
            counter: StuffAboutLocation::initial(),
        };

        // Increment the counter and print the location information
        unsafe {
            LOCATION_VAR.counter += 1;
        }

        let guard = RecursionDetectionZoneGuard {
            is_recursing: was_visited_before,
            location: unsafe { &mut LOCATION_VAR },

        };
        guard // Return the guard instance
    }};
}


// Function to display the contents of the VisitedLocations hashmap
fn display_visited_locations() {
    PER_THREAD_VISITED_LOCATIONS.with(|locations| {
        println!("Visited Locations in thread id='{:?}':", std::thread::current().id());
        for (location, count) in locations.borrow().iter() {
            println!("{} {:?}", location, count);
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
    let zone1_guard = recursion_detection_zone!(start); // Mark this location as start of zone that needs protection from recursion and the zone ends when manually dropped or until caller' scope ends!
                                // or manually drop()
    println!("{}┌zone1, recursing from it? {} level={}", leading_spaces, zone1_guard, level);
    if !zone1_guard.is_recursing {
        recursion_detection_zone!(end, zone1_guard);//end zone manually
        let zone2_guard=recursion_detection_zone!(start);
        println!("{}├zone2, recursing from it? {} level={}", leading_spaces, zone2_guard, level);
        if !zone2_guard.is_recursing {
            println!("{}{}zone2, recursion starting from level={}",leading_spaces,PIPE,level);
            recursive_function(level+1);
        }
        recursion_detection_zone!(end, zone2_guard);//explicit tho not needed, if we're relying on end-of-scope drop()
    } else {
        //drop(zone1_guard);
        //zone1_guard.done();
        //zone1_guard.drop();
        recursion_detection_zone!(end, zone1_guard);
        //^(any above) ends scope(aka zone) early, because we can say the action that this 'zone1_guard' was
        //protecting, has completed successfully.
        //so then below, any other recursion will allow the above block to execute again as if fresh, because
        //presumably the recursion wasn't triggered by the above block!
    }

    //begin another action block but protects against inf.rec. until the scope ends.
    let zone3_guard = recursion_detection_zone!(start);
    println!("{}├zone3, recursing from it? {} level={}", leading_spaces,zone3_guard, level);
    if !zone3_guard.is_recursing {
        println!("{}{}zone3, recursion starting from level={}",leading_spaces, PIPE,level);
        recursive_function(level+1);
    }

    println!("{}└ending recursion at level={}",leading_spaces, level);
}//zone2_guard unvisits here.

fn main() {
    let handle = std::thread::spawn(|| {
        recursive_function(1); // Call recursive_function in a separate thread
        display_visited_locations();
    });
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    println!("Recursion test starting.........");
    recursive_function(1);
    println!("Starting again.........");
    recursive_function(1);
    println!("Recursion test done.");
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
