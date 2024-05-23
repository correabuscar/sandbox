#[macro_export]
macro_rules! replace_with_2_dots {
    ($($input:tt)*) => {
        ..
    };
}

#[macro_export]
macro_rules! enum_str {
    (
    $( $variant:ident
       $( ( $($tfield:ty),* $(,)? ) )?
       $( { $($sfield:ident: $stype:ty),* $(,)? } )?
    ),* $(,)?
    ) => {

    pub enum Color {
            $(
                $variant $( ($($tfield),*) )?
                         $( { $($sfield: $stype),* })?
            ),*
        }

    impl Color {
        fn variant_name_as_str(&self) -> &str {
            match self {
                $(
                    Self::$variant $( ( $crate::replace_with_2_dots!( $($tfield),* ) ) )? $( { $($sfield: _),* } )? => stringify!($variant),
                    //$(Self::$variant_struct { .. })? => stringify!($variant),
                )*
            }
        }
    }
    };
}

enum Foo {
    TupleVarianttttt(), //this works!
}

enum_str! {

    Tee { f: i32 },
    Red, Green, Blue,
    StructVariant2 {},
    StructVariant1 {
        field1: i32,
    },
    TupleVariant(i32,),
    //TupleVariant2(),
    //TupleVariant3(i8,u8,i128,),
}

fn main() {
    let c=Color::Tee { f: 10 };
    println!("Hello, world! {}",c.variant_name_as_str());
}
