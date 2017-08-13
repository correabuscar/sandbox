//#[cfg(feature = "debug-assertions")] //no effect
#[cfg(debug_assertions)] //thanks to Arnavion on irc!
const MODE: &'static str = //XXX: this repetition is necessary
"--debug";
#[cfg(not(debug_assertions))]
const MODE: &'static str = //XXX: this repetition is necessary
"--release";

fn main() {
    println!("Hello, world! {}", MODE);
}
