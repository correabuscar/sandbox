// Define a custom trait for providing default values
trait MyDefault {
    fn default() -> Self;
}

// Define a struct with named fields representing function arguments
struct GreetingArgs {
    name: String,
    greeting: String,
    punctuation: Option<String>,
}

// Implement the custom trait for the struct
impl MyDefault for GreetingArgs {
    fn default() -> Self {
        GreetingArgs {
            name: String::new(),
            greeting: String::from("Hello"),
            punctuation: None,
        }
    }
}

// Define the greet function
fn greet(args: GreetingArgs) {
    // Use pattern matching to handle optional parameters
    let punctuation_str = args.punctuation.unwrap_or_else(|| String::from("!"));

    // Print the greeting
    println!("{} {}{} How are you?", args.greeting, args.name, punctuation_str);
}

fn main() {
    // Create an instance of the struct with default values
    //let default_args = MyDefault::my_default();

    // Create an instance of the struct with some fields overridden
    greet( GreetingArgs {
        name: String::from("Alice"),
        ..MyDefault::default() // Use default values for other fields
    });

    // Call the greet function with instances of the struct
    //greet(default_args); // Using default values for all fields
    //greet(alice_args);   // Overriding the name field
}

