// file:///home/user/build/2nonpkgs/rust.stuff/cli-wg/book/in-depth/signals.html
use std::{error::Error, thread};
use signal_hook::{iterator::Signals, SIGINT};

fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            if sig == 2 { // C-c
                break;
            }
        }
    }).join().unwrap();

    Ok(())
}
