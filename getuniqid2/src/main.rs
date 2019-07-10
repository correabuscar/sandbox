use std::thread;
use thread_id;

fn main() {
    let spawned = thread::spawn(move || {
        println!("spawned thread has id {}", thread_id::get());
    });
    //spawned.join().unwrap();

    println!("main thread has id {}", thread_id::get());
    spawned.join().unwrap();
}
