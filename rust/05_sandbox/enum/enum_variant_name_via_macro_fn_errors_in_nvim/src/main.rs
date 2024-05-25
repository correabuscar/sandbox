#![allow(dead_code)]

macro_rules! replace_with_2_dots {
    ( $($input:tt)* ) => {
        ..
    };
}

macro_rules! enum_str {
    (
    $(
        $variant:ident
            ( $($tfield:ty),* )
    ),* ) => {
        enum Foo {
            $(
                $variant ( $($tfield),* ),
            )*
        }

        impl Foo {
            fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        //Self::$variant (..) // WORKS
                        Self::$variant ( replace_with_2_dots!( $($tfield),* ) )
                          => stringify!($variant),

                    )*
                }
            }
        }
    };
}

enum_str! {
    //TupleVariant1(i32)//WORKS
    //TupleVariant2()//DOESN'T: E0023: this pattern has 1 field, but the corresponding tuple struct has 0 fields
    TupleVariant3(i8,u8,i128)//DOESN'T: E0023: this pattern has 1 field, but the corresponding tuple struct has 3 fields
}

//XXX src/main.rs|36-40 col 1 error| E0023: this pattern has 1 field, but the corresponding tuple struct has 3 fields
//run with: $ rust-analyzer diagnostics ./
//see: https://github.com/rust-lang/rust/issues/125464

fn main() {
}
