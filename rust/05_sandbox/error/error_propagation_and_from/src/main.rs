use std::cell::RefCell;

// Define your own error type
#[derive(Debug)]
enum MyError {
    #[allow(dead_code)]
    RefCellError(String),
    // Add other error variants as needed
}

// Implement conversion from BorrowMutError to MyError
impl From<std::cell::BorrowMutError> for MyError {
    fn from(err: std::cell::BorrowMutError) -> Self {
        MyError::RefCellError(format!("BorrowMutError: {}", err))
    }
}

// Define a thread-local storage for recursion depth
thread_local! {
    static RECURSION_DEPTH: RefCell<usize> = RefCell::new(0);
}

fn recursive_function() -> Result<(), MyError> {
    // Attempt to borrow mutably, handle potential borrow errors
    RECURSION_DEPTH.with(|depth| {
        // Borrow mutably or return RefCellError
        let mut borrow = depth.try_borrow_mut()?; //XXX: this error propagation, uses that From impl above

        // Increment recursion depth
        *borrow += 1;

        // Simulate some work
        println!("Recursion depth: {}", *borrow);

        // Simulate recursion
        if *borrow < 5 {
            // Recurse
            recursive_function()?;
        }

        // Decrement recursion depth
        *borrow -= 1;

        // If depth is zero, we're done with recursion
        if *borrow == 0 {
            return Ok(());
        }

        // Simulate more work
        println!("Continuing with recursion depth: {}", *borrow);

        Ok(())
    })
}

fn main() {
    match recursive_function() {
        Ok(()) => println!("Recursion completed successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}

