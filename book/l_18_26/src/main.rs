#![deny(clippy::pedantic, clippy::all, clippy::correctness, clippy::nursery)]
//^ no effect on: unreachable_patterns
#![deny(warnings)]
//^ this works!

fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    /*    match num {
        Some(x) => println!("{}", x),
        Some(x) if x < 5 => println!("less than five: {}", x), // warning: unreachable pattern
        None => (),
    }*/
}
