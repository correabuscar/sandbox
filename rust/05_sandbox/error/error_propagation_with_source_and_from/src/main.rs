use std::cell::BorrowMutError;
use std::fmt;
use std::time::Duration;
use std::backtrace::Backtrace;

// Step 1: Define a custom error type
#[derive(Debug)]
enum MyError {
    AlreadyBorrowedOrRecursingError(BorrowMutError, Backtrace),
    TimeoutError(Duration, u64, Backtrace), // Duration and thread ID (tid)
}

impl From<BorrowMutError> for MyError {
    fn from(error: BorrowMutError) -> Self {
        MyError::AlreadyBorrowedOrRecursingError(error, Backtrace::capture())
    }
}


// Step 2: Implement Display for the custom error type
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::AlreadyBorrowedOrRecursingError(_, _) => write!(f, "Already borrowed or recursing error"),
            MyError::TimeoutError(duration, tid, _) => write!(f, "Timeout after {:?} while trying to find a free slot for thread {}", duration, tid),
        }
    }
}

// Step 3: Implement the Error trait
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::AlreadyBorrowedOrRecursingError(e, _) => Some(e),
            MyError::TimeoutError(_, _, _) => None,
        }
    }
}

// Step 4: Define a custom Result type
type MyResult<T> = Result<T, MyError>;

// Example function using the custom Result type
fn example_function() -> MyResult<()> {
    // Simulating an error from RefCell::try_borrow_mut()
    let cell: std::cell::RefCell<i32> = std::cell::RefCell::new(42);
    //let _borrowed = cell.try_borrow_mut().map_err(|e| MyError::AlreadyBorrowedOrRecursingError(e, Backtrace::new()))?;
    let _borrowed = cell.try_borrow_mut()?;
    let _borrowed = cell.try_borrow_mut()?;//XXX: this triggers

    // Simulating a timeout error
    let duration = Duration::new(5, 0);
    let tid = 12345;
    if duration.as_secs() > 0 {
        return Err(MyError::TimeoutError(duration, tid, Backtrace::capture()));
    }

    Ok(())
}

fn log_error_chain(error: Box<dyn std::error::Error>) {
    eprintln!("Error: {}", error);
    let mut source = error.source();
    while let Some(err) = source {
        eprintln!("Caused by: {}", err);
        source = err.source();
    }
    if let Some(my_error) = error.downcast_ref::<MyError>() {
        match my_error {
            MyError::AlreadyBorrowedOrRecursingError(_, backtrace) => eprintln!("Backtrace:\n{}", backtrace),
            MyError::TimeoutError(_, _, backtrace) => eprintln!("Backtrace:\n{}", backtrace),
        }
    }
}


fn main() {
    match example_function() {
        Ok(_) => println!("Operation succeeded."),
        Err(e) => log_error_chain(Box::new(e)),
    }
}

