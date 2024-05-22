//#![feature(stmt_expr_attributes)]
//#![deny(unused_must_use)] // yeah it works here, ofc! but it's too broad!




mod my_error_things {
include!(concat!(env!("OUT_DIR"), "/project_dir.rs")); //gets me 'PROJECT_DIR'

//const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 6;
pub const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 4096;//one kernel page?!
                                                 //
use std::fmt;
use std::cell::BorrowMutError;
use std::time::Duration;

#[derive(Debug)]
pub struct LocationInSource {
    file: &'static str,
    line: u32,
    column: u32,
}

impl fmt::Display for LocationInSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}",
            file_without_prefix(self.file),
            self.line, self.column)
    }
}

impl LocationInSource {
    pub const fn new(file:&'static str, line: u32, column: u32) -> Self {
        Self {
            file, line, column
        }
    }

}

#[derive(Debug)]
pub enum MyError {
    AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        //where an instance of this error was created, in source code
        location_of_instantiation: LocationInSource,
        //custom_message: [u8; CUSTOM_ERROR_MSG_BUFFER_SIZE],
        custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>,
        //custom_message: &'static str,
        //custom_message_len: usize,
    },
    TimeoutError {
        location_of_instantiation: LocationInSource,
        #[allow(dead_code)]
        duration: Duration,
        #[allow(dead_code)]
        tid: u64,
        //custom_message: [u8; CUSTOM_ERROR_MSG_BUFFER_SIZE],
        //custom_message_len: usize,
        //custom_message: &'static str,
        //custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize< { crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE } >,
        custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>,
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
                location_of_instantiation,
                custom_message,
                //custom_message_len,
                source,
            } => {
                write!(
                    f,
                    "MyError::AlreadyBorrowedOrRecursingError at location: '{}', custom msg:'{}', generic msg: Already borrowed or recursing error, source error: '{}'",
                    //std::any::type_name::<MyError>(),//::<Self::AlreadyBorrowedOrRecursingError>(),//self,
                    //TODO: how to show the variant itself with the type prefixing it too, without duplicating its name inside the string and hopefully without procedural macros?
                    //String::from_utf8_lossy(&custom_message[..*custom_message_len]),
                    location_of_instantiation,
                    custom_message,
                    source
                )
            }
            MyError::TimeoutError {
                location_of_instantiation,
                custom_message,
                //custom_message_len,
                ..
            } => {
                write!(f, "MyError::TimeoutError at location '{}', custom msg: '{}', generic msg: Timeout after {:?} while trying to find a free slot for thread {}.",
                    //String::from_utf8_lossy(&custom_message[..*custom_message_len]),
                    location_of_instantiation,
                    custom_message,
                    Duration::new(0, 0), 0)
            }
        }
    }
}

#[macro_export]
macro_rules! my_error {
    ($variant:path, $message:expr, $($field:ident : $value:expr),* $(,)?) => {{
        let location_of_instantiation=$crate::my_error_things::LocationInSource::new(
            file!(),
            line!(),
            column!(),
            );
        $variant {
            location_of_instantiation,
            custom_message: $message,
            //custom_message_len: len,
            $($field: $value),*
        }
    }};
}


} //mod
  //
//#[deny(unused_must_use)] //no effect
mod static_noalloc_msg {

#[derive(Debug)]
pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize> {
    msg: [u8; SIZE],
    len: usize,
    //msg_slice:&'static str, //points into the 'msg' buffer - can't be done this way apparently, XXX: rust?!
}
impl<const SIZE: usize> std::fmt::Display for NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let slice=std::str::from_utf8(&self.msg[..self.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
        let slice=self.get_msg();
        write!(f,"{}",slice)
    }
}

impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
    pub fn get_msg(&self) -> &str {
        let slice=std::str::from_utf8(&self.msg[..self.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
        slice
    }
    pub const fn new(msg: [u8; SIZE], len: usize) -> Self {
        //let mut inst=NoAllocFixedLenMessageOfPreallocatedSize {
        NoAllocFixedLenMessageOfPreallocatedSize {
            msg,
            len,
            //msg_slice: "",
        }
//        //let msg_slice=std::str::from_utf8(&inst.msg[..inst.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
//        //inst.msg_slice=msg_slice;
//        //inst
//        assert!(len <= SIZE);
//        let mut ret=NoAllocFixedLenMessageOfPreallocatedSize {
//            msg,
//            len,
//            msg_slice: "",
//        };
//        ret.update_slice()
//    }
//
//    fn update_slice(mut self) -> Self {
//        let msg_slice=std::str::from_utf8(&self.msg[..self.len])
//            .unwrap_or(
//                concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
//        //self.msg_slice=msg_slice;
//        Self {
//            msg_slice,
//            ..self
//        }
//    }
//    fn get_msg_as_slice(&self) -> &'static str {
//        &self.msg_slice
    }
} // impl

/// format the args (like println!()) into the returned pre-allocated buffer
//#[deny(unused_must_use)] // no effect!
#[macro_export]
macro_rules! format_into_buffer {
    ($fmt:expr, $($arg:tt)*) => {
        //#[must_use] // unused_attributes: `#[must_use]` has no effect when applied to an expression `#[warn(unused_attributes)]` on by default
        //#[deny(unused_must_use)] //no effect here
        {
        let mut buffer = [0u8; $crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE];//allocated at call site aka destination, due to being itself returned/owned, instead of a ref to it which wouldn't work unless it's a static but then every thread will share same one, even tho it's different for each macro call site!
        let mut cursor = std::io::Cursor::new(&mut buffer[..]);
        //use std::io::Write;
        //let res=write!(cursor, $fmt, $($arg)*);
        //OR: "The write! macro is indeed associated with the std::io::Write trait, and without importing it, you cannot directly use the write! macro even with fully qualified paths. To accomplish what you want without using the use statement, you would need to resort to using the write_fmt method directly" - chatgpt 3.5
        let res = std::io::Write::write_fmt(&mut cursor, format_args!($fmt, $($arg)*));

        let len = cursor.position() as usize;
        //let ret_slice:&'static str=std::str::from_utf8(&buffer[..len]).unwrap_or("<invalid UTF-8>");
        let ret_type=$crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize::< { $crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE } >::new(buffer, len);
        if res.is_err() {
            eprintln!("Failed to write to buffer of size '{}' due to error '{}' (was it due to buffer too small? '{}'), wrote '{}' bytes so far, as '{:?}' or as rust UTF-8 string: '{}'",
            $crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,
            res.err().unwrap(),
            len==$crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,
            len,
            buffer,
                //String::from_utf8_lossy(&buffer[..len]) //XXX:this is on heap!
                //std::str::from_utf8(&buffer[..len]).unwrap_or("<invalid UTF-8>") //good, no heap!
         //       ret_slice,
            ret_type,
                );
        }
        //#[deny(unused_must_use)] // no effect
        #[must_use] // can't apply it to the expression 'ret_type'
        #[inline(always)]
        fn ensure_used<T>(t: T) -> T { t }
        //#[deny(unused_must_use)] //XXX: has no effect here! but rather, only at call sites!
        ensure_used(ret_type) /*XXX: if u see this, it's an error at macro invocation site, not inside the macro! you've to use the return of the macro!*/
        //it errors here: ^ unused_must_use: use `let _ = ...` to ignore the resulting value: `let _ = `, `;`

        //ret_type
    }};
} //macro

} //mod


use std::cell::RefCell;
use std::time::Duration;

//#[deny(unused_must_use)] // works ofc, because it applies to call sites!
fn main() {
    let r = RefCell::new(0);
    //let inst = r.borrow_mut();//this won't drop the borrow, even tho it's gonna be shadowed.
    let _inst = r.borrow_mut();//this won't drop the borrow
    //let _ = r.borrow_mut();//this immediately drops the borrow, so a next borrow mut won't err!
    let inst = r.try_borrow_mut();

    //format_into_buffer!("Custom borrow error message with error code {}", 404); //XXX: this gets an unused warning!
    let borrow_error = my_error!(
        my_error_things::MyError::AlreadyBorrowedOrRecursingError,
        format_into_buffer!("Custom borrow error message with error code {}", 404),
        source: inst.err().unwrap()
    );
    //let borrow_error = my_error!(
    //    my_error_things::MyError::AlreadyBorrowedOrRecursingError,
    //    format!("Custom borrow error message with error code {}", 404),
    //    source: inst.err().unwrap()
    //);

    let dur=Duration::new(5, 0);
    let tid=6;
    let timeout_error = my_error!(
        my_error_things::MyError::TimeoutError,
        format_into_buffer!("Timeout occurred for thread {} after {:?}", tid, dur),
        duration: dur,
        tid: tid,
    );

    println!("{}", borrow_error);
    println!("{}", timeout_error);
}

