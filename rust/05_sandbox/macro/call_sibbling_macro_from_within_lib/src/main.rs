use call_sibbling_macro_from_within_lib::{foo,bar};

macro_rules! bar {
    () => {
        println!("Oh Helloooo.");
    }
}

fn main() {
    //call_sibbling_macro_from_within_lib::foo!();
    foo!();
}
