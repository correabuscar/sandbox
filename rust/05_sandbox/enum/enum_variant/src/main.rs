//  I just want to see the type, as string (the goal is to know what type an enum var is).

// How do I do this, without using the Debug/Display trait (because toString will show a message instead, on the enum I want to apply it to)
// and without passing the enum type to the macro.
// Doesn't have to be via macros.

// XXX: So apparently can't be done except that it can show you the exact type but not the
// variant(unless it derives Debug, but if you manually derived Debug then the exact variant name
// is probably lost because eg. you decided to show a message instead like pkg_config::Error enum
// does)

#![allow(dead_code)]
#![feature(type_name_of_val)]

use std::any;

macro_rules! print_enum_variant {
    ( $e:expr) => {
        println!("Enum variant: {}::{:?}", any::type_name_of_val(&$e), $e);
    };
}

// Define an enum
#[derive(Debug)]
enum MyEnum {
    Variant1,
    Variant2,
    // Add more variants...
}

#[derive(Debug)]
enum AnotherEnum {
    Item1,
    Item2,
    // Add more variants...
}

fn main() {
    let e1 = MyEnum::Variant2;
    // Use the macro to print the current variant of the enum
    print_enum_variant!( e1);

    let e2 = AnotherEnum::Item1;
    // Use the macro to print the current variant of the enum
    print_enum_variant!( e2);
}

