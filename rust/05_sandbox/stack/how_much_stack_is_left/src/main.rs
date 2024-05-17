#![feature(thread_local)]

use std::io::Write;

// Function to recursively call itself
fn recurse(depth: usize) {
    // Print the current depth to observe stack usage
    eprintln!("Depth: {}, stack left: {:?}", depth, stacker::remaining_stack());
    let cloj=|| {
        //XXX: so array allocs stack mem in multiples of 16? presumably it's padding.
        let mut _buffer = [0u8; 16*10];//960 left for 16 or less, but 944 left for 17 hmmm
        //816 left for 16*10, and 16*10+1 is overflow! so 816 or less, or maybe <=800 is overflow!
        println!("From cloj, stack left: {:?}", stacker::remaining_stack());//this is ok with 16*10
        //eprintln!("From cloj, stack left: {:?}", stacker::remaining_stack());//this blows stack,
                                                                             //needs 16*8
//        // Get the standard output stream
//        let stdout = std::io::stderr();
//        // Lock the stdout so that it can be written to
//        let mut handle = stdout.lock();
//
//        // Evaluate the expression and store the result in a variable
//        let remaining_stack = stacker::remaining_stack().unwrap();
//
//        // Convert the value to a string representation
//        let stack_str = remaining_stack.to_string();
//
//        // Write the string to stdout
//        let _ =handle.write_all(b"From cloj, stack left: ");
//        let _ =handle.write_all(stack_str.as_bytes());
//        let _=handle.write_all(b"\n");
//
//        // Ensure the output is flushed to the console
//        handle.flush().expect("Failed to flush stdout");
    };

    // Recursively call itself
    #[cfg(not(debug_assertions))]
    const DEPTH:usize=121;
    #[cfg(debug_assertions)]
    const DEPTH:usize=58;
    if depth >=DEPTH {
        println!("End={}",depth);
        //panic!("wtw");
        cloj();
    } else {
        let foo=depth+1;
        recurse(foo);
    }
}

fn main() {
    eprintln!("Hello, main world! {:?}", stacker::remaining_stack());
    // Set the desired stack size
    let stack_size = 150*128+5696;//via `cargo run` to 150 depth, 149 is min.


    // Create a new thread with the specified stack size
    let result = std::thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            // Your main program logic goes here
            let foo=stacker::remaining_stack();
            println!("Hello, thread world! {:?}", foo);
            recurse(1);
        });

    // Handle errors
    match result {
        Ok(handle) => {
            // Wait for the thread to finish
            handle.join().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to create thread: {:?}", e);
        }
    }
}
