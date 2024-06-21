#![feature(const_type_name)]
#![feature(const_mut_refs)] // Enable mutable references in const functions
#![feature(const_trait_impl)] // const impl
#![feature(effects)]
//#![feature(generic_const_exprs)] // warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes

//#![feature(const_trait_impl)]
//#![feature(stmt_expr_attributes)]
//#![deny(unused_must_use)] // yeah it works here, ofc! but it's too broad!

mod some_macro {
    /// matches any variants that are: unit, tuple or struct like, with/without discriminant (ie. Ok=200)
    /// though you'd have to add eg. #[repr(u8)] //else, error[E0732]: `#[repr(inttype)]` must be specified
    /// empty enums are not supported, like pub enum Foo{}, on purpose!
    /// see: https://doc.rust-lang.org/reference/items/enumerations.html
    #[macro_export]
    macro_rules! enum_str {
    (
        //matches attributes like #[allow(dead_code)], if any!
        //or the more likely-used one: #[repr(u8)]
        //or doc comments like /// text, which are #[doc = " text"]
        $( #[ $enum_outer_attr:meta ] )* // this is enum's outer attribute
                                         // meta: an Attr, the contents of an attribute

        //matches 'pub enum Something<T,G,F>' but also just 'enum Something'
        $visibility:vis
        // vis: a possibly empty Visibility qualifier
        // aka it can be missing, or has an implied $()? wrapper!

        //Enumeration : `enum` IDENTIFIER  GenericParams? WhereClause? `{` EnumItems? `}`
        enum $name:ident
        //https://doc.rust-lang.org/reference/items/generics.html
        // GenericParams : `<` `>` | `<` (GenericParam `,`)* GenericParam `,`? `>`
        $(< $($generics_simple:ident),* > )?
        { //enum's opening brace

        // EnumItems : EnumItem ( `,` EnumItem )* `,`?
        $(
            // EnumItem : OuterAttribute* Visibility? IDENTIFIER ( EnumItemTuple | EnumItemStruct )? EnumItemDiscriminant?
            // OuterAttribute*
            $(
                #[ $enumitem_outer_attr:meta ]
            )*
            // Visibility?
            //$(
                $enumitem_visibility:vis
                // vis: a possibly empty Visibility qualifier, so $()? isn't needed(and in fact an error!)
            //)?
            //matches VariantName, VariantName(), VariantName(i32), VariantName(i32,i128,)
            //but also weirds like: VariantName(,)
            //also matches VariantName {}, VariantName { f:i32, }, VariantName { f:i32, g: u8, },
            //but also weirds like: StructVariantOops1 {,}

            // IDENTIFIER
            $variant:ident //aka enumitem

            // ( EnumItemTuple | EnumItemStruct )?
            // XXX: that | aka OR, can't be achieved this way(not just here), so the invalid version of both(or none) at same time could match here:
            //$(
                // EnumItemTuple : `(` TupleFields? `)`
                $( (
                        // TupleFields?
                        //$(
                            // TupleFields : TupleField (`,` TupleField)* `,`?
                            $(
                                // TupleField : OuterAttribute* Visibility? Type

                                // OuterAttribute*
                                $(
                                    #[ $enumitem_tuple_field_outer_attr:meta ]
                                )*
                                // Visibility?
                                //$(
                                $enumitem_tuple_field_visibility:vis
                                // vis: a possibly empty Visibility qualifier, so $()? isn't needed(and in fact an error!)
                                //)?
                                // Type
                                $enumitem_tuple_field:ty
                            ),*
                            // `,`?
                            $(,)?
                        //)?
                ) )?

                // EnumItemStruct : `{` StructFields? `}`
                $( {
                    // StructFields?
                    //$(
                        // StructFields : StructField (`,` StructField)* `,`?
                        $(
                            // StructField : OuterAttribute* Visibility? IDENTIFIER `:` Type
                            // OuterAttribute*
                            $(
                                #[ $enumitem_struct_field_outer_attr:meta ]
                            )*
                            // Visibility?
                            //$(
                            $enumitem_tuple_struct_visibility:vis
                            // vis: a possibly empty Visibility qualifier, so $()? isn't needed(and in fact an error!)
                            //)?
                            // IDENTIFIER `:` Type
                            $enumitem_struct_field_ident:ident : $enumitem_struct_field_type:ty
                        ),*
                        // `,`?
                        $(,)?
                    //)?
                } )?
            //)? //can't include this parent $()? block because the two inner ones are already $()? aka can be empty
            // EnumItemDiscriminant?
            $(
                // EnumItemDiscriminant : `=` Expression
                = $enumitem_discriminant:expr
            )?
        ),*
        $(,)? //don't care if lone comma can be matched too, better to not dup the matcher block.
        } //enum's closing brace
    ) => {
        $(#[$enum_outer_attr])*
            $visibility enum $name
            $(<$($generics_simple),*>)?
        {
            $(
                $( #[ $enumitem_outer_attr ] )*
                $variant
                $( ( $($enumitem_tuple_field),* ) )?
                $( { $($enumitem_struct_field_ident: $enumitem_struct_field_type),* } )?
                $( = $enumitem_discriminant )?
            ),*
        }//enum

        impl
            $(<$($generics_simple),*>)?
            $name
            $(<$($generics_simple),*>)?
            {
                pub const fn variant_name_as_str(&self) -> &str {
                    match self {
                        $(
                            Self::$variant
                            // XXX: so {..} can be used for any enum variant!
                            { .. }
                            => stringify!($variant),
                        )*
                            //                        #[allow(unreachable_patterns)]
                            //                        _ => {
                            //                            //that {{}} is actually expanded to: {} aka escaped, and this panic!() works in 'const fn' as opposed to unreachable!()
                            //                            panic!("Unreachable! This was only needed in case of empty enum like: enum Foo {{}}, because we can't conditionally not include the whole impl based on $variant due to macro saying it's already repeating at this depth");
//                        }
                    }//match
                }//fn
            }//impl
    };//arm
} //macro
}

mod my_error_things {

    //const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 6;
    pub const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 4096; // one kernel page?!
    //^ must be pub due to being used in a macro!
    // nvm fixed by patch /patches/portage/dev-lang/rust.reused/2100_rustfmt_single_line_comments_left_space_behind.patch //old://space before ^ is required for rustfmt(aka cargo fmt) to not 'error[internal]: left behind trailing whitespace' see: https://github.com/rust-lang/rustfmt/issues/6157#issuecomment-2096598378 https://github.com/rust-lang/rustfmt/issues/5391#issuecomment-1159932006

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
            write!(f, "{}:{}:{}", self.file, self.line, self.column)
        }
    }

    impl LocationInSource {
        pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
            Self { file, line, column }
        }
    }

    crate::enum_str! {
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
    } //macro

    impl MyError {
        //TODO: the size of the returned here shouldn't need to be same as CUSTOM_ERROR_MSG_BUFFER_SIZE it can be different/less!
        pub fn variant_name_full(
            &self,
        ) -> crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<
            CUSTOM_ERROR_MSG_BUFFER_SIZE,
        > {
            //let type_name = std::any::type_name::<Self>();/* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            //let type_name= std::any::type_name_of_val(self); /* same ^ */
            let type_name = self.type_name_without_crate();
            //let i:i32=type_name;//&str
            //let type_name_len = type_name.len().min(type_name_buffer.len() - 1);
            //let type_name_slice = &type_name.as_bytes()[..type_name_len];
            //let type_name_str = std::str::from_utf8(type_name_slice).unwrap_or("UnknownType");
            //let variant_name = unsafe {
            //    std::intrinsics::variant_name(std::mem::discriminant(self))
            //};
            //let variant_name= std::any::type_name_of_val(self);
            let variant_name = self.variant_name_as_str(); //made by enum_str! macro
            //self.as_str();
            //let fixed=crate::format_into_buffer!("{}::{}", type_name, variant_name).get_msg();/* E0716: temporary value dropped while borrowed consider using a `let` binding to create a longer lived value */
            let fixed: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<
                CUSTOM_ERROR_MSG_BUFFER_SIZE,
            > = crate::format_into_buffer!(
                CUSTOM_ERROR_MSG_BUFFER_SIZE,
                "{}::{}",
                type_name,
                variant_name
            );
            //let fixed=fixed.get_msg();
            return fixed;
        }

        #[inline(always)]
        pub const fn type_name_full(&self) -> &str {
            //both need this: #![feature(const_type_name)]
            std::any::type_name::<Self>() /* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            //std::any::type_name_of_val(self) /* same ^ */
        }

        #[allow(dead_code)]
        pub fn type_name_short(&self) -> &str {
            //"The type name returned by std::any::type_name::<Self>() is a string literal, which is stored in the program's data segment and does not require heap allocation.
            //Substring slicing: The &[start..end] syntax for creating string slices operates on existing memory without allocating new memory. It simply points to a portion of the original string's memory." - chatgpt 3.5
            //ok so no heap allocations and can return substring aka slice because that's in data segment.
            let full_type_name = self.type_name_full(); //std::any::type_name::<Self>(); /* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            //std::any::type_name_of_val(self) /* same ^ */
            if let Some(last_double_colon) = full_type_name.rfind("::") {
                &full_type_name[(last_double_colon + 2)..] // Skip the last '::'
            } else {
                full_type_name
            }
        }

        pub fn type_name_without_crate(&self) -> &str {
            let full_type_name = self.type_name_full(); //std::any::type_name::<Self>(); /* error_propagation_with_own_msg_and_location::my_error_things::MyError */
            if let Some(first_double_colon) = full_type_name.find("::") {
                &full_type_name[(first_double_colon + 2)..] // Skip the crate prefix
            } else {
                full_type_name
            }
        }
    } // impl

    impl fmt::Debug for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let full = self.variant_name_full();
            //let full=full.get_msg();
            //let mut err_msg_buf=crate::static_noalloc_msg::ErrMessage{ buffer:[0u8; crate::static_noalloc_msg::err_msg_max_buffer_size(CUSTOM_ERROR_MSG_BUFFER_SIZE)], len:0 }; // field 'len' is private
            let mut err_msg_buf = crate::static_noalloc_msg::ErrMessage::<
                {
                    crate::static_noalloc_msg::err_msg_max_buffer_size(CUSTOM_ERROR_MSG_BUFFER_SIZE)
                },
            >::new(); //{ buffer:[0u8; crate::static_noalloc_msg::err_msg_max_buffer_size(CUSTOM_ERROR_MSG_BUFFER_SIZE)], len:0 }; // field 'len' is private
            //let err=&full.get_msg_as_lossy();//kindadoneTODO: this is proper ugly but can't get it only when it errors, LOL!
            //const ERR:&str = &crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize::<0>::get_msg_as_lossy();
            //let err=&crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize::<0>::get_msg_as_lossy();
            let full_as_str: &str = full.get_msg_as_str_maybe().unwrap_or_else(|_e| {
                //TODO: use 'e'
                //err
                full.append_msg_to_dest_as_lossy(&mut err_msg_buf);
                err_msg_buf.as_str()
                //ref to temp value:
                //&crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize::<0>::get_msg_as_lossy()
            });
            match self {
                MyError::AlreadyBorrowedOrRecursingError {
                    source,
                    location_of_instantiation,
                    custom_message,
                } => f
                    .debug_struct(full_as_str)
                    .field("source", source)
                    .field("location_of_instantiation", location_of_instantiation)
                    .field("custom_message", custom_message)
                    .finish(),
                MyError::TimeoutError {
                    location_of_instantiation,
                    duration,
                    tid,
                    custom_message,
                } => f
                    .debug_struct(full_as_str)
                    .field("location_of_instantiation", location_of_instantiation)
                    .field("duration", duration)
                    .field("tid", tid)
                    .field("custom_message", custom_message)
                    .finish(),
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
    //            //FIXME: since using the macro inside this function, i don't see file:line:column of the caller, thus this is bad(NOTE: coulda maybe used #[track_caller] here and     println!("{}", std::panic::Location::caller()); https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute ), so:
    //            //XXX: due to ^ let's not use '?' but map_err() instead, and the '?' after it;
    //            //XXX: thus not implementing From trait for our error type will prevent using '?' and "tell" us to use map_err()
    //            let borrow_error = crate::my_error!(
    //                crate::my_error_things::MyError::AlreadyBorrowedOrRecursingError,
    //                crate::format_into_buffer!("Custom borrow error message with error code {}", 404),
    //                source: err,
    //            );
    //            borrow_error
    //        }
    //    }

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
                        "{} at location: '{}', custom msg: '{}', generic msg: Already borrowed or recursing error, source error: '{}'",
                        self.variant_name_full(),
                        //std::any::type_name::<MyError>(),//::<Self::AlreadyBorrowedOrRecursingError>(),//self,
                        //doneTODO: how to show the variant itself with the type prefixing it too, without duplicating its name inside the string and hopefully without procedural macros?
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
                    write!(
                        f,
                        "{} at location '{}', custom msg: '{}', generic msg: Timeout after {:?} while trying to find a free slot for thread {}.",
                        self.variant_name_full(),
                        //String::from_utf8_lossy(&custom_message[..*custom_message_len]),
                        location_of_instantiation,
                        custom_message,
                        Duration::new(0, 0),
                        0
                    )
                }
            }
        }
    }

    /// use this as second arg to my_error!() macro
    #[macro_export]
    macro_rules! formatted_msg {
        ($fmt:expr, $($arg:tt)*) => {
            //$crate::my_error_things::format_into_buffer!($crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,
            $crate::format_into_buffer!($crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,
                $fmt, $($arg)*)
        }
    }

    /// the second arg(ie. the message) can be a macro call to formatted_msg!() macro.
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
    //    impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
    //        const fn foo() -> usize {
    //            return SIZE;
    //        }
    //    }
    impl<const SIZE: usize> std::fmt::Display for NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //where [(); err_msg_max_buffer_size(SIZE)]: {
            //let slice=std::str::from_utf8(&self.msg[..self.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
            //let slice = self.get_msg();
            let result = self.get_msg_as_str_maybe();
            //            .unwrap_or_else(|e| {
            //                //doneTODO: use 'e'
            //                &self.get_msg_as_lossy()
            //            });
            match result {
                Ok(slice) => write!(f, "{}", slice),
                Err(err) => {
                    //const FOO:usize=SIZE; //can't use generic parameters from outer item: use of generic parameter from outer item
                    let mut err_msg_buf = ErrMessage::< { err_msg_max_buffer_size(4096) } >::new(); //ErrMessage{ the_buffer:[0u8; err_msg_max_buffer_size(4096)], len:0 }; // unconstrained generic constant FIXME: temp used fixed value 4096 there! so it compiles!
                    self.append_msg_to_dest_as_lossy(&mut err_msg_buf);
                    let s: &str = err_msg_buf.as_str();
                    write!(f, "<{} actual err: {}>", s, err)
                }
            } //match
        } //fn
    } //impl

    impl<const SIZE: usize> std::fmt::Debug for NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //let slice=std::str::from_utf8(&self.msg[..self.len]).unwrap_or(concat!("<invalid UTF-8 in this instance of NoAllocFixedLenMessageOfPreallocatedSize::<",stringify!(SIZE),">>"));
            let mut err_msg_buf = ErrMessage::< { err_msg_max_buffer_size(4096) } >::new(); //ErrMessage{ the_buffer:[0u8; err_msg_max_buffer_size(4096)], the_buf_len:0 }; // unconstrained generic constant FIXME: temp used fixed value 4096 there! so it compiles!
            let slice: &str = self.get_msg_as_str_maybe().unwrap_or_else(|_e| {
                //TODO: show 'e' too?
                //doneTODO: show the msg as lossy? can't it needs heap, unless... find another way to do it on stack?
                self.append_msg_to_dest_as_lossy(&mut err_msg_buf);
                let err: &str = err_msg_buf.as_str();
                err
            });
            //write!(f, "{}", slice)
            //uhmFIXME: use noalloc buffer for the struct name? to not hardcode it in &str, or macro_rules!
            //cantFIXME: stringify!(SIZE) was wrong anyway, lol!
            f.debug_struct(Self::get_name_of_self().as_str())
                //                concat!(stringify!(NoAllocFixedLenMessageOfPreallocatedSize),"::<",
                //                    //stringify!(SIZE),
                //                    ">"))
                .field("msg", &slice)
                .field("msg_len", &self.msg_len)
                .finish()
        }
    } //impl
    //macro_rules! foo {
    //    ($($e:tt),*) => {
    //        concat!( $( $e ),*)
    //        }
    //}

    pub const fn err_msg_max_buffer_size(msg_size: usize) -> usize {
        //doneFIXME: this needs to be 4096+whatever extras I added! well SIZE+extras actually
        msg_size + 80 + 4 // Arbitrary size, should be enough for the err_message, if too low it fails compile time, so just increase it then, but 80+length of SIZE as &str is enough
    }

    pub const fn self_name_max_buffer_size() -> usize {
        2 + 42 + 4 // Arbitrary size, should be enough for the err_message, if too low it fails compile time, so just increase it then, but 42+length of SIZE as &str is enough
    }

    pub struct ErrMessage<const BUFFER_SIZE: usize> {
        the_buffer: [u8; BUFFER_SIZE],
        the_buf_len: usize,
    }

    //deref as in the "&" in "&instance", altho it should be the * in *instance
    //XXX: don't impl Defer/deref due to the possibility of accidentally deref-ing when not intended!
    //    impl<const BUFFER_SIZE: usize> std::ops::Deref for ErrMessage<BUFFER_SIZE> {
    //        type Target = str;
    //
    //        fn deref(&self) -> &Self::Target {
    //            // Safety: We assume that the buffer contains valid UTF-8 data up to `self.len`.
    //            //unsafe { std::str::from_utf8_unchecked(&self.buffer[..self.len]) }
    //            //can't make it lossy because it needs String aka heap
    //            std::str::from_utf8(&self.buffer[..self.len]).unwrap_or_else(|_e| {
    //                //TODO: use 'e' but how?
    //                //XXX: shouldn't happen, unless I missed something; like everywhere this is used, source things are UTF-8
    //                concat!("<invalid UTF-8 in ", stringify!(ErrMessage), " instance>")
    //            })
    //        }
    //    }//impl

    impl<const BUFFER_SIZE: usize> ErrMessage<BUFFER_SIZE> {
        //yeTODO: can this(or a newly named one) be made 'const fn' ?
        pub const fn as_str<'a>(&'a self) -> &'a str {
            //XXX: E0015: cannot call non-const fn ... it's official .unwrap_or*() are crap shortcuts ;p
            //std::str::from_utf8(&self.the_buffer[..self.the_buf_len]).unwrap_or(
            //    concat!("<invalid UTF-8 in ", stringify!(ErrMessage), " instance>")
            //);
            //And this would be why:
            /*
error: method has missing const stability attribute
    --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1407:5
     |
1407 | /     pub const fn unwrap_or(self, default: T) -> T {
1408 | |         match self {
1409 | |             Ok(t) => t,
1410 | |             Err(_) => default,
1411 | |         }
1412 | |     }
     | |_____^

error: method has missing const stability attribute
    --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1428:5
     |
1428 | /     pub const fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
1429 | |         match self {
1430 | |             Ok(t) => t,
1431 | |             Err(e) => op(e),
1432 | |         }
1433 | |     }
     | |_____^

error[E0493]: destructor of `T` cannot be evaluated at compile-time
    --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1407:34
     |
1407 |     pub const fn unwrap_or(self, default: T) -> T {
     |                                  ^^^^^^^ the destructor for this type cannot be evaluated in constant functions

error[E0493]: destructor of `result::Result<T, E>` cannot be evaluated at compile-time
    --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1407:28
     |
1407 |     pub const fn unwrap_or(self, default: T) -> T {
     |                            ^^^^ the destructor for this type cannot be evaluated in constant functions

error[E0493]: destructor of `F` cannot be evaluated at compile-time
    --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1428:58
     |
1428 |     pub const fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
     |                                                          ^^ the destructor for this type cannot be evaluated in constant functions

For more information about this error, try `rustc --explain E0493`.
             */
            let res=
            std::str::from_utf8(&self.the_buffer[..self.the_buf_len]);
            match res {
                Err(_e) => {
            //.unwrap_or_else(|_e| {
                //TODO: use 'e' but how?
                //XXX: shouldn't happen, unless I missed something; like everywhere this is used, source things are UTF-8
                concat!("<invalid UTF-8 in ", stringify!(ErrMessage), " instance>")
                },
                Ok(s) => s,
                //TODO: so what about return a type that has len+fixed size array, instead of &str, for both match arms here; or an enum type like Cow is? but then have to kinda handle both cases at call site, unless type has a method to return str like as_str heh.
            //})
            } //match
        }
        pub const fn new() -> ErrMessage<BUFFER_SIZE> {
            //
            //ErrMessage{ buffer:[0u8; err_msg_max_buffer_size(4096)], len:0 } //nvmFIXME: used temp const 4096; won't work must be BUFFER_SIZE sized array!
            ErrMessage {
                the_buffer: [0u8; BUFFER_SIZE],
                the_buf_len: 0,
            } //must be BUFFER_SIZE
        }
        //        const fn get_as_array(&self) -> &[u8; BUFFER_SIZE] {
        //            &self.buffer
        //        }
        ///
        // that buffer param there, needs: #![feature(const_mut_refs)] // Enable mutable references in const functions
        pub const fn append_size_as_str(&mut self, input_size: usize) {
            const DIGITS_LEN: usize = log10(usize::MAX); //eg. 20; maximum number of digits for a usize

            let mut digits = [b'0'; DIGITS_LEN];
            let mut num = input_size;
            let mut index = DIGITS_LEN;
            let start = self.the_buf_len;

            // Skip leading zeroes
            while num > 0 {
                index -= 1;
                digits[index] = b'0' + (num % 10) as u8;
                num /= 10;
            }

            // If the number is zero, just return a single zero
            if index == DIGITS_LEN - 1 {
                self.the_buffer[start] = b'0';
                self.the_buf_len += 1;
                return;
            }

            // Copy the digits starting from the non-zero part into the buffer
            let mut len = 0;
            let mut i = index;
            while i < DIGITS_LEN {
                self.the_buffer[start + len] = digits[i]; //if this fails with index outta bounds, u don't have enough space in buffer!
                len += 1;
                i += 1;
            }
            self.the_buf_len += len;
        } //fn

        pub const fn append(&mut self, input_bytes: &[u8]) {
            let bytes_len = input_bytes.len();
            //        XXX: can't properly err from this! because 'const fn'
            let start_at = self.the_buf_len;
            let have_space = self.the_buffer.len() - start_at;
            if have_space < bytes_len {
                [()][bytes_len]; // XXX: reports the value of this
                [()][have_space];
                //            //panic!("foo");
                //            //panic!("You have {have_space} bytes in buffer but you need {bytes_len}, so {} more bytes.",bytes_len - have_space);
                //            //panic!("{}",format_args!("You have {} bytes in buffer but you need {}, so {} more bytes.",have_space, bytes_len, bytes_len - have_space));
                //            //panic!("{}",crate::format_into_buffer!("You have {} bytes in buffer but you need {}, so {} more bytes.",have_space, bytes_len, bytes_len - have_space));
                //            //let foo=crate::format_into_buffer!("You have {} bytes in buffer but you need {}, so {} more bytes.",have_space, bytes_len, bytes_len - have_space);
                //            //panic!("not enough space left");
                assert!(have_space >= bytes_len, "don't have space to append");
            }
            let mut j = 0;

            while j < bytes_len {
                self.the_buffer[start_at + j] = input_bytes[j]; // if out of bounds it means err_msg_max_buffer_size() is too low, like if u're using too big of a SIZE
                j += 1;
            }

            self.the_buf_len = start_at + bytes_len;
        } //fn

        pub const fn append_from_utf8_lossy(&mut self, input: &[u8]) {
            const REPLACEMENT: &[u8] = b"\xEF\xBF\xBD"; // UTF-8 for replacement character U+FFFD
            const REPL_LEN: usize = REPLACEMENT.len();

            //let mut buffer = [0u8; 1024];
            let start_at_in_buffer: usize = self.the_buf_len;
            let mut len: usize = start_at_in_buffer;
            let input_len = input.len();
            if len + input_len > BUFFER_SIZE {
                //XXX: not bad to see the value of, but it only shows me the first one that fails, due to runtime panic on first!
                [()][len + input_len]; //4203
                [()][len]; //107
                [()][input_len]; //4096
                [()][BUFFER_SIZE]; //4180

                //try1:
                //let _=std::panic::catch_unwind(||
                //[()][len]; //);
                //let _ = [1][len] + [2][input_len] + [3][BUFFER_SIZE];

                //try2: fail!
                //struct ValueIsTooLarge<const N: usize>;
                //let _ = ValueIsTooLarge::<len>; //won't work at runtime, and 'len' needs to be const
                assert!(len + input_len <= BUFFER_SIZE, "won't fit"); // can't format it in 'const fn', tried this: "it wouldn't fit, needs: {} but have {}.", len+input_len, BUFFER_SIZE);
            }
            //assert_ne!(len + input_len, BUFFER_SIZE);// 'assert' works but 'assert_ne' no way!

            let mut i = 0;
            while i < input_len {
                //&& len < BUFFER_SIZE {
                if len >= BUFFER_SIZE {
                    [()][len]; // XXX: show me the value
                    assert!(len < BUFFER_SIZE, "no more space left in dest buffer0");
                }
                let result = std::str::from_utf8(&input[i..]);
                match result {
                    Ok(valid) => {
                        let valid_bytes = valid.as_bytes();
                        let vb_len = valid_bytes.len();
                        //                        if len + vb_len > BUFFER_SIZE {
                        //                            break;
                        //                        }
                        if len + vb_len >= BUFFER_SIZE {
                            [()][len]; // XXX: show me the value
                            [()][vb_len]; // XXX: show me the value
                            assert!(len + vb_len < BUFFER_SIZE, "shouldn't go past last index");
                        }
                        let mut j = 0;
                        while j < vb_len {
                            if len >= BUFFER_SIZE {
                                [()][len]; // XXX: show me the value
                                assert!(len < BUFFER_SIZE, "no more space left in dest buffer1");
                            }
                            self.the_buffer[len] = valid_bytes[j];
                            len += 1;
                            j += 1;
                        }
                        break;
                    }
                    Err(e) => {
                        let valid_up_to = e.valid_up_to();
                        let invalid_sequence_length = match e.error_len() {
                            Some(len) => len,
                            None => 1,
                        };

                        let mut j = 0;
                        while j < valid_up_to {
                            if len >= BUFFER_SIZE {
                                [()][len]; // XXX: show me the value
                                assert!(len < BUFFER_SIZE, "no more space left in dest buffer2");
                            }
                            self.the_buffer[len] = input[i + j];
                            len += 1;
                            j += 1;
                        } //while

                        let mut k = 0;
                        while k < REPL_LEN {
                            if len >= BUFFER_SIZE {
                                [()][len]; // XXX: show me the value
                                assert!(
                                    len < BUFFER_SIZE,
                                    "can't insert replacement char due to not enough space in destination buffer"
                                );
                            }
                            //if len < BUFFER_SIZE {
                            self.the_buffer[len] = REPLACEMENT[k];
                            len += 1;
                            //} else {
                            //    break;
                            //}
                            k += 1;
                        } //while

                        i += valid_up_to + invalid_sequence_length;
                    }
                } //match
            } //while

            self.the_buf_len = len;
            //ErrMessage { buffer, len }
        } //fn
    } //impl

    pub const fn log10(n: usize) -> usize {
        let mut count = 0;
        let mut tmp = n;
        while tmp >= 10 {
            tmp /= 10;
            count += 1;
        }
        count + 1
    }

    // that buffer param there, needs: #![feature(const_mut_refs)] // Enable mutable references in const functions
    const fn size_to_str(size: usize, buffer: &mut [u8], start: usize) -> usize {
        const DIGITS_LEN: usize = log10(usize::MAX); //eg. 20; maximum number of digits for a usize

        let mut digits = [b'0'; DIGITS_LEN];
        let mut num = size;
        let mut index = DIGITS_LEN;

        // Skip leading zeroes
        while num > 0 {
            index -= 1;
            digits[index] = b'0' + (num % 10) as u8;
            num /= 10;
        }

        // If the number is zero, just return a single zero
        if index == DIGITS_LEN - 1 {
            buffer[start] = b'0';
            return 1;
        }

        // Copy the digits starting from the non-zero part into the buffer
        let mut len = 0;
        let mut i = index;
        while i < DIGITS_LEN {
            buffer[start + len] = digits[i]; //if this fails with index outta bounds, u don't have enough space in buffer!
            len += 1;
            i += 1;
        }
        len
    }

    const fn copy_to_buf(buf: &mut [u8], start_at: usize, bytes: &[u8]) -> usize {
        let bytes_len = bytes.len();
        //        XXX: can't properly err from this! because 'const fn'
        //        let have_space=buf.len()-start_at;
        //        if have_space < bytes_len {
        //            //panic!("foo");
        //            //panic!("You have {have_space} bytes in buffer but you need {bytes_len}, so {} more bytes.",bytes_len - have_space);
        //            //panic!("{}",format_args!("You have {} bytes in buffer but you need {}, so {} more bytes.",have_space, bytes_len, bytes_len - have_space));
        //            //panic!("{}",crate::format_into_buffer!("You have {} bytes in buffer but you need {}, so {} more bytes.",have_space, bytes_len, bytes_len - have_space));
        //            //let foo=crate::format_into_buffer!("You have {} bytes in buffer but you need {}, so {} more bytes.",have_space, bytes_len, bytes_len - have_space);
        //            //panic!("not enough space left");
        //        }
        let mut j = 0;

        while j < bytes_len {
            buf[start_at + j] = bytes[j]; // if out of bounds it means err_msg_max_buffer_size() is too low, like if u're using too big of a SIZE
            j += 1;
        }

        start_at + bytes_len
    }

    const fn copy_to_buf_2(
        buf: &mut [u8],
        start_at: usize,
        bytes: &[u8],
        bytes_len: usize,
    ) -> usize {
        let mut j = 0;

        while j < bytes_len {
            buf[start_at + j] = bytes[j]; // if out of bounds it means err_msg_max_buffer_size() is too low, like if u're using too big of a SIZE
            j += 1;
        }

        start_at + bytes_len
    }

    pub const fn from_utf8_lossy(input: &[u8]) -> ErrMessage<1024> {
        const REPLACEMENT: &[u8] = b"\xEF\xBF\xBD"; // UTF-8 for replacement character U+FFFD
        let mut buffer = [0u8; 1024];
        let mut len = 0;

        let mut i = 0;
        while i < input.len() && len < buffer.len() {
            let result = std::str::from_utf8(&input[i..]);
            match result {
                Ok(valid) => {
                    let valid_bytes = valid.as_bytes();
                    if len + valid_bytes.len() > buffer.len() {
                        break;
                    }
                    let mut j = 0;
                    while j < valid_bytes.len() {
                        buffer[len] = valid_bytes[j];
                        len += 1;
                        j += 1;
                    }
                    break;
                }
                Err(e) => {
                    let valid_up_to = e.valid_up_to();
                    let invalid_sequence_length = match e.error_len() {
                        Some(len) => len,
                        None => 1,
                    };

                    let mut j = 0;
                    while j < valid_up_to {
                        buffer[len] = input[i + j];
                        len += 1;
                        j += 1;
                    } //while

                    let mut k = 0;
                    while k < REPLACEMENT.len() {
                        if len < buffer.len() {
                            buffer[len] = REPLACEMENT[k];
                            len += 1;
                        } else {
                            break;
                        }
                        k += 1;
                    } //while

                    i += valid_up_to + invalid_sequence_length;
                }
            } //match
        } //while

        ErrMessage {
            the_buffer: buffer,
            the_buf_len: len,
        }
    }

    impl<const SIZE: usize> NoAllocFixedLenMessageOfPreallocatedSize<SIZE> {
        pub const fn get_name_of_self() -> ErrMessage<{ self_name_max_buffer_size() }> {
            //let mut buffer = [0u8; self_name_max_buffer_size()];
            //let mut len = 0;
            let mut ret = ErrMessage {
                the_buffer: [0u8; self_name_max_buffer_size()],
                the_buf_len: 0,
            };

            const PART1: &[u8] = stringify!(NoAllocFixedLenMessageOfPreallocatedSize).as_bytes();
            const PART2: &[u8] = b"::<";
            const PART3: &[u8] = b">";

            //len = copy_to_buf(&mut buffer, len, PART1);
            ret.append(PART1);
            //len = copy_to_buf(&mut buffer, len, PART2);
            ret.append(PART2);
            //len += size_to_str(SIZE, &mut buffer, len);
            //ret.len += size_to_str(SIZE, &mut ret.buffer, ret.len);
            ret.append_size_as_str(SIZE);
            //len = copy_to_buf(&mut buffer, len, PART3);
            ret.append(PART3);

            ret
        }

        //pub const fn append_msg_as_lossy(&self, dest: &mut ErrMessage<{ err_msg_max_buffer_size(SIZE) }>) {
        pub const fn append_msg_to_dest_as_lossy<const ANY_SIZE: usize>(
            &self,
            dest: &mut ErrMessage<ANY_SIZE>,
        ) {
            //let mut buffer = [0u8; err_msg_max_buffer_size()];
            //let mut len = 0;
            //let mut ret=ErrMessage { buffer:[0u8; err_msg_max_buffer_size()], len:0 };

            const PART1: &[u8] = b"<invalid UTF-8 in this instance of ";
            const PART2: &[u8] = stringify!(NoAllocFixedLenMessageOfPreallocatedSize).as_bytes();
            //assert!(PART2.len() == 40);
            const PART3: &[u8] = b"::<";
            assert!(PART3.len() == 3); //just to show that they've a size that matches the assignment.
            const PART4: &[u8] = b"> but here it is lossily: \"";
            const PART5: &[u8] = b"\">";
            assert!(PART5.len() == 2);

            //len = copy_to_buf(&mut buffer, len, PART1);
            dest.append(PART1);
            //len = copy_to_buf(&mut buffer, len, PART2);
            dest.append(PART2);
            //len = copy_to_buf(&mut buffer, len, PART3);
            dest.append(PART3);
            //len += size_to_str(SIZE, &mut buffer, len);
            //dest.len += size_to_str(SIZE, &mut dest.buffer, dest.len);
            dest.append_size_as_str(SIZE);
            //len = copy_to_buf(&mut buffer, len, PART4);
            dest.append(PART4);
            //okFIXME: this is messy, and double copies; maybe make it place it in buf directly!
            //let foo=from_utf8_lossy(&self.msg[..self.msg_len]);
            //dest.from_utf8_lossy_to_buf(&self.msg[..self.msg_len], dest.len);
            dest.append_from_utf8_lossy(&self.msg[..self.msg_len]);
            //len = copy_to_buf_2(&mut buffer, len, &foo.buffer, foo.len);
            //dest.len = copy_to_buf(&mut dest.buffer, dest.len, PART5);
            dest.append(PART5);
            //dest
        }

        pub const fn get_msg_as_str_maybe(&self) -> Result<&str, std::str::Utf8Error> {
            //itisprocmacrosonoTODO: maybe use something like https://github.com/rodrimati1992/const_format_crates/  like "concatcp: Concatenates integers, bool, char, and &str constants into a &'static str constant." instead of concat!()
            //okTODO: use this /home/user/sandbox/rust/05_sandbox/strings/concat_strings_on_stack/concat_strings_and_a_num_const or better: /home/user/sandbox/rust/05_sandbox/strings/concat_strings_on_stack/concat_strings_and_a_num_const_at_compiletime
            //eprintln!("{:?}", &self.msg[..self.msg_len]);
            //let s =
            std::str::from_utf8(&self.msg[..self.msg_len])
            //;
            //eprintln!("{}", s.unwrap());
            //s
            //                let foo=self.get_msg_as_lossy();
            //                &foo
            //            }
            //                concat!(
            //                "<invalid UTF-8 in this instance of ",
            //                stringify!(NoAllocFixedLenMessageOfPreallocatedSize),
            //                "::<",
            //                //stringify!(SIZE),//this is "SIZE" lol
            //                //wtwFIXME: can't put SIZE here without proc macros looks like
            //                //SIZE, // expected a literal only literals (like `"foo"`, `-42` and `3.14`) can be passed to `concat!()`
            //                ">>"
            //            )
            //            );
            //            slice
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
    ($buffer_size_const:expr, $fmt:expr, $($arg:tt)*) => {
        //#[must_use] // unused_attributes: `#[must_use]` has no effect when applied to an expression `#[warn(unused_attributes)]` on by default
        //#[deny(unused_must_use)] //no effect here
        {
            //doneTODO: don't hardcode this const here CUSTOM_ERROR_MSG_BUFFER_SIZE, allow it to be first arg? but also make a version of this macro with hardcoded arg for the MyError type in its module
            //const fn check_usize(val: usize) -> usize {
            //    //FIXME: detect values over CUSTOM_ERROR_MSG_BUFFER_SIZE and warn or compile error or something!
            //    val
            //}
            //check_usize($buffer_size_const);

            //const fn const_min(a: usize, b: usize) -> usize {
            //    if a <= b { a } else { b }
            //}
        //const LESSER_ONE:usize=const_min($buffer_size_const, $crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE);
        const BUFFER_SIZE:usize=$buffer_size_const;//coerce/ensure it's const and usize!
        let mut buffer = [0u8; BUFFER_SIZE];//allocated at call site aka destination, due to being itself returned/owned, instead of a ref to it which wouldn't work unless it's a static but then every thread will share same one, even tho it's different for each macro call site!
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
        let ret_type=$crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize::< { BUFFER_SIZE } >::new(buffer, len);
        if res.is_err() {
            eprintln!("Failed to write to buffer of size '{}' due to error '{}' (was it due to buffer too small? '{}'), wrote '{}' bytes so far, as '{:?}' or as rust UTF-8 string: '{}'",
            //$crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,
            BUFFER_SIZE,
            res.err().unwrap(),
            //len==$crate::my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,
            len==BUFFER_SIZE,
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
        //fn ensure_used<T>(t: T) -> T { t }
        //#[deny(unused_must_use)] //XXX: has no effect here! but rather, only at call sites!(which this may seem like one, but doesn't count if it's in macro, it has to be at macro call site!)
        ensure_used(ret_type) /*XXX: if u see this, it's an error at macro invocation site, not inside the macro! you've to use the return of the macro!*/
        //it errors here: ^ unused_must_use: use `let _ = ...` to ignore the resulting value: `let _ = `, `;`

        //ret_type
    }};
    } //macro
} //mod static_noalloc_msg

use std::cell::RefCell;
use std::time::Duration;

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Implement custom allocation logic here
        static ALREADY_BEING_HERE:AtomicBool=AtomicBool::new(false);
        if ! ALREADY_BEING_HERE.load(Ordering::Relaxed) {
            eprintln!("!! before alloc, size={}",layout.size());
            if PANIC_ON_ALLOC.load(Ordering::Relaxed) {
                // since panic!() will alloc
                match ALREADY_BEING_HERE.compare_exchange(false,true,Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(prev) => {
                        assert_eq!(false, prev);
                        //XXX: using panic here is UB, as per docs:
                        //"It’s undefined behavior if global allocators unwind. This restriction may be lifted in the future, but currently a panic from any of these functions may lead to memory unsafety." - https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety
                        //but if we wanna see the stacktrace then... panic
                        panic!("allocation detected when it shouldn't have allocated anymore!");
                        // this panic deadlocks in cleanup() of stdio due to STDOUT.get_or_init() ah, it's because the realloc below gets triggered and we didn't also panic in it! which would detect a double panic and abort instead of deadlock.
                        // sure maybe panic shouldn't be called from the allocator, but still, the type of STDOUT seems off.
                        // XXX: should use:
                        #[allow(unreachable_code)] //FIXME: why does this not silence the warning?!
                        {
                        std::alloc::handle_alloc_error(layout); //this doesn't return, but acts as if we returned null ptr, so no deadlocks!
                        }
                        //put it back, in case we decide to comment out the panic!() call!
                        #[allow(unreachable_code)]
                        {
                        let _ = ALREADY_BEING_HERE.compare_exchange(true,false,Ordering::Relaxed, Ordering::Relaxed);
                        }
                    },
                    Err(prev) => {
                        assert_eq!(true, prev);
                    },
                }
            }
        }
        // Delegating to System allocator for actual allocation
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        static HAPPENED_ONCE_ALREADY:AtomicBool=AtomicBool::new(false);
        if ! HAPPENED_ONCE_ALREADY.load(Ordering::Relaxed) {
        // Implement custom deallocation logic here
        // Delegating to System allocator for actual deallocation
            eprintln!("!! before dealloc, size={}",layout.size());
            match HAPPENED_ONCE_ALREADY.compare_exchange(false,true,Ordering::Relaxed, Ordering::Relaxed) {
                Ok(prev) => {
                    assert_eq!(false, prev);
                    eprintln!("!! further deallocs ignored to avoid spam");
                },
                Err(prev) => {
                    assert_eq!(true, prev);
                },
            }
        }
        System.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        static ALREADY_BEING_HERE:AtomicBool=AtomicBool::new(false);
        if ! ALREADY_BEING_HERE.load(Ordering::Relaxed) {
            eprintln!("!! before alloc_zeroed, size={}",layout.size());
            if PANIC_ON_ALLOC.load(Ordering::Relaxed) {
                // since panic!() will alloc
                match ALREADY_BEING_HERE.compare_exchange(false,true,Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(prev) => {
                        assert_eq!(false, prev);
                        panic!("allocation(zeroed) detected when it shouldn't have allocated anymore!");
                        //put it back, in case we decide to comment out the panic!() call!
                        #[allow(unreachable_code)]
                        {
                        let _ = ALREADY_BEING_HERE.compare_exchange(true,false,Ordering::Relaxed, Ordering::Relaxed);
                        }
                    },
                    Err(prev) => {
                        assert_eq!(true, prev);
                    },
                }
            }
        }
        // Delegating to System allocator for actual allocation
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        static ALREADY_BEING_HERE:AtomicBool=AtomicBool::new(false);
        if ! ALREADY_BEING_HERE.load(Ordering::Relaxed) {
            eprintln!("!! before realloc, oldsize={} newsize={}",layout.size(), new_size);
            match ALREADY_BEING_HERE.compare_exchange(false,true,Ordering::Relaxed, Ordering::Relaxed) {
                Ok(prev) => {
                    assert_eq!(false, prev);
                    eprintln!("REallocation detected when it shouldn't have allocated anymore! Further reallocs are ignored to avoid spam.");
                    //panic!("REallocation detected when it shouldn't have allocated anymore!");

                    //put it back (to enable spam)
                    //let _=ALREADY_BEING_HERE.compare_exchange(true,false,Ordering::Relaxed, Ordering::Relaxed);
                },
                Err(prev) => {
                    assert_eq!(true, prev);
                },
            }
        }
        System.realloc(ptr, layout, new_size)
    }

/*    // error[E0407]: method `dealloc_excess` is not a member of trait `GlobalAlloc`
    fn dealloc_excess(&self, ptr: *mut u8, layout: Layout, new_size: usize) {
        System.dealloc_excess(ptr, layout, new_size)
    }

    fn alloc_layout(&self, layout: Layout) -> Result<*mut u8, alloc::AllocError> {
        System.alloc_layout(layout)
    }

    fn realloc_layout(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> Result<*mut u8, alloc::AllocError> {
        System.realloc_layout(ptr, layout, new_size)
    }
*/
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MyAllocator = MyAllocator;

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
            //crate::format_into_buffer!(my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,"Custom ·\u{b7}borrow error message with error code {}", 404),
            crate::formatted_msg!("Custom ·\u{b7}borrow error message with error code {}", 404),
            source: err,
        )
    })?; /*XXX: but we can forget to use '?' at the end there!*/
    //let i:i32=_inst;//found `RefMut<'_, {integer}>`

    Ok(())
}

static PANIC_ON_ALLOC:AtomicBool=AtomicBool::new(false);

//#[deny(unused_must_use)] // works ofc, because it applies to call sites!
//fn main() {
fn main() -> Result<(), my_error_things::MyError> {
    //println!("sup"); //allocs 1k and flushes the output too (because it ends with newline)
    PANIC_ON_ALLOC.store(true, Ordering::Relaxed);//from now on, panic on any memory allocations!
    print!("sup!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");//this allocates on first use a buffer(of 1k) for stdout.
    //let mut _vec = Vec::<i32>::with_capacity(200);

    let res = some_fn();
    let err = res.err().unwrap();
    println!("{}\n======", err);
    //return err;
    let _res = some_fn()?;

    let r = RefCell::new(0);
    //let inst = r.borrow_mut();/*this won't drop the borrow, even tho it's gonna be shadowed.*/
    let _inst = r.borrow_mut(); /*this won't drop the borrow*/
    //let _ = r.borrow_mut();/*this immediately drops the borrow, so a next borrow mut won't err!*/
    let inst = r.try_borrow_mut();

    //format_into_buffer!(1,"Custom borrow error message with error code {}", 404); /*XXX: this gets an unused warning! */
    let borrow_error = my_error!(
        my_error_things::MyError::AlreadyBorrowedOrRecursingError,
        crate::formatted_msg!("Custom borrow error message with error code {}", 404),
        //format_into_buffer!(my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,"Custom borrow error message with error code {}", 404),
        source: inst.err().unwrap()
    );

    let dur = Duration::new(5, 0);
    let tid = 6;
    let timeout_error = my_error!(
        my_error_things::MyError::TimeoutError,
        formatted_msg!("Timeout occurred for thread {} after {:?}", tid, dur),
        //format_into_buffer!(my_error_things::CUSTOM_ERROR_MSG_BUFFER_SIZE,"Timeout occurred for thread {} after {:?}", tid, dur),
        duration: dur,
        tid: tid,
    );

    println!("{}", borrow_error);
    println!("{}", timeout_error);
    Ok(())
}
