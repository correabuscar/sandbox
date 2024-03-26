// Define a struct with named fields representing function arguments
//#[derive(Default)]
struct GreetingArgs {
    name: String,
    greeting: String,
}

// Manually implement the Default trait for the struct
impl Default for GreetingArgs {
    fn default() -> Self {
        GreetingArgs {
            name: String::new(),
            greeting: String::from("Hello"),
        }
    }
}

// Define the greet function
fn greet(args: GreetingArgs) {

    // Print the greeting
    println!("{} {}. How are you?", args.greeting, args.name);
}

fn main() {
    // Create an instance of the struct with default values
    //let default_args = MyDefault::my_default();

    // Create an instance of the struct with some fields overridden
    greet( GreetingArgs {
        name: String::from("Alice"),
        ..Default::default() // Use default values for other fields
    });

    // Call the greet function with instances of the struct
    //greet(default_args); // Using default values for all fields
    //greet(alice_args);   // Overriding the name field
}

