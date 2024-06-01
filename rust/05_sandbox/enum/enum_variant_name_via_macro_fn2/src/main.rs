// nvmTODO: support empty enum?  Foo {}, actuall not a good idea! no point and would only clutter the 'match' with a default arm for every enum!
//
//doneTODO: should use {} in macro call/enum def. or keep it with just args and commas ? the {} version allows for commenting out the macro call and its end brance to keep the raw enum!
//doneTODO: handle enum with constants: Variant = val
// Define the VariantName trait
//trait VariantNameAsStr {
//    fn variant_name_as_str(&self) -> &str;
//}

//#[macro_export]
//macro_rules! replace_with_2_dots {
//    ($($input:tt)*) => {
//        ..
//    };
//}

// TODO: can merge this with the main macro actually!
/// unconditionally places the first arg
#[macro_export]
macro_rules! place_first_arg_ignore_rest_if_any {
    ($mandatory:tt $(, $optional:tt)* $(,)?) => {
        $mandatory
    };
}

///// just puts first arg
//#[macro_export]
//macro_rules! replace_with2 {
//    (
//        $mandatory:tt | $($optional:tt),* $(,)?
//     ) => {
//        $mandatory
//    };
//}

// https://users.rust-lang.org/t/enum-variant-name-as-str-without-debug-display-proc-macro-or-heap-allocation/111876/
//
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


// Define custom types used in the enum fields
#[derive(Debug)]
pub struct BorrowMutError;

#[derive(Debug)]
pub struct LocationInSource;

#[derive(Debug)]
pub struct Duration;

pub const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 256;

mod static_noalloc_msg {
    #[derive(Debug)]
    pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize>;
}

// Use the macro to declare the enum with visibility
enum_str! {
        #[derive(Debug)]
        /// wtw
          pub enum MyError {
        /// wtw
          AlreadyBorrowedOrRecursingError {
        /// wtw
              source: BorrowMutError,
              //where an instance of this error was created, in source code
        /// wtw
              location_of_instantiation: LocationInSource,
              //custom_message: [u8; CUSTOM_ERROR_MSG_BUFFER_SIZE],
        /// wtw
              custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<
                  CUSTOM_ERROR_MSG_BUFFER_SIZE,
              >,
              //custom_message: &'static str,
              //custom_message_len: usize,
          },
        /// wtw
          TimeoutError {
              location_of_instantiation: LocationInSource,
              #[allow(dead_code)]
              duration: Duration,
              #[allow(dead_code)]
              tid: u64,
              //custom_message: [u8; CUSTOM_ERROR_MSG_BUFFER_SIZE],
              //custom_message_len: usize,
              //custom_message: &'static str,
              //custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize< { crate::my_error_t  hings::CUSTOM_ERROR_MSG_BUFFER_SIZE } >,
              custom_message: crate::static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize<
                  CUSTOM_ERROR_MSG_BUFFER_SIZE,
              >,
          },
      } // enum
}


fn main() {
    let error1 = MyError::AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        location_of_instantiation: LocationInSource,
        custom_message: static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize,
    };
    assert_eq!(
        error1.variant_name_as_str(),
        "AlreadyBorrowedOrRecursingError"
    );

    let error2 = MyError::TimeoutError {
        location_of_instantiation: LocationInSource,
        duration: Duration,
        tid: 12345,
        custom_message: static_noalloc_msg::NoAllocFixedLenMessageOfPreallocatedSize,
    };
    assert_eq!(error2.variant_name_as_str(), "TimeoutError");
    println!("{:?}", error1);
}
