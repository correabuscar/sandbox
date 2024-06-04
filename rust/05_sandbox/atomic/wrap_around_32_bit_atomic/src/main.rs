// 32 bit wrap around in 40 sec, so hmm +- 40x128=5120 years for 64 bit?
// inspired by: https://marabos.nl/atomics/atomics.html#example-id-allocation
// initially made via Gemini LLM
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
//use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    //let next_id = Arc::new(AtomicU32::new(0));
    //let running = Arc::new(AtomicU32::new(1));
    let next_id = &AtomicU32::new(0);
    let running = &AtomicBool::new(true);

    let i=std::time::Instant::now();
    thread::scope(|s| {
        let _printer = s.spawn(
            //{
            //let next_id_clone = next_id.clone(); // Clone next_id
            //let running_clone = running.clone(); // Clone running

            || {
                while running.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_secs(1));
                    println!("Current ID: {}", next_id.load(Ordering::Relaxed));
                }
            }
        //}
        );

        let _incrementer = s.spawn(|| {
            loop {
                let new_id = next_id.fetch_add(1, Ordering::Relaxed);
                if new_id == u32::MAX {
                    running.store(false, Ordering::Relaxed); // Use store for writes
                    break;
                }
            }

            println!("ID wrapped around, stopping");
        });

    });//threads are auto joined here!

    //printer.join().unwrap();
    println!("main exiting, elapsed: {} seconds", i.elapsed().as_secs());
}
