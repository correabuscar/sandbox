#![feature(thread_local)]

// Function to recursively call itself
fn recurse(depth: usize) {
    // Print the current depth to observe stack usage
    println!("Depth: {}", depth);
    let cloj=|| {
        //let mut buffer = [0u8; 16384];
        let mut _buffer = [0u8; 560];//561 is overflow! but 560+80depth isn't!
        //let mut buffer = [0u8; 1];//1(or none) here and 81 depth is overflow
    };

    // Recursively call itself
    if depth >=80 {
        println!("End={}",depth);
        //panic!("wtw");
        cloj();
    } else {
        let foo=depth+1;
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

