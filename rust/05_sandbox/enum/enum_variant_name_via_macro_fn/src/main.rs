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
/// matches any variants that are: unit, tuple or struct like,
/// and also variants with associated value/constant like Ok=200 but in this latter case, but only when mixed with other variant types, you'd have to add eg. #[repr(u8)] //else, error[E0732]: `#[repr(inttype)]` must be specified
/// empty enums are not supported, like pub enum Foo{}, on purpose!
/// see: https://doc.rust-lang.org/reference/items/enumerations.html
#[macro_export]
macro_rules! enum_str {
    (
        //matches attributes like #[allow(dead_code)], if any!
        //or the more likely-used one: #[repr(u8)]
        $( #[ $enum_outer_attr:meta ] )* // this is enum's outer attribute
                                         // meta: an Attr, the contents of an attribute
        //matches 'pub enum Something<T,G,F>' but also just 'enum Something'
        $visibility:vis
        //TODO: get genericparams right/complete!
        //Enumeration : `enum` IDENTIFIER  GenericParams? WhereClause? `{` EnumItems? `}`
        enum $name:ident
        //https://doc.rust-lang.org/reference/items/generics.html
        // GenericParams : `<` `>` | `<` (GenericParam `,`)* GenericParam `,`? `>`
        $(<
            // GenericParam : OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )
            $(
                // OuterAttribute : `#` `[` Attr `]`
                $( #[ $enum_generics_outer_attr:meta ] )*
                //it's supposed to be one of the following, but can't tell it that thru macro_rules matching,
                //so it can be all 3 at once or none at all
                //TODO: go on... from here:
                //LifetimeParam : LIFETIME_OR_LABEL ( `:` LifetimeBounds )?
                $(
                )?
                //TypeParam : IDENTIFIER( `:` TypeParamBounds? )? ( `=` Type )?
                $(
                $enum_generic:ident
                )?
                //ConstParam: `const` IDENTIFIER `:` Type ( `=` Block | IDENTIFIER | `-`?LITERAL )?
                $(
                )?
            ),*
            >)?
         // Added support for a where clause
        //FIXME: make this match(verb) each possible thing in the where, individually, due to: local ambiguity when calling macro `enum_str`: multiple parsing options: built-in NTs tt ('where_clause') or 1 other option.
        $(where
            //yes 'where' with nothing following it is allowed in rust, oddly enough.
            //WhereClause: where ( WhereClauseItem , )* WhereClauseItem ?
            $(
                // WhereClauseItem : LifetimeWhereClauseItem  | TypeBoundWhereClauseItem
                //$( $where_clause:tt )+
                $(
                    // LifetimeWhereClauseItem : Lifetime `:` LifetimeBounds
                    $lifetime:lifetime :
                    $lifetime_bounds:tt
                )?
                $(
                    // TypeBoundWhereClauseItem :     ForLifetimes? Type `:` TypeParamBounds?
                    $(
                        // ForLifetimes : `for` GenericParams
                        for
                        //TODO: get genericparams right!
                        <$($generic_params:ident),*>
                    )?
                    $where_type:ty
                )?
            ),*
        )?
        { //enum's opening brace
        $(
            //matches VariantName, VariantName(), VariantName(i32), VariantName(i32,i128,)
            //but also weirds like: VariantName(,)
            //also matches VariantName {}, VariantName { f:i32, }, VariantName { f:i32, g: u8, },
            //but also weirds like: StructVariantOops1 {,}
            $variant:ident
            $( (
                    $($tfield:ty),*
                    $(,)?
            ) )?
            $( {
                $($sfield:ident: $stype:ty),*
                    $(,)?
            } )?
            $( = $assoc_value:expr
            )?
        ),*
        $(,)?
        } //enum's closing brace
    ) => {
        $(#[$enum_outer_attr])*
            $visibility enum $name $(<$($gen),*>)?
            $(where
                $(
                    $lifetime:$lifetime_bounds
                )?
                $(
                    $(
                        for <$generic_params>
                    )?
                )?
                $where_type
            )?
            {
                $(
                    $variant
                      $( ( $($tfield),* ) )?
                      $( { $($sfield: $stype),* } )?
                      $( = $assoc_value )?
                ),*
            }//enum

        //$crate::replace_with2!(
        impl $(<$($gen),*>)? $name $(<$($gen),*>)? {
            pub const fn variant_name_as_str(&self) -> &str {
                match self {
                    $(
                        Self::$variant
                        //below, .. is the Rest pattern https://doc.rust-lang.org/reference/patterns.html#rest-patterns
                        //this handles both unit and tuple  enum variants:
                        //"Each repetition in the transcriber must contain at least one metavariable to decide how many times to expand it. " src: https://doc.rust-lang.org/reference/macros-by-example.html#repetitions
                        //this is why we must use $tfield below to know whether to even create the whole line
                        //and then know to place () if it's empty, or (..) if it has any $tfield-s
                        $( ( $crate::place_first_arg_ignore_rest_if_any!(.., $($tfield),* ) ) )?
                        //below, _ is the Inferred type, https://doc.rust-lang.org/reference/types/inferred.html
                        //so it's not the Wildcard pattern https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern
                        //this handles only the struct enum variants:
                        $( { $($sfield: _),* } )?
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
        //| $variant)
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
    #[warn(dead_code)]
    pub enum MyError {
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
}

enum_str! {
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

enum_str! {
    #[allow(dead_code)]
    #[repr(u8)] //error[E0732]: `#[repr(inttype)]` must be specified
    enum Color {
        Red, Green, Blue,
        StructVariant0 {},
        StructVariantOops1 {,},
        StructVariant1 {
            field1: i32,
        },
        StructVariant2 {
            field1: i32,
            field2: i32,
        },
        TupleVariant(i32),
        Ooops(,),//FIXME: shouldn't match but hey, the macro itself is a hack!
        //Oops2(){},//it errors but it's not clear why it does, unless u know it's bad syntax.
        UnitWithConstant = 200, //XXX: due to mixed with other variant types, it requires #[repr(u8)] //else: error[E0732]: `#[repr(inttype)]` must be specified
    }
}

//enum_str! {
    pub enum MyEnum<'a, 'b> where 'a: 'b {
        Variant1(&'a str),
        Variant2(&'b str),
    }
//}

enum_str! {
    pub enum Color2<T,G>
    where T:Copy, G: Clone,
        T::Item: Copy,          // Bound on an associated type
        String: PartialEq<T>,   // Bound on `String`, using the type parameter
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

enum_str! {
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

fn main() {
    //let empty=YourEmpty;//XXX: can't instantiate "In Rust, an empty enum, such as YourEmpty in your example (pub enum YourEmpty {}), can be defined, but it cannot be instantiated."

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
