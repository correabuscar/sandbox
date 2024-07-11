use std::env;
use std::iter::Iterator;

fn main() {
    // Get the command-line arguments as an iterator
    let args = env::args();
    let program_name=args.next();

    // Skip the first argument (the program's name)
    //let args = args.skip(1);

    // Apply a filter to keep only arguments that are not empty
    let filtered_args = args.clone().filter(|arg| !arg.is_empty());

    // Iterate over the filtered arguments and print them
    for arg in filtered_args {
        println!("Argument: {}", arg);
    }
}

