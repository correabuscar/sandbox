#![feature(internal_output_capture)]
use std::sync::{Arc, Mutex};

fn main() {
    // Enable output capture (unstable feature)
    std::io::set_output_capture(Some(Default::default()));

    // Print something to capture
    println!("hello");
    eprintln!("world");

    // Disable output capture and get captured output
    let captured = std::io::set_output_capture(None);
    let captured = captured.unwrap();

    // Convert captured data to a String
    let captured_string = {
        let captured_mutex = captured.lock().unwrap();
        String::from_utf8_lossy(&captured_mutex[..]).into_owned()
    };

    // Print the captured output as a string
    print!("Captured:\n{}", captured_string);
}

