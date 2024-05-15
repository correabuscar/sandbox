use std::cell::RefCell;
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
        // Remove the prefix of PROJECT_DIR from the file field, so output is less cluttered!
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
struct RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    //this bool is only used to hold the return bool from the macro call.
    //so doesn't have to be part of this struct actually.
    //and is thus only updated due to the call, not afterwards.
    is_recursing: bool,

    //this location is used to know which location to unvisit when going out of scope!
    location: T,
}

impl fmt::Display for RecursionDetectionZoneGuard<&'static AllocThreadLocalForThisZone> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?})", self.is_recursing, self.location)
    }
}

trait UnvisitTrait {
    fn unvisit(&self);
}



impl UnvisitTrait for RecursionDetectionZoneGuard<&'static AllocThreadLocalForThisZone> {

    //mustn't call this manually
    fn unvisit(&self) {
        //unvisits
        //if self.can_heap_alloc {
        //TODO: try_with() "This function will still panic!() if the key is uninitialized and the key’s initializer panics."
        //TODO: handle error cases, ie. what if can't borrow, or stuff.
        let res=self.location.try_with(|refcell| {
            //let i:i32=refcell;//found `&RefCell<Option<...>>`
            if let Ok(mut ref_mut_location) = refcell.try_borrow_mut() {
                //let i:i32=ref_mut_location;//found `RefMut<'_, Option<...>>`
                //println!("!{}",self.location);
                if let Some(lwc) = ref_mut_location.as_mut() {
                    //let i:i32=counter;//&mut StuffAboutLocation
                    if lwc.counter > 0 {
                        lwc.counter -= 1;
                    } else {
                        //TODO: return Result<> ? but then rename to try_unvisit() ?
                        panic!("counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!(1)", lwc);
                    }
                }
            }
        });
        if let Err(err)=res {
            eprintln!("unvisiting errored, error={}",err);
        }
    }
}//impl

/// Define the maximum number of threads that are concurrently supported in the same zone,
/// before putting new ones on wait(with a timeout) until the prev. ones exit the zone.
const MAX_NUM_THREADS_AT_ONCE: usize = 10;
//TODO: need to rename this type:
type NoHeapAllocationsThreadLocalForHere=no_heap_allocations_thread_local::NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE,LocationWithCounter>;
impl UnvisitTrait for RecursionDetectionZoneGuard<&NoHeapAllocationsThreadLocalForHere> {

    //mustn't call this manually
    fn unvisit(&self) {
        //println!("unvisiting self={:?}",self);
        let mut can_dispose:bool=false;
        {
            let loc=self.location.maybe_get_mut_ref_if_set();
            //let i:i32=loc;//`Option<RefMut<'_, Option<LocationWithCounter>>>`
            if let Some(mut refmut)=loc {
                //let i:i32=refmut;//`RefMut<'_, Option<LocationWithCounter>>`
                //so it's already being used
                if let Some(lwc)=refmut.as_mut() {
                    //let i:i32=lwc;//`&mut LocationWithCounter`
                    //let i:i32=lwc.counter;//found `StuffAboutLocation`
                    if lwc.counter > 0 {
                        lwc.counter-=1;
                        if lwc.counter == 0 {
                            can_dispose=true;
                        }
                    } else {
                        panic!("counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!(2)", lwc.counter);
                    }
                }
                //drop(refmut);
            } else {
                //it's not used, can drop it:
                can_dispose=true;
            }
        //drop(loc);//E0382: use of partially moved value: `loc` 
        }//so, is 'loc' dropped here or what? FIXME
        if can_dispose {
            //yesTODO: test to see if this is ever called!
            //println!("disposing current tid from noallocthreadlocal {:?}",self.location);
            self.location.unset();
            //println!("disposed current tid from noallocthreadlocal {:?}",self.location);
        }
    }
}

impl<T> RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
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

impl<T> Drop for RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    fn drop(&mut self) {
        self.unvisit();
    }
}

/// not meant to be accessible by caller
#[derive(Debug, Clone, PartialEq)]
struct StuffAboutLocation {
    //this is 1 or more while in the zone
    //if it's more than 1 it's currently recursing and recursion started from within the zone
    times_visited_currently: u64,

    //a 1 on this means normal execution
    //a 2+ means recursed this many times minus 1
    max_times_visited_ever: u64,
}

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
// -----------
    (noalloc start, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here!($timeout, $default_value_on_timeout)
    };
    (noalloc begin, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here!($timeout, $default_value_on_timeout)
    };
    (noalloc new, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here!($timeout, $default_value_on_timeout)
    };
    (noalloc mark_beginning, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here!($timeout, $default_value_on_timeout)
    };
    (noalloc mark beginning, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here!($timeout, $default_value_on_timeout)
    };
// -----------
    (noalloc start, $timeout:expr) => {
        been_here!($timeout)
    };
    (noalloc begin, $timeout:expr) => {
        been_here!($timeout)
    };
    (noalloc new, $timeout:expr) => {
        been_here!($timeout)
    };
    (noalloc mark_beginning, $timeout:expr) => {
        been_here!($timeout)
    };
    (noalloc mark beginning, $timeout:expr) => {
        been_here!($timeout)
    };
// -----------
    (end, $guard:ident) => {
        been_here!(end, $guard)
    };
    (end_zone, $guard:ident) => {
        been_here!(end, $guard)
    };
    (end zone, $guard:ident) => {
        been_here!(end, $guard)
    };
    (end_zone_aka_drop, $guard:ident) => {
        been_here!(end, $guard)
    };
    (done, $guard:ident) => {
        been_here!(end, $guard)
    };
    (drop, $guard:ident) => {
        been_here!(end, $guard)
    };
    (finish, $guard:ident) => {
        been_here!(end, $guard)
    };
    (mark end, $guard:ident) => {
        been_here!(end, $guard)
    };
    (mark_end, $guard:ident) => {
        been_here!(end, $guard)
    };
    (mark_ending, $guard:ident) => {
        been_here!(end, $guard)
    };
    (mark ending, $guard:ident) => {
        been_here!(end, $guard)
    };
}

//TL=for the thread_local declaration
type TLAllocThreadLocalForThisZone = RefCell<Option<LocationWithCounter>>;
//This is for the reference to what we've declared with thread_local
type AllocThreadLocalForThisZone = std::thread::LocalKey<TLAllocThreadLocalForThisZone>;
//TODO: get rid of thread_local!() macro call, and thus use only one type alias here!
//TODO: actually don't need it to be a RefCell, since we're giving the whole static to the guard! but for the noalloc version we do.

macro_rules! been_here {
//---------
    (end, $guard:ident) => {
        $guard.end_zone_aka_drop();
    };
//---------
    () => {{ //double curlies, all the way! else 'let' won't work; single {} expects expression,
             //double {{}} is like a normal {} that returns an expression even if it's () unit.


        // Thread-local storage for the current zone/call-location of this macro
        thread_local! {
            //XXX: thread_local itself does heap alloc internally(because pthread_key_create does alloc)!
            //it's gonna be a different static for each location where this macro is called; seems it has same name but internally it's mangled and global, however only visible here.
            static A_STATIC_FOR_THIS_CALL_LOCATION: TLAllocThreadLocalForThisZone = TLAllocThreadLocalForThisZone::new(None);
            //doneTODO: keep a max times visited?
        }
        let was_visited_before=A_STATIC_FOR_THIS_CALL_LOCATION.try_with(|refcell| {
            let mut ref_mut=refcell.borrow_mut();
            if ref_mut.is_none() {
                let loc_of_this_macro_call=
                    //This is the location(in source code) of our macro call.
                    LocationWithCounter {
                        location: LocationInSourceCode {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                        counter: StuffAboutLocation::initial(),
                    };
                *ref_mut=Some(loc_of_this_macro_call);
            }
            assert!(ref_mut.is_some(),"code logic is wrong");
            let lwc=ref_mut.as_mut().unwrap();
            //let i:i32=lwc;//found `&mut LocationWithCounter`
            lwc.counter+=1;
            lwc.counter > 1 // Return true if is_recursing (counter > 1)
            //assert_eq!(ref_mut.as_mut().unwrap().counter,1,"developer coded it wrongly");
        }).unwrap_or(true);
        //XXX: so we say is_recursing=true if failed to acquire lock which means it's likely due to recursion
        //while inside the try_with() closure, ie. recursion_detection_zone!(start) is called again while inside the
        //above try_with(), how? maybe this is used inside the rust std panic handling code and it
        //panicked inside the try_with() somehow!
        //doneTODO: return the bool and the Option<LocationInSourceCode> so that it can be *counter-=1 later when
        //done; i don't think we can do this on Drop because catch_unwind() would trigger it, hmm,
        //maybe this is a good thing? didn't think this thru.
        let guard:RecursionDetectionZoneGuard<&'static AllocThreadLocalForThisZone> = RecursionDetectionZoneGuard {
            is_recursing: was_visited_before,
            location: &A_STATIC_FOR_THIS_CALL_LOCATION,
            //nogoodTODO: maybe don't give ref to the static, but a ref to the inner instead? which means, we'd need the RefCell::borrow_mut() here. Well actually giving a refcell mut ref here would prevent recursive call from modifying the inner because it's already mut borrowed!

        };
        guard // Return the guard instance
    }};
//---------
    //TODO: code is duplicated in the following 2 macro branches. This is very bad for keeping things in sync when modifying the code in one of them.
    ($timeout:expr, $default_value_on_timeout:expr) => {{
        static LOCATION_VAR: NoHeapAllocationsThreadLocalForHere = NoHeapAllocationsThreadLocalForHere::new();

        let loc_of_this_macro_call=
            //This is the location(in source code) of our macro call.
            LocationWithCounter {
                location: LocationInSourceCode {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                counter: StuffAboutLocation::initial(),
            };
        let mut clone=loc_of_this_macro_call.clone();
        let (was_already_set,lwc_refmut)=LOCATION_VAR.get_or_set(
            loc_of_this_macro_call,
            $timeout,
            true,
            );
        let was_visited_before=if let Some(mut lwc)=lwc_refmut {
            let lwc=lwc.as_mut().unwrap();
            //let i:i32=lwc;//`&mut LocationWithCounter`
            assert_eq!(lwc, &mut clone,"the type of the static is coded wrongly!");
            assert!(lwc.counter>=0);
            let was_visited_before= lwc.counter>0;
            lwc.counter+=1;
            assert_eq!(was_visited_before, was_already_set, "these two should be in sync");
            //drop(lwc);//it's a ref
            was_visited_before
        } else {
            assert!(lwc_refmut.is_none());
            drop(lwc_refmut);
            //ie. timeout
            fn assert_bool(_: bool) {}
            assert_bool($default_value_on_timeout);
            $default_value_on_timeout
        };
        let guard = RecursionDetectionZoneGuard {
            is_recursing: was_visited_before,
            location: &LOCATION_VAR,
        };
        guard // Return the guard instance
    }};
//---------
    ($timeout:expr) => {{
        //doneFIXME: well now need this to be thread_local but without allocating, soo... fixed sized
        //array which would represent only the currently visiting(counter>0) location paired with
        //thread id number, as one of the elements of the array.
        //and have new threads wait if it's full, but with a timeout(5sec?) and if timeout then
        //return what? true that it's recursing or false that it's now? allow user to provide value
        //to be returned if timeout?
        //use no_heap_allocations_thread_local::NoHeapAllocThreadLocal;
        //static LOCATION_VAR: NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE,LocationWithCounter> = NoHeapAllocThreadLocal::new();
        static LOCATION_VAR: NoHeapAllocationsThreadLocalForHere = NoHeapAllocationsThreadLocalForHere::new();

        let loc_of_this_macro_call=
            //This is the location(in source code) of our macro call.
            LocationWithCounter {
                location: LocationInSourceCode {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                counter: StuffAboutLocation::initial(),
            };
        let mut clone=loc_of_this_macro_call.clone();
        let (was_already_set,lwc_refmut)=LOCATION_VAR.get_or_set(
            loc_of_this_macro_call,
            $timeout,
            true,
            );
        if let Some(mut lwc)=lwc_refmut {
            let lwc=lwc.as_mut().unwrap();
            assert_eq!(lwc, &mut clone,"the type of the static is coded wrongly!");
            assert!(lwc.counter>=0);
            let was_visited_before= lwc.counter>0;
            lwc.counter+=1;
            assert_eq!(was_visited_before, was_already_set, "these two should be in sync");
            //drop(lwc);//it's a ref
            let guard = RecursionDetectionZoneGuard {
                is_recursing: was_visited_before,
                location: &LOCATION_VAR,
            };
            Some(guard) // Return the guard instance
        } else {
            assert!(lwc_refmut.is_none());
            drop(lwc_refmut);
            //ie. timeout
            None
        }
    }};
//---------
}//macro

#[derive(Debug, Clone, PartialEq)]
struct LocationWithCounter {
    location: LocationInSourceCode,
    counter: StuffAboutLocation,
}


//macro_rules! been_here_without_allocating {
//}//macro


// Function to display the contents of the VisitedLocations hashmap
//fn display_visited_locations() {
//    PER_THREAD_VISITED_LOCATIONS.with(|locations| {
//        println!("Visited Locations in thread id='{:?}':", std::thread::current().id());
//        for (location, count) in locations.borrow().iter() {
//            println!("{} {:?}", location, count);
//        }
//    });
//}

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
        //display_visited_locations();
    });
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    println!("Recursion test starting.........");
    recursive_function(1);
    println!("Starting again.........");
    recursive_function(1);
    println!("Recursion test done.");
    for i in 1..=5 {
        //let rd_zone_guard=recursion_detection_zone!(noalloc start,std::time::Duration::from_secs(3)).unwrap();
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
