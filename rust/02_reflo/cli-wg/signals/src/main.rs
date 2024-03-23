// file:///home/user/build/2nonpkgs/rust.stuff/cli-wg/book/in-depth/signals.html
use signal_hook::{iterator::Signals, SIGABRT, SIGINT};
use std::{error::Error, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[
        SIGINT, SIGABRT, // XXX: can catch this from `kill -6 $pid` tho.
    ])?;

    let first = thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            if sig == 2 {
                // C-c
                break;
            }
        }
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    let second = thread::spawn(move || {
        //std::process::abort(); //can't catch this TODO:
    });
    println!("{:#?}", first.join().unwrap());
    println!("{:#?}", second.join().unwrap());
    println!("Main says bye.");

    Ok(())
}
