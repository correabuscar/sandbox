#![feature(const_type_name)]
//#![feature(const_trait_impl)]
//#![feature(stmt_expr_attributes)]
//#![deny(unused_must_use)] // yeah it works here, ofc! but it's too broad!

mod my_error_things {
    include!(concat!(env!("OUT_DIR"), "/project_dir.rs")); //gets me 'PROJECT_DIR'

    //const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 6;
    pub const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 4096; //one kernel page?!
                                                          //
    use std::cell::BorrowMutError;
    use std::fmt;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct LocationInSource {
        file: &'static str,
        line: u32,
        column: u32,
    }

    impl fmt::Display for LocationInSource {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}:{}:{}",
                file_without_prefix(self.file),
                self.line,
                self.column
            )
        }
    }

    impl LocationInSource {
        pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
            Self { file, line, column }
        }
    }

//    #[derive(Debug)]
    pub enum MyError {
        AlreadyBorrowedOrRecursingError {
            source: BorrowMutError,
            //where an instance of this error was created, in source code
            location_of_instantiation: LocationInSource,
            //custom_message: [u8; CUSTOM_ERROR_MSG_BUFFER_SIZE],
            custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<
                CUSTOM_ERROR_MSG_BUFFER_SIZE,
            >,
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
            custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<
                CUSTOM_ERROR_MSG_BUFFER_SIZE,
            >,
        },
    } // enum

    impl MyError {
        pub const fn variant_name_only(&self) -> &str {
            //FIXME: use this macro /home/user/sandbox/rust/05_sandbox/enum/enum_variant_name_via_macro_fn
            match self {
                MyError::AlreadyBorrowedOrRecursingError {..} => "AlreadyBorrowedOrRecursingError",
                MyError::TimeoutError {..} => "TimeoutError",
            }
        }

        pub fn variant_name_full(&self) -> crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE> {
            //let type_name = std::any::type_name::<Self>();/* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            //let type_name= std::any::type_name_of_val(self); /* same ^ */
            let type_name=self.type_name_without_crate();
            //let i:i32=type_name;//&str
            //let type_name_len = type_name.len().min(type_name_buffer.len() - 1);
            //let type_name_slice = &type_name.as_bytes()[..type_name_len];
            //let type_name_str = std::str::from_utf8(type_name_slice).unwrap_or("UnknownType");
            //let variant_name = unsafe {
            //    std::intrinsics::variant_name(std::mem::discriminant(self))
            //};
            //let variant_name= std::any::type_name_of_val(self);
            let variant_name=self.variant_name_only();
            self.as_str();
            //let fixed=crate::format_into_buffer!("{}::{}", type_name, variant_name).get_msg();/* E0716: temporary value dropped while borrowed consider using a `let` binding to create a longer lived value */
            let fixed: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>=crate::format_into_buffer!("{}::{}", type_name, variant_name);
            //let fixed=fixed.get_msg();
            return fixed;
        }

        #[inline(always)]
        pub const fn type_name_full(&self) -> &str {
            std::any::type_name::<Self>()/* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            //std::any::type_name_of_val(self) /* same ^ */
        }

        #[allow(dead_code)]
        pub fn type_name_short(&self) -> &str {
            //"The type name returned by std::any::type_name::<Self>() is a string literal, which is stored in the program's data segment and does not require heap allocation.
            //Substring slicing: The &[start..end] syntax for creating string slices operates on existing memory without allocating new memory. It simply points to a portion of the original string's memory." - chatgpt 3.5
            //ok so no heap allocations and can return substring aka slice because that's in data segment.
            let full_type_name=self.type_name_full();//std::any::type_name::<Self>(); /* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            //std::any::type_name_of_val(self) /* same ^ */
            if let Some(last_double_colon) = full_type_name.rfind("::") {
                &full_type_name[(last_double_colon + 2)..] // Skip the last '::'
            } else {
                full_type_name
            }
        }

        pub fn type_name_without_crate(&self) -> &str {
            let full_type_name=self.type_name_full();//std::any::type_name::<Self>(); /* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            if let Some(first_double_colon) = full_type_name.find("::") {
                &full_type_name[(first_double_colon + 2)..] // Skip the crate prefix
            } else {
                full_type_name
            }
        }
    } // impl

    impl fmt::Debug for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let full=self.variant_name_full();
            let full=full.get_msg();
            match self {
                MyError::AlreadyBorrowedOrRecursingError { source, location_of_instantiation, custom_message } => {
                    f.debug_struct(full)
                        .field("source", source)
                        .field("location_of_instantiation", location_of_instantiation)
                        .field("custom_message", custom_message)
                        .finish()
                },
                MyError::TimeoutError { location_of_instantiation, duration, tid, custom_message } => {
                    f.debug_struct(full)
                        .field("location_of_instantiation", location_of_instantiation)
                        .field("duration", duration)
                        .field("tid", tid)
                        .field("custom_message", custom_message)
                        .finish()
                },
            }
        }
    } // impl

//    // Implement the Error trait
//    impl std::error::Error for MyError {
//        //FIXME: this seems to do some heappage, maybe don't impl. at all?!
//        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//            match self {
//                MyError::AlreadyBorrowedOrRecursingError { source, .. } => Some(source),
//                MyError::TimeoutError { .. } => None,
//            }
//        }
//    }

//    // Implement conversion from BorrowMutError to MyError
//    impl From<std::cell::BorrowMutError> for MyError {
//        fn from(err: std::cell::BorrowMutError) -> Self {
////            MyError::AlreadyBorrowedOrRecursingError(
////                format!("BorrowMutError: {}", err)
////                )
//            //FIXME: since using the macro inside this function, i don't see file:line:column of the caller, thus this is bad, let's not use '?' but map_err() instead, and the '?' after it;
//            //XXX: thus not implementing From trait for our error type will prevent using '?' and "tell" us to use map_err()
//            let borrow_error = crate::my_error!(
//                crate::my_error_things::MyError::AlreadyBorrowedOrRecursingError,
//                crate::format_into_buffer!("Custom borrow error message with error code {}", 404),
//                source: err,
//            );
//            borrow_error
//        }
//    }

    fn file_without_prefix(file: &str) -> &str {
        // Remove the prefix of PROJECT_DIR from the file field
        match file.strip_prefix(PROJECT_DIR) {
            Some(suffix) => suffix,
            None => file,
        }
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                //mehTODO: use for the enum to &str this: https://docs.rs/strum_macros/latest/strum_macros/derive.Display.html  but we don't want to use external crates. So I guess we need to do the macro_rules way, since proc macros need to be in their own crates!
                MyError::AlreadyBorrowedOrRecursingError {
                    location_of_instantiation,
                    custom_message,
                    //custom_message_len,
                    source,
                } => {
                    write!(
                    f,
                    "{} at location: '{}', custom msg:'{}', generic msg: Already borrowed or recursing error, source error: '{}'",
                    self.variant_name_full(),
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
                    write!(f, "{} at location '{}', custom msg: '{}', generic msg: Timeout after {:?} while trying to find a free slot for thread {}.",
                    self.variant_name_full(),
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
} //mod my_error_things
  //
  //#[deny(unused_must_use)] //no effect
mod static_noalloc_msg {

    pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize> {
        msg: [u8; SIZE],
        msg_len: usize,
        //msg_slice:&'static str, //points into the 'msg' buffer - can't be done this way apparently, XXX: rust?!
    }
    impl<const SIZE: usize> std::fmt::Display for NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //let slice=std::str::from_utf8(&self.msg[..self.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
            let slice = self.get_msg();
            write!(f, "{}", slice)
        }
    }

    impl<const SIZE: usize> std::fmt::Debug for NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //let slice=std::str::from_utf8(&self.msg[..self.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
            let slice = self.get_msg();
            //write!(f, "{}", slice)
            //FIXME: use noalloc buffer for the struct name? to not hardcode it in &str
            //FIXME: stringify!(SIZE) was wrong anyway, lol!
            f.debug_struct(concat!("NoAllocFixedLenMessageOfPreallocatedSize<",stringify!(SIZE),">"))
                .field("msg", &slice)
                .field("msg_len", &self.msg_len)
                .finish()
        }
    }

    impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
        pub fn get_msg(&self) -> &str {
            //TODO: maybe use something like https://github.com/rodrimati1992/const_format_crates/  like "concatcp: Concatenates integers, bool, char, and &str constants into a &'static str constant." instead of concat!()
            let slice = std::str::from_utf8(&self.msg[..self.msg_len]).unwrap_or(concat!(
                "<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",
                //stringify!(SIZE),//this is "SIZE" lol
                SIZE, // expected a literal only literals (like `"foo"`, `-42` and `3.14`) can be passed to `concat!()`
                ">>"
            ));
            slice
        }

        pub const fn new(msg: [u8; SIZE], msg_len: usize) -> Self {
            assert!(msg_len <= SIZE);
            //let mut inst=NoAllocFixedLenMessageOfPreallocatedSize {
            NoAllocFixedLenMessageOfPreallocatedSize {
                msg,
                msg_len,
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
            //TODO: don't hardcode this const here CUSTOM_ERROR_MSG_BUFFER_SIZE, allow it to be first arg? but also make a version of this macro with hardcoded arg for the MyError type in its module
        let mut buffer = [0u8; $crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE];//allocated at call site aka destination, due to being itself returned/owned, instead of a ref to it which wouldn't work unless it's a static but then every thread will share same one, even tho it's different for each macro call site!
        let mut cursor = std::io::Cursor::new(&mut buffer[..]);
        //use std::io::Write;
        //let res=write!(cursor, $fmt, $($arg)*);
        //OR: "The write! macro is indeed associated with the std::io::Write trait, and without importing it, you cannot directly use the write! macro even with fully qualified paths. To accomplish what you want without using the use statement, you would need to resort to using the write_fmt method directly" - chatgpt 3.5
        let res = std::io::Write::write_fmt(&mut cursor, format_args!($fmt, $($arg)*));
        /* "In Rust, `format_args!` is a macro that constructs a `fmt::Arguments` object, which represents the format string and its arguments. This macro does not perform any heap allocation by itself. Instead, it constructs the `fmt::Arguments` object entirely on the stack.

The `fmt::Arguments` object can then be passed to various formatting functions, such as `write!`, `writeln!`, `format!`, or `print!`, to perform the actual formatting.

When you use `format_args!`, you're essentially deferring the formatting process until later. This allows you to construct the format string and arguments without immediately performing the formatting operation.

The allocation behavior of `format_args!` itself does not depend on the `Display` implementation of the arguments. Instead, it's the formatting functions (`write!`, `writeln!`, `format!`, `print!`) that determine whether heap allocation occurs.

If any of the arguments passed to the formatting function requires heap allocation during formatting (for example, if it involves a `String` or another type that allocates memory), then the formatting operation may result in heap allocation. Otherwise, if all arguments can be formatted without heap allocation (for example, if they implement `Display` without allocating memory), then no heap allocation will occur during formatting.

So, in summary, `format_args!` itself does not allocate memory on the heap. However, heap allocation may occur during the subsequent formatting operation, depending on the `Display` implementation of the arguments and the specific formatting function used." - chatgpt 3.5 */


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
} //mod static_noalloc_msg

use std::cell::RefCell;
use std::time::Duration;

type MyResult<T> = Result<T, my_error_things::MyError>;

fn some_fn() -> MyResult<()> {
    let r = RefCell::new(0);
    //let inst = r.borrow_mut();/*this won't drop the borrow, even tho it's gonna be shadowed.*/
    let _inst = r.borrow_mut(); /*this won't drop the borrow*/
    //let _ = r.borrow_mut();/*this immediately drops the borrow, so a next borrow mut won't err!*/
    //let _inst = r.try_borrow_mut()?;/*XXX: can't use '?' directly anymore, good */
    let _inst = r.try_borrow_mut().map_err(|err| {
        crate::my_error!(
            crate::my_error_things::MyError::AlreadyBorrowedOrRecursingError,
            crate::format_into_buffer!("Custom borrow error message with error code {}", 404),
            source: err,
        )
    })?;/*XXX: but we can forget to use '?' at the end there!*/
    //let i:i32=_inst;//found `RefMut<'_, {integer}>`


    Ok(())
}

//#[deny(unused_must_use)] // works ofc, because it applies to call sites!
//fn main() {
fn main() -> Result<(), my_error_things::MyError> {
    let res=some_fn();
    let err=res.err().unwrap();
    println!("{}\n======", err);
    //return err;
    let _res=some_fn()?;

    let r = RefCell::new(0);
    //let inst = r.borrow_mut();/*this won't drop the borrow, even tho it's gonna be shadowed.*/
    let _inst = r.borrow_mut(); /*this won't drop the borrow*/
    //let _ = r.borrow_mut();/*this immediately drops the borrow, so a next borrow mut won't err!*/
    let inst = r.try_borrow_mut();

    //format_into_buffer!("Custom borrow error message with error code {}", 404); /*XXX: this gets an unused warning! */
    let borrow_error = my_error!(
        my_error_things::MyError::AlreadyBorrowedOrRecursingError,
        format_into_buffer!("Custom borrow error message with error code {}", 404),
        source: inst.err().unwrap()
    );

    let dur = Duration::new(5, 0);
    let tid = 6;
    let timeout_error = my_error!(
        my_error_things::MyError::TimeoutError,
        format_into_buffer!("Timeout occurred for thread {} after {:?}", tid, dur),
        duration: dur,
        tid: tid,
    );

    println!("{}", borrow_error);
    println!("{}", timeout_error);
    Ok(())
}
