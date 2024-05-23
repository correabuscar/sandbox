// Define the VariantName trait
trait VariantNameAsStr {
    fn variant_name_as_str(&self) -> &str;
}
macro_rules! enum_str {
    //XXX: arm matches unit varians like Red and struct variants like Red { field1: i32, field2: i64, }, and a mixture of both is supported!
    ($(#[$attr:meta])* $vis:vis enum $name:ident $(<$($gen:ident),*>)?, $($variant:ident $({ $($field:ident: $ftype:ty),* $(,)? })?),* $(,)?) => {
        $(#[$attr])*
        $vis enum $name $(<$($gen),*>)? {
            $(
                $variant $({ $($field: $ftype),* })?
            ),*
        }//enum

        impl $(<$($gen),*>)? VariantNameAsStr for $name $(<$($gen),*>)? {
            fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        // Handle variants with fields
                        Self::$variant $({ $($field: _),* })? => stringify!($variant),
                        //Self::$variant $({..})? => stringify!($variant),
                    )*
                }
            }//fn
        }//impl
    };

    //XXX: arm matches only tuple variants eg. Red(i32,i64,i128) but not Red, nor Red { field:i32 }, so you can't mix them!
    ($(#[$attr:meta])* $vis:vis enum $name:ident $(<$($gen:ident),*>)?, $($variant:ident $(($($ftype:ty),* $(,)? ))?),* $(,)?) => {
        $(#[$attr])*
            $vis enum $name $(<$($gen),*>)? {
                $(
                    $variant $(($($ftype),*))?,
                )*
            }//enum

        impl $(<$($gen),*>)? VariantNameAsStr for $name $(<$($gen),*>)? {
            fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        Self::$variant(..) => stringify!($variant),
                    )*
                }
            }
        }//impl
    };//arm
}//macro


// Define custom types used in the enum fields
#[derive(Debug)]
pub struct BorrowMutError;

#[derive(Debug)]
pub struct LocationInSource;

#[derive(Debug)]
pub struct Duration;

pub const CUSTOM_ERROR_MSG_BUFFER_SIZE: usize = 256;

#[derive(Debug)]
pub struct NoAllocFixedLenMessageOfPreallocatedSize<const SIZE: usize>;


// Use the macro to declare the enum with visibility
enum_str! {
    #[derive(Debug)]
    pub enum MyError,
    AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        location_of_instantiation: LocationInSource,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>
    },
    TimeoutError {
        location_of_instantiation: LocationInSource,
        duration: Duration,
        tid: u64,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>
    },
    Shie,
    //NoShie(i32),//XXX: this isn't accepted here!
}

enum_str! {
    #[derive(Debug)]
    pub enum MyError2<T,F>,
    AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        location_of_instantiation: T,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>
    },
    TimeoutError {
        location_of_instantiation: LocationInSource,
        duration: F,
        tid: u64,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>
    },
}

enum_str! {
    pub enum Color,
    Red, Green, Blue,
    StructVariant1 {
        field1: i32,
    },
}

enum_str! {
    pub enum Color2<T,G>,
    //Tee { f: i32 }, // if u use this, then the tuple variant below isn't accepted!
    Red(T,G), Green(G,i32), Blue(i64,i128,),
    //Magenta,//XXX: this isn't accepted here!
    //Foo { field1: i32 }, //XXX: this isn't accepted here!
}

fn main() {
    let error1 = MyError::AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        location_of_instantiation: LocationInSource,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize,
    };
    assert_eq!(error1.variant_name_as_str(), "AlreadyBorrowedOrRecursingError");

    let error2 = MyError::TimeoutError {
        location_of_instantiation: LocationInSource,
        duration: Duration,
        tid: 12345,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize,
    };
    assert_eq!(error2.variant_name_as_str(), "TimeoutError");
    println!("{:?}", error1);
}

