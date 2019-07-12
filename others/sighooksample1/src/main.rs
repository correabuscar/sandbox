extern crate libc;
extern crate signal_hook;

use std::io::Error;
use std::thread;

use signal_hook::iterator::Signals;

fn main() -> Result<(), Error> {
    let signals = Signals::new(&[
        signal_hook::SIGUSR1,
        signal_hook::SIGUSR2,
        signal_hook::SIGHUP,
    ])?;
    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                signal_hook::SIGUSR1 => {
                    println!("exiting");
                    break;
                }
                signal_hook::SIGUSR2 => {
                    println!("ignoring {}", signal);
                }
                signal_hook::SIGHUP => {
                    println!("ignoring {}", signal);
                }
                _ => unreachable!(),
            }
        }
    })
    .join()
    .unwrap();
    Ok(())
}
