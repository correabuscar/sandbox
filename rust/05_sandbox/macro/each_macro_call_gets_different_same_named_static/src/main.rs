use std::sync::Mutex;
use std::thread;

macro_rules! create_static {
    ($initial_value:expr) => {{
        // Define the type of the static variable
        struct MyType {
            // Example fields
            value: i32,
        }

        // Create a static reference to hold the shared value
        // XXX: so this static is different per each macro call location(ie. if location of (macro)call is
        // different, it's a different static!)
        static SHARED_STATIC: Mutex<MyType> = Mutex::new(MyType { value: $initial_value });

        // Access the static variable
        let mut shared = SHARED_STATIC.lock().unwrap();

        // Print the value of the static variable
        let value = shared.value;
        println!("Static variable value: {} addr:{:?}", value,
                 std::ptr::addr_of!(SHARED_STATIC) 
                 );
        shared.value+=1;//add 1 to the inner static i32
    }};
}

fn call_macro() {
    create_static!(42);//XXX: same static no matter which thread calls this fn.
}

fn main() {
    // Call the function that invokes the macro in the main thread
    call_macro(); //same static
    call_macro(); //same static
    create_static!(10);//diff. static
    // Spawn a new thread
    let thread = thread::spawn(|| {
        // Call the function that invokes the macro in the new thread
        call_macro(); //same static
    });

    // Wait for the thread to finish
    thread.join().unwrap();
}

