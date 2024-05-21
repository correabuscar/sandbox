use std::cell::BorrowMutError;
use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::time::Duration;

include!(concat!(env!("OUT_DIR"), "/project_dir.rs")); //gets me 'PROJECT_DIR'


//const BUFFER_SIZE: usize = 6;
const BUFFER_SIZE: usize = 4096;//one kernel page?!

#[derive(Debug)]
enum MyError {
    AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        file: &'static str,
        line: u32,
        column: u32,
        message: [u8; BUFFER_SIZE],
        len: usize,
    },
    TimeoutError {
        duration: Duration,
        tid: u64,
        file: &'static str,
        line: u32,
        column: u32,
        message: [u8; BUFFER_SIZE],
        len: usize,
    },
}

  // Implement the Error trait
  impl std::error::Error for MyError {
      fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
          match self {
              MyError::AlreadyBorrowedOrRecursingError { source, .. } => Some(source),
              MyError::TimeoutError{ .. } => None,
          }
      }
  }

fn file_without_prefix(file:&str) -> &str {
    // Remove the prefix of PROJECT_DIR from the file field
    match file.strip_prefix(PROJECT_DIR) {
        Some(suffix) => suffix,
        None => file,
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::AlreadyBorrowedOrRecursingError {
                file,
                line,
                column,
                message,
                len,
                source,
            } => {
                write!(
                    f,
                    "'{}' at {}:{}:{} - Already borrowed or recursing error, source error: '{}'",
                    //std::any::type_name::<MyError>(),//::<Self::AlreadyBorrowedOrRecursingError>(),//self,
                    //TODO: how to show the variant itself with the type prefixing it too, without duplicating its name inside the string and hopefully without procedural macros?
                    String::from_utf8_lossy(&message[..*len]),
                    file_without_prefix(file),
                    line,
                    column,
                    source
                )
            }
            MyError::TimeoutError {
                file,
                line,
                column,
                message,
                len,
                ..
            } => {
                write!(f, "{} at {}:{}:{} - Timeout after {:?} while trying to find a free slot for thread {}", String::from_utf8_lossy(&message[..*len]),
                    file_without_prefix(file),
                    line, column, Duration::new(0, 0), 0)
            }
        }
    }
}

macro_rules! format_into_buffer {
    ($fmt:expr, $($arg:tt)*) => {{
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut cursor = std::io::Cursor::new(&mut buffer[..]);
        let res=write!(cursor, $fmt, $($arg)*);
        let len = cursor.position() as usize;
        if res.is_err() {
            eprintln!("Failed to write to buffer of size '{}' due to error '{}' (was it due to buffer too small? '{}'), wrote '{}' bytes so far, as '{:?}' or as lossy string: '{}'",BUFFER_SIZE, res.err().unwrap(), len==BUFFER_SIZE,len, buffer, String  ::from_utf8_lossy(&buffer[..len]));
        }
        (buffer, len)
    }};
}
macro_rules! my_error {
    ($variant:path, $message:expr, $($field:ident : $value:expr),*) => {{
        let (buffer, len) = $message;

        $variant {
             file: file!(),
            line: line!(),
            column: column!(),
            message: buffer,
            len: len,
            $($field: $value),*
        }
    }};
}

fn main() {
    let r = RefCell::new(0);
    //let inst = r.borrow_mut();//this won't drop the borrow, even tho it's gonna be shadowed.
    let _inst = r.borrow_mut();//this won't drop the borrow
    //let _ = r.borrow_mut();//this immediately drops the borrow, so a next borrow mut won't err!
    let inst = r.try_borrow_mut();

    let borrow_error = my_error!(
        MyError::AlreadyBorrowedOrRecursingError,
        format_into_buffer!("Custom borrow error message with error code {}", 404),
        source: inst.err().unwrap()
    );

    let timeout_error = my_error!(
        MyError::TimeoutError,
        format_into_buffer!("Timeout occurred for thread {} after {:?}", 1234, Duration::new(5, 0)),
        duration: Duration::new(5, 0),
        tid: 1234
    );

    println!("{}", borrow_error);
    println!("{}", timeout_error);
}

