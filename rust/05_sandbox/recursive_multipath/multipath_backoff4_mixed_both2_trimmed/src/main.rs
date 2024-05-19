#![feature(internal_output_capture)]

use std::cell::RefCell;
use std::thread_local;
use std::fmt;
include!(concat!(env!("OUT_DIR"), "/project_dir.rs")); //gets me 'PROJECT_DIR'


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
    //this is the tracker that we use to update every time we enter/exit the zone
    location_tracker: T,
}

impl fmt::Display for RecursionDetectionZoneGuard<&'static HeapAllocsThreadLocalForThisZone> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.is_recursing, self.location_tracker)
    }
}

impl fmt::Display for RecursionDetectionZoneGuard<&'static NoHeapAllocsThreadLocalForThisZone> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "{} {:?}", self.is_recursing, self.location_tracker)
        write!(f, "{}", self.is_recursing)
    }
}

trait UnvisitTrait {
    fn unvisit(&self);
}



impl UnvisitTrait for RecursionDetectionZoneGuard<&'static HeapAllocsThreadLocalForThisZone> {

    //mustn't call this manually
    fn unvisit(&self) {
        //unvisits
        //if self.can_heap_alloc {
        //TODO: try_with() "This function will still panic!() if the key is uninitialized and the key’s initializer panics."
        //TODO: handle error cases, ie. what if can't borrow, or stuff.
        let res=self.location_tracker.try_with(|refcell| {
            //let i:i32=refcell;//found `&RefCell<Option<...>>`
            let mut res_borrow=refcell.try_borrow_mut();
            if let Ok(ref mut ref_mut_location) = res_borrow {
                //let i:i32=ref_mut_location;//found `RefMut<'_, Option<...>>`
                //println!("!{}",self.location);
                if let Some(sal) = ref_mut_location.as_mut() {
                    //let i:i32=counter;//&mut StuffAboutLocation
                    if *sal> 0 {
                        *sal -= 1;
                    } else {
                        //TODO: return Result<> ? but then rename to try_unvisit() ?
                        panic!("counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!(1)", sal);
                    }
                } else {
                    eprintln!("!!! unvisiting found None as the L.W.C., this is pretty bad as it means inconsistency in coding the logic");
                }
            } else {
                eprintln!("!!! unvisiting errored, couldn't borrow, this is pretty bad as it means inconsistency in tracking, error='{:?}'",res_borrow);
            }
            drop(res_borrow);//now can be dropped
        });
        if let Err(err)=res {
            //TODO: this is pretty bad, maybe somehow set the is_recursing bool to some default ?
            eprintln!("!!! unvisiting errored, this is pretty bad as it means inconsistency in tracking, error='{}'",err);
        }
    }
}//impl

/// Define the maximum number of threads that are concurrently supported in the same zone,
/// before putting new ones on wait(with a timeout) until the prev. ones exit the zone.
const MAX_NUM_THREADS_AT_ONCE: usize = 10;
//doneTODO: need to rename this type:
type NoHeapAllocsThreadLocalForThisZone=no_heap_allocations_thread_local::NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE,StuffAboutLocation>;
impl UnvisitTrait for RecursionDetectionZoneGuard<&NoHeapAllocsThreadLocalForThisZone> {

    //mustn't call this manually
    fn unvisit(&self) {
        //println!("unvisiting self={:?}",self);
        let mut can_dispose:bool=false;
        {
            let mut loc=self.location_tracker.maybe_get_mut_ref_if_set();
            //let i:i32=loc;//`Option<RefMut<'_, Option<LocationWithCounter>>>`
            if let Some(ref mut refmut)=loc {
                //let i:i32=refmut;//`RefMut<'_, Option<LocationWithCounter>>`
                //so it's already being used
                if let Some(sal)=refmut.as_mut() {
                    //let i:i32=sal;//`&mut LocationWithCounter`
                    //let i:i32=sal.counter;//found `StuffAboutLocation`
                    if *sal > 0 {
                        *sal -=1;
                        if *sal == 0 {
                            can_dispose=true;
                        }
                    } else {
                        panic!("counter was already 0 or less = '{:?}', coded wrongly?! or manually invoked!(2)", sal);
                    }
                }
                //drop(refmut);
            } else {
                //it's not used, can drop it:
                can_dispose=true;
            }
            drop(loc);//E0382: use of partially moved value: `loc`
        }//so, is 'loc' dropped here or what? yeFIXME
        if can_dispose {
            //yesTODO: test to see if this is ever called!
            //println!("disposing current tid from noallocthreadlocal {:?}",self.location);
            self.location_tracker.unset();
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
#[derive(Debug)]
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

//needed for comparisons like: self.counter > u64
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
    (noheapalloc start, $timeout:expr, $default_value_on_timeout:expr) => {
        been_here!($timeout, $default_value_on_timeout)
    };
    (no_heap_alloc start, $timeout:expr, $default_value_on_timeout:expr) => {
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
    (noheapalloc start, $timeout:expr) => {
        been_here!($timeout)
    };
    (no_heap_alloc start, $timeout:expr) => {
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
type TLHeapAllocsThreadLocalForThisZone = RefCell<Option<StuffAboutLocation>>;
//This is for the reference(&) to what we've declared with thread_local
type HeapAllocsThreadLocalForThisZone = std::thread::LocalKey<TLHeapAllocsThreadLocalForThisZone>;
//ohwellTODO: get rid of thread_local!() macro call, and thus use only one type alias here! It won't work, still needs 2 types! so no use.
//cantTODO: actually don't need it to be a RefCell, since we're giving the whole static to the guard! but for the noalloc version we do. Still need RefCell wrapper with thread_local!() else I can't mutate the inner value because .try_with() gives me an immutable ref.

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
            static A_STATIC_FOR_THIS_CALL_LOCATION: TLHeapAllocsThreadLocalForThisZone = TLHeapAllocsThreadLocalForThisZone::new(None);
            //doneTODO: keep a max times visited?
        }
        let was_visited_before=A_STATIC_FOR_THIS_CALL_LOCATION.try_with(|refcell| {
            let mut ref_mut=refcell.borrow_mut();
            //let i:i32=ref_mut;//found `RefMut<'_, Option<...>>`
            if ref_mut.is_none() {
                //first time init:
                *ref_mut=Some(StuffAboutLocation::initial());
            }
            assert!(ref_mut.is_some(),"code logic is wrong");
            let sal=ref_mut.as_mut().unwrap();
            //let i:i32=sal;//found `&mut StuffAboutLocation`
            *sal += 1;
            *sal > 1 // Return true if is_recursing (counter > 1)
            //assert_eq!(ref_mut.as_mut().unwrap().counter,1,"developer coded it wrongly");
        }).unwrap_or(true);
        //XXX: so we say is_recursing=true if failed to acquire lock which means it's likely due to recursion
        //while inside the try_with() closure, ie. recursion_detection_zone!(start) is called again while inside the
        //above try_with(), how? maybe this is used inside the rust std panic handling code and it
        //panicked inside the try_with() somehow!
        //doneTODO: return the bool and the Option<LocationInSourceCode> so that it can be *counter-=1 later when
        //done; i don't think we can do this on Drop because catch_unwind() would trigger it, hmm,
        //maybe this is a good thing? didn't think this thru.
        let guard:RecursionDetectionZoneGuard<&'static HeapAllocsThreadLocalForThisZone> = RecursionDetectionZoneGuard {
            is_recursing: was_visited_before,
            location_tracker: &A_STATIC_FOR_THIS_CALL_LOCATION,
            //nogoodTODO: maybe don't give ref to the static, but a ref to the inner instead? which means, we'd need the RefCell::borrow_mut() here. Well actually giving a refcell mut ref here would prevent recursive call from modifying the inner because it's already mut borrowed!

        };
        guard // Return the guard instance
    }};
//---------
    //TODO: code is duplicated in the following 2 macro branches. This is very bad for keeping things in sync when modifying the code in one of them.
    ($timeout:expr, $default_value_on_timeout:expr) => {{
        static LOCATION_VAR: NoHeapAllocsThreadLocalForThisZone = NoHeapAllocsThreadLocalForThisZone::new();

        let (was_already_set,sal_refmut)=LOCATION_VAR.get_or_set(
            StuffAboutLocation::initial(),
            $timeout,
            false,
            );
        let was_visited_before=if let Some(mut sal)=sal_refmut {
            let sal=sal.as_mut().unwrap();
            //let i:i32=sal;//`&mut LocationWithCounter`
            //assert_eq!(sal, &mut clone,"the type of the static is coded wrongly!");
            assert!(*sal>=0);
            let was_visited_before= *sal>0;
            *sal+=1;
            assert_eq!(was_visited_before, was_already_set, "these two should be in sync");
            //drop(sal);//it's a ref
            was_visited_before
        } else {
            assert!(sal_refmut.is_none());
            drop(sal_refmut);
            //ie. timeout
            fn assert_bool(_: bool) {}
            assert_bool($default_value_on_timeout);
            $default_value_on_timeout
        };
        let guard = RecursionDetectionZoneGuard {
            is_recursing: was_visited_before,
            location_tracker: &LOCATION_VAR,
        };
        guard // Return the guard instance
    }};
//---------
    //TODO: code is duplicated in the 2 macro branches (the one above and the one below). This is very bad for keeping things in sync when modifying the code in one of them.
    ($timeout:expr) => {{
        //doneFIXME: well now need this to be thread_local but without allocating, soo... fixed sized
        //array which would represent only the currently visiting(counter>0) location paired with
        //thread id number, as one of the elements of the array.
        //and have new threads wait if it's full, but with a timeout(5sec?) and if timeout then
        //return what? true that it's recursing or false that it's now? allow user to provide value
        //to be returned if timeout?
        //use no_heap_allocations_thread_local::NoHeapAllocThreadLocal;
        //static LOCATION_VAR: NoHeapAllocThreadLocal<MAX_NUM_THREADS_AT_ONCE,LocationWithCounter> = NoHeapAllocThreadLocal::new();
        static LOCATION_VAR: NoHeapAllocsThreadLocalForThisZone = NoHeapAllocsThreadLocalForThisZone::new();

        let (was_already_set,sal_refmut)=LOCATION_VAR.get_or_set(
            StuffAboutLocation::initial(),
            $timeout,
            false,
            );
        if let Some(mut sal)=sal_refmut {
            let sal=sal.as_mut().unwrap();
            //assert_eq!(sal, &mut clone,"the type of the static is coded wrongly!");
            assert!(*sal>=0);
            let was_visited_before= *sal>0;
            *sal+=1;
            assert_eq!(was_visited_before, was_already_set, "these two should be in sync");
            //drop(sal);//it's a ref
            let guard = RecursionDetectionZoneGuard {
                is_recursing: was_visited_before,
                location_tracker: &LOCATION_VAR,
            };
            Some(guard) // Return the guard instance
        } else {
            assert!(sal_refmut.is_none());
            drop(sal_refmut);
            //ie. timeout
            None
        }
    }};
//---------
}//macro

//macro_rules! been_here_no_alloc_helper {
//}


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
    let zone1_guard = recursion_detection_zone!(noalloc start, ONE_SECOND).unwrap();
    println!("{}┌zone1, recursing from it? {} level={}", leading_spaces, zone1_guard, level);
    if !zone1_guard.is_recursing {
        recursion_detection_zone!(end, zone1_guard);//end zone manually
        //let zone2_guard=recursion_detection_zone!(start);
        let zone2_guard=recursion_detection_zone!(noalloc start, ONE_SECOND).unwrap();
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
    //let zone3_guard = recursion_detection_zone!(start);
    let zone3_guard = recursion_detection_zone!(noalloc start, ONE_SECOND).unwrap();
    println!("{}├zone3, recursing from it? {} level={}", leading_spaces,zone3_guard, level);
    if !zone3_guard.is_recursing {
        println!("{}{}zone3, recursion starting from level={}",leading_spaces, PIPE,level);
        recursive_function(level+1);
    }

    println!("{}└ending recursion at level={}",leading_spaces, level);
}//zone2_guard unvisits here.

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
