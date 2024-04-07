use std::sync::{Arc, Mutex};
use std::any::type_name;
use lazy_static::lazy_static;

// Define a struct to hold your hook data
struct HookTracker {
    list: Arc<Mutex<Vec<&'static str>>>,
}

impl HookTracker {
    // Function to register that a hook has started executing
    fn started_executing<F>(&self, func: F)
    where
        F: Fn(),
    {
        // Get the name of the function as a string
        let func_name = type_name::<F>();

        // Lock the list to ensure exclusive access
        let mut guard = self.list.lock().expect("Unexpected concurrent execution attempted");

        // Add the name of the function to the list
        guard.push(func_name);

        // Print the name of the function
        println!("Executing {}", func_name);
    }

    // Function to print the names of executed functions
    fn print_executed_hooks(&self) {
        // Lock the list to ensure exclusive access
        let guard = self.list.lock().expect("Failed to acquire lock.");

        println!("Here's what executed so far:");
        // Print the names of executed functions
        for (index, func_name) in guard.iter().enumerate() {
            println!("Function {}: {}", index + 1, func_name);
        }
    }
}

// Define your functions
fn func1() {
    HOOK_TRACKER.started_executing(func1);
    // Your func1 implementation here
}

fn func2() {
    HOOK_TRACKER.started_executing(func2);
    // Your func2 implementation here
}

// Implement func3, func4, func5, func6 in a similar manner

// Create a static instance of HookTracker using lazy_static
lazy_static! {
    static ref HOOK_TRACKER: HookTracker = HookTracker {
        list: Arc::new(Mutex::new(Vec::new())),
    };
}

fn main() {
    // Execute your functions in the expected order
    func1();
    func2();
    // Call func3, func4, func5, func6 in the expected order

    // Print the list of executed functions
    HOOK_TRACKER.print_executed_hooks();
}

