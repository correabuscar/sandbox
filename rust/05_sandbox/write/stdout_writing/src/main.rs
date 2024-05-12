use std::thread;
use std::io::{self, Write};

const BUFFER_SIZE: usize = 100 * 1024 * 1024; // 300 MB buffer size

fn main() {
    // Create a thread that writes to stdout
    let thread1 = thread::spawn(|| {
        write_to_stdout("Thread 1 writes to stdout!\n".repeat(BUFFER_SIZE));
    });

    // Create another thread that writes to stdout
    let thread2 = thread::spawn(|| {
        write_to_stdout("Thread 2 writes to stdout!\n".repeat(BUFFER_SIZE));
    });

    // Write to stdout from the main thread
    write_to_stdout("Main thread writes to stdout!\n".repeat(BUFFER_SIZE));

    // Wait for threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn write_to_stdout(data: String) {
    io::stdout().write_all(data.as_bytes()).unwrap(); // Write data to stdout
}

