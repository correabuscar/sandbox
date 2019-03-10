extern crate graceful;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use std::io::{self, Write};

use graceful::SignalGuard;

static STOP: AtomicBool = AtomicBool::new(false);

fn main() {
    let signal_guard = SignalGuard::new();

    let handle = thread::spawn(|| {
        println!("!thread! Worker thread started. Type Ctrl+C to stop.");
        while !STOP.load(Ordering::Acquire) {
            print!("!thread! Sleeping for 5 sec...");
            io::stdout().flush();//lock().flush();
            thread::sleep(Duration::from_millis(5000));
            println!("done.");
        }
        println!("!thread! Bye.");
    });

    signal_guard.at_exit(move |sig| { //atexit libc::atexit at_exit hook (for grep)
        println!("!main! Signal {} received.", sig);
        STOP.store(true, Ordering::Release);
        println!("!main! Waiting for thread(s) to finish");
        handle.join().unwrap();//so wait for all threads to finish on their own! XXX without this, immediately exit!
    });
}
