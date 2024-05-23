macro_rules! replace_with_2_dots {
    ($($input:tt)*) => {
        ..
    };
}

macro_rules! enum_str {
    ($(#[$attr:meta])* $vis:vis enum $name:ident $(<$($gen:ident),*>)?,
    $(
        $variant:ident
            $( ( $($tfield:ty),* $(,)? ) )?
            $( { $($sfield:ident: $stype:ty),* $(,)? } )?
    ),* $(,)?
    ) => {
        $(#[$attr])*
        $vis enum $name $(<$($gen),*>)? {
            $(
                $variant $( ( $($tfield),* ) )?
                         $( { $($sfield: $stype),* } )?,
            )*
        }

        #[allow(dead_code)]
        impl $(<$($gen),*>)? $name $(<$($gen),*>)? {
            fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        Self::$variant $( ( replace_with_2_dots!( $($tfield),* ) ) )? $( { $($sfield: _),* } )? => stringify!($variant),

                    )*
                }
            }
        }
    };
}
//enum_str! {
//    pub enum Color,
//    Red, Green, Blue,
//    StructVariant1 { field1: i32 },
//    TupleVariant(i32),
//}

//XXX: src/main.rs|47-59 col 1 error| E0023: this pattern has 1 field, but the corresponding tuple struct has 0 fields



// XXX: this error happens only in neovim
// src/main.rs|42-49 col 1 error| E0023: this pattern has 1 field, but the corresponding tuple struct has 2 fields
enum_str! {
    pub enum Color2, //<T, G>,
//    Tee { f: i32 },
//    Red(T, G), Green(G, i32), Blue(i64, i128),
    //Roo(T,G),
    //TupleVariant1(i32),//WORKS
    TupleVariant2(),//DOESN'T
    //TupleVariant3(i8,u8,i128,),//DOESN'T
//    Magenta,
//    Foo { field1: i32 },
}

fn main() {
//    let c = Color::Blue;
//    assert_eq!(c.variant_name_as_str(), "Blue");

//    let c2 = Color2::<i128, &str>::Green("text", 2);
//    assert_eq!(c2.variant_name_as_str(), "Green");

//    let c3 = Color2::<i32,i32>::Magenta;
//    assert_eq!(c3.variant_name_as_str(), "Magenta");

//    let c4 = Color2::<u8,u8>::Foo { field1: 42 };
//    assert_eq!(c4.variant_name_as_str(), "Foo");

 //   let c5 = Color::StructVariant1 { field1: 10 };
 //   assert_eq!(c5.variant_name_as_str(), "StructVariant1");

 //   let c6 = Color::TupleVariant(10);
 //   assert_eq!(c6.variant_name_as_str(), "TupleVariant");
}
