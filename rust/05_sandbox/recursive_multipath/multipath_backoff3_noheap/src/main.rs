
#[derive(PartialEq, Eq, Hash, Debug)]
struct LocationInSourceCode {
    file: &'static str,
    line: u32,
    column: u32,
}

// Define a struct to hold the location information and counter
#[derive(Debug)]
struct LocationWithCounter {
    #[allow(dead_code)]
    location: LocationInSourceCode,
    counter: u32,
}

// Define the maximum number of threads that are concurrently supported in the same zone,
// before putting new ones on wait until the prev. ones exit the zone.
const MAX_NUM_THREADS_AT_ONCE: usize = 10;

macro_rules! initialize_thread_location {
    () => {{
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
            counter: 0,
        };

        // Increment the counter and print the location information
        unsafe {
            LOCATION_VAR.counter += 1;
            //println!("LocationWithCounter: {:?}", LOCATION_VAR);
        }

        // Return a wrapper object that decrements the counter when dropped
        CounterGuard {
            location_var: unsafe { &mut LOCATION_VAR },
        }
    }};
}

// Define a wrapper object that decrements the counter when dropped
#[derive(Debug)]
struct CounterGuard<'a> {
    location_var: &'a mut LocationWithCounter,
}

impl<'a> CounterGuard<'a> {
    fn is_recursing(&self) -> bool {
        self.location_var.counter>1
    }
}

impl<'a> Drop for CounterGuard<'a> {
    fn drop(&mut self) {
        self.location_var.counter -= 1;
        println!("!!drop {:?}", self.location_var);
    }
}

fn recursive_func() {
    let guard1 = initialize_thread_location!();
    println!("{:?}", guard1);
    if !guard1.is_recursing() {
        println!("!!r1 {:?}", guard1);
        drop(guard1);
        let guard2 = initialize_thread_location!();
        println!("{:?}", guard2);
        if !guard2.is_recursing() {
            println!("!!r2 {:?}", guard2);
            recursive_func();
        }
        drop(guard2);
        println!("!!e2");
    } else {
        drop(guard1);
    }
    println!("!!e1");

}

fn main() {
    recursive_func();
}
