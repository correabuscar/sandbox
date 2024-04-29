#![feature(thread_local)]

// Function to recursively call itself
fn recurse(depth: usize) {
    // Print the current depth to observe stack usage
    println!("Depth: {}, stack left: {:?}", depth, stacker::remaining_stack());
    let cloj=|| {
        //XXX: so array allocs stack mem in multiples of 16?
        let mut _buffer = [0u8; 16*10];//960 left for 16 or less, but 944 left for 17 hmmm
        //816 left for 16*10, and 16*10+1 is overflow! so 816 or less, or maybe <=800 is overflow!
        println!("From cloj, stack left: {:?}", stacker::remaining_stack());
    };

    // Recursively call itself
    if depth >=58 {
        println!("End={}",depth);
        //panic!("wtw");
        cloj();
    } else {
        let foo=depth+1;
        recurse(foo);
    }
}

fn main() {
    println!("Hello, main world! {:?}", stacker::remaining_stack());
    // Set the desired stack size
    let stack_size = 150*128+5696;//via `cargo run` to 150 depth, 149 is min.

    // Create a new thread with the specified stack size
    let result = std::thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            // Your main program logic goes here
            println!("Hello, thread world! {:?}", stacker::remaining_stack());
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
