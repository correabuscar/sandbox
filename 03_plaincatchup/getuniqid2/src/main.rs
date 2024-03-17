use std::thread;
use thread_id;

fn main() {
    //ok so same number, per thread! not new uniq number on each call! got it!
    let spawned = thread::spawn(move || {
        println!("spawned thread has id {}", thread_id::get());
        println!("spawned thread has id {}", thread_id::get());
    });
    //spawned.join().unwrap();

    println!("main thread has id {}", thread_id::get());
    println!("main thread has id {}", thread_id::get());
    spawned.join().unwrap();
}
