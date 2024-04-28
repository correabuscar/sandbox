#![feature(thread_local)]

// Function to recursively call itself
fn recurse(depth: usize) {
    // Print the current depth to observe stack usage
    println!("Depth: {}", depth);

    // Recursively call itself
    let foo=depth+1;
    if depth >=57 {
        println!("End={}",depth);
        panic!("12345678901234567890123456789012345678901234567890123456789012341234567890123456789012345678901234567890123456789012345678901234123456789012345678901234567890123456789012345678901234567890123412345678901234567890123456789012345678901234567890123456789012341234567890123456789012345678901234567890123456789012345678901234123456789012345678901234567890123456789012345678901234567890123412345678901234567890123456789012345678901234567890123456789012341234567890123456789012345678901234567890123456789012345678901234_djsl");
    } else {
    recurse(foo);
    }
}

fn main() {
    // Set the desired stack size
    //let stack_size = 132*192-576;//on playground, 132 depth, 131 is minimum.
    let stack_size = 150*128+5696;//via `cargo run` to 150 depth, 149 is min.

    // Create a new thread with the specified stack size
    let result = std::thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            // Your main program logic goes here
            println!("Hello, world!");
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

