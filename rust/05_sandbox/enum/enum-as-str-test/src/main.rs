#![allow(non_camel_case_types)]
/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![cfg_attr(feature="parse-generics-poc", feature(plugin))]
#![cfg_attr(feature="parse-generics-poc", plugin(parse_generics_poc))]
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_as_str;
#[macro_use] extern crate parse_generics_shim;
#[macro_use] extern crate parse_macros;

custom_derive! {
    #[allow(dead_code)]
    #[derive(enum_as_str)]
    enum Dagashi {
        Umaibou,
        PotatoFries,
        CoffeeMilk,
        YoungDonuts,
        KinakoBou,
        NamaikiBeer,
        FueRamune,
        Menko,
    }
}

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
//enum_str! {
//FIXME: "no rules expected the token `record` no rules expected this token in macro call"
custom_derive! {
    #[allow(dead_code)]
    #[derive(enum_as_str)]
    #[derive(Debug)]
    #[warn(dead_code)]
    /// doc comment
    /// ^ works because it's #[doc = " doc comment"] in actuality, aka outer attribute(s)
    /// second
    pub enum MyError {
        /// doc comment2
        AlreadyBorrowedOrRecursingError {
            source: BorrowMutError,
            location_of_instantiation: LocationInSource,
            custom_message: NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>
        },
        /// doc comment3
        TimeoutError {
            location_of_instantiation: LocationInSource,
            duration: Duration,
            tid: u64,
            custom_message: NoAllocFixedLenMessageOfPreallocatedSize<CUSTOM_ERROR_MSG_BUFFER_SIZE>
        },
        /// doc comment4
        Shie,
        /// doc comment5
        NoShie(i32),
        /// doc comment6
        RustAnalyzerHi(i32,i8),
        /// doc comment7
        RustAnalyzerHi2(),
    }
}

//enum_str! {
//FIXME: "no rules expected the token `record` no rules expected this token in macro call"
custom_derive! {
    #[allow(dead_code)]
    #[derive(enum_as_str)]
    #[derive(Debug)]
    pub enum MyError2<T,F> {
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
}

pub enum Color0 {
    Red, Green, Blue,
    StructVariant1 {
        field1: i32,
    },
    //Foo{i32},//XXX: not valid
    Foo(i32),
}

//enum_str! {
custom_derive! {
    #[allow(dead_code)]
    #[derive(enum_as_str)]
    #[allow(dead_code)]
    #[repr(u8)] //error[E0732]: `#[repr(inttype)]` must be specified
    enum Color {
        Red, Green, Blue,
        StructVariant0 {},
        //StructVariantOops1 {,}, //XXX: good, it won't allow this!
        StructVariant1 {
            field1: i32,
        },
        StructVariant2 {
            field1: i32,
            field2: i32,
        }=100,//yes this should work  FIXME
        TupleVariant(i32)=101,//so this FIXME
        //Ooops(,),//goodFIXME: shouldn't match but hey, the macro itself is a hack!
        //Oops2(){},//it errors but it's not clear why it does, unless u know it's bad syntax.
        UnitWithConstant = 200, //FIXME: the =200 isn't supported by macro. //XXX: due to mixed with other variant types, it requires #[repr(u8)] //else: error[E0732]: `#[repr(inttype)]` must be specified
    }
}

//enum_str! {
    pub enum MyEnum<'a, 'b> where 'a: 'b {
        Variant1(&'a str),
        Variant2(&'b str),
    }
//}
// Example trait to include T::Item if necessary
trait MyTrait {
    type Item;
}

// Implementing MyTrait for demonstration purposes
impl MyTrait for i32 {
    type Item = i32;
}
//enum_str! {
custom_derive! {
    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Color2<T,G>
    where T:Copy, G: Clone,
        //T::Item: Copy,          // Bound on an associated type: "You cannot directly place trait bounds on associated types (T::Item: Copy) within the where clause of an enum definition. Instead, you should place such bounds in the context where you use the enum. If you need to ensure that T::Item satisfies certain bounds within the enum, you should handle that in functions or methods that work with the enum."
          T: Copy+PartialEq<String>+std::fmt::Debug,
        //String: PartialEq<T>,   // Bound on `String`, using the type parameter
        i32: Default,           // Allowed, but not useful
    {
        Tee { f: i32 },
        Red(T,G), Green(G,i32), Blue(i64,i128,),
        Magenta,
        Foo { field1: i32 },
        Cons(i32, Box<Color2<T,G>>),
        Nil,
        Cons2{ f:i32, g:Box<Color2<T,G>> },
    }
}

fn process_color<T, G>(color: Color2<T, G>)
where
    T: MyTrait + Copy+PartialEq<String>+std::fmt::Debug,
    T::Item: Copy, // Bound on associated type
    G: Clone+std::fmt::Debug,
{
    match color {
        Color2::Tee { f } => println!("Tee: {}", f),
        Color2::Red(t, g) => println!("Red: {:?}, {:?}", t, g),
        Color2::Green(g, i) => println!("Green: {:?}, {}", g, i),
        Color2::Blue(a, b) => println!("Blue: {}, {}", a, b),
        Color2::Magenta => println!("Magenta"),
        Color2::Foo { field1 } => println!("Foo: {}", field1),
        Color2::Cons(i, box_color) => {
            println!("Cons: {}", i);
            process_color(*box_color); // Recursive call with bound
        }
        Color2::Nil => println!("Nil"),
        Color2::Cons2 { f, g } => {
            println!("Cons2: {}", f);
            process_color(*g); // Recursive call with bound
        }
    }
}

//enum_str! {
custom_derive! {
    #[allow(dead_code)]
    pub enum Foo where { //empty where is supported in normal rust too!
        Foo2,
        r#New,
        _Moo,
        r#_Moo2,
        東京,
        r#東京2,
    }
}
//enum_str! {
//    pub enum YourEmpty,// {} //XXX: not supported on purpose!
//}

//enum_str! {
custom_derive! {
    #[allow(dead_code)]
    pub enum Ahh<'a, T>
        where
            T: Iterator,            // Could use A<T: Iterator> instead
    T::Item: Copy,          // Bound on an associated type
            String: PartialEq<T>,   // Bound on `String`, using the type parameter
            i32: Default,           // Allowed, but not useful
            i64:,
            'a:,
            'a: 'a+,
            'a: 'a+'a,
            {
                f(T),
                g(&'a T),
            }

}




fn main() {
    println!("{}", Dagashi::FueRamune.as_str());
}
