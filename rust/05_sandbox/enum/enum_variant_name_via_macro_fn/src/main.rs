// TODO: support empty enum?
//TODO: should use {} in macro call/enum def. or keep it with just args and commas ?

// Define the VariantName trait
//trait VariantNameAsStr {
//    fn variant_name_as_str(&self) -> &str;
//}

#[macro_export]
macro_rules! replace_with_2_dots {
    ($($input:tt)*) => {
        ..
    };
}

// https://users.rust-lang.org/t/enum-variant-name-as-str-without-debug-display-proc-macro-or-heap-allocation/111876/
//
#[macro_export]
macro_rules! enum_str {
    ($(#[$attr:meta])*
     $vis:vis enum $name:ident $(<$($gen:ident),*>)?,
     $(
         $variant:ident
             $( (
                 $($tfield:ty),*
                 $(,)?
             ) )?
             $( {
                 $($sfield:ident: $stype:ty),*
                 $(,)?
             } )?
     ),*
     $(,)?
     ) => {
        $(#[$attr])*
        $vis enum $name $(<$($gen),*>)? {
            $(
                $variant $( ( $($tfield),* ) )?
                         $( { $($sfield: $stype),* })?
            ),*
        }//enum

        //impl $(<$($gen),*>)? VariantNameAsStr for $name $(<$($gen),*>)? {
        impl $(<$($gen),*>)? $name $(<$($gen),*>)? {
            pub const fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        // Handle variants with fields
                        Self::$variant $( ( $crate::replace_with_2_dots!( $($tfield),* ) ) )?
                                       $( { $($sfield: _),* } )?
                        => stringify!($variant),
                    )*
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
    NoShie(i32),
    RustAnalyzerHi(i32,i8),
    RustAnalyzerHi2(),
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

pub enum Color0 {
    Red, Green, Blue,
    StructVariant1 {
        field1: i32,
    },
    //Foo{i32},//XXX: not valid
    Foo(i32),
}

enum_str! {
    pub enum Color,
    Red, Green, Blue,
    StructVariant1 {
        field1: i32,
    },
    TupleVariant(i32),
}

enum_str! {
    pub enum Color2<T,G>,
    Tee { f: i32 },
    Red(T,G), Green(G,i32), Blue(i64,i128,),
    Magenta,
    Foo { field1: i32 },
}

fn main() {
    let error1 = MyError::AlreadyBorrowedOrRecursingError {
        source: BorrowMutError,
        location_of_instantiation: LocationInSource,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize,
    };
    assert_eq!(
        error1.variant_name_as_str(),
        "AlreadyBorrowedOrRecursingError"
    );

    let error2 = MyError::TimeoutError {
        location_of_instantiation: LocationInSource,
        duration: Duration,
        tid: 12345,
        custom_message: NoAllocFixedLenMessageOfPreallocatedSize,
    };
    assert_eq!(error2.variant_name_as_str(), "TimeoutError");
    println!("{:?}", error1);
    let c=Color::Blue;
    assert_eq!(c.variant_name_as_str(),"Blue");
    let c2=Color2::<i128,&str>::Green("text",2);
    assert_eq!(c2.variant_name_as_str(),"Green");
}
