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
        //or doc comments like /// text, which are #[doc = " text"]
        $( #[ $enum_outer_attr:meta ] )* // this is enum's outer attribute
                                         // meta: an Attr, the contents of an attribute

        //matches 'pub enum Something<T,G,F>' but also just 'enum Something'
        $visibility:vis
        // vis: a possibly empty Visibility qualifier
        // aka it can be missing, or has an implied $()? wrapper!

        //Enumeration : `enum` IDENTIFIER  GenericParams? WhereClause? `{` EnumItems? `}`
        enum $name:ident
        //TODO: get genericparams right/complete!
        //https://doc.rust-lang.org/reference/items/generics.html
        // GenericParams : `<` `>` | `<` (GenericParam `,`)* GenericParam `,`? `>`
        $(<
            // XXX: if you change anything in the below GenericParam code block, remember to also change it in the one after that deals with more GenericParam prefixed by comma (which is needed so lone comma won't be matched), each metavar is named like 1of2 or 2of2 to know that's used in 2 blocks and thus u'll know if changes you've done adjacently need to be done in another/the_other block too.
            // GenericParam : OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )
            //$( //see below, tl;dr: same effect even if this isn't here.
                // OuterAttribute : `#` `[` Attr `]`
                $( #[ $enum_generics_outer_attr_1of2:meta ] )*
                //it's supposed to be one of the following, but can't tell it that thru macro_rules matching,
                //so it can be all 3 at once or none at all; I guess we let the compiler warn us about this at expanded location!

                //so what we do next is optionalize each of ( LifetimeParam | TypeParam | ConstParam )

                //LifetimeParam : LIFETIME_OR_LABEL ( `:` LifetimeBounds )?
                $(
                    // LIFETIME_OR_LABEL here and not LIFETIME_TOKEN(which is what 'lifetime' does)
                    $enum_generics_lifetime_1of2:lifetime //so not this! but a lone ' errors, so this!
//                    //LIFETIME_OR_LABEL : `'` NON_KEYWORD_IDENTIFIER (not immediately followed by `'`)
//                    ' //XXX: errors on this, so we've to use 'lifetime' then! and expect compile errors at expansion site to catch the relevant cases!
//                    //NON_KEYWORD_IDENTIFIER : IDENTIFIER_OR_KEYWORD Except a strict or reserved keyword
//                    // https://doc.rust-lang.org/reference/keywords.html#strict-keywords
//                    // https://doc.rust-lang.org/reference/keywords.html#reserved-keywords
//                    // ok wel do any ident then  'cuz how can we except those! ^
//                    $enum_generics_lifetime_id:ident
                    // ( `:` LifetimeBounds )?
                    $(
                        : // lone ":" is valid!
                        // LifetimeBounds : ( Lifetime `+` )* Lifetime?
                        // so LifetimeBounds is itself optional, basically: can be none,one, or if more separated by +
//                        $(
//                            $enum_generics_lifetime_bounds:lifetime
//                            //+
//                        )+*  //diditdifferentlyTODO: crap!//XXX: so this is $()+ and literal *, not $()+* aka 0 or more of + separated elements that don't end with + !! "$ ( ... ) sep rep" "sep is an optional separator token. It may not be a delimiter or one of the repetition operators. Common examples are , and ;." src: https://veykril.github.io/tlborm/decl-macros/macros-methodical.html#repetitions
                        $(
                            $enum_generics_lifetime_bounds_1of2:lifetime
                            $(+ $enum_generics_lifetime_bounds_more_1of2:lifetime)*
                        )?
                    )?


                 )? //the LifetimeParam
                // TypeParam : IDENTIFIER( `:` TypeParamBounds? )? ( `=` Type )?
                $(
                    $enum_generic_typeparam_ident_1of2:ident
                    //TODO: go on
                    // ( `:` TypeParamBounds? )?
                    $(
                        :
                        // TypeParamBounds?
                        //$(
                            // TypeParamBounds : TypeParamBound ( `+` TypeParamBound )* `+`?
                            // TypeParamBound : Lifetime | TraitBound
                            $(
                                // Lifetime
                                $enum_generics_lifetime_param_type_param_bound_lifetime_1of2:lifetime
                            )?
                            // TraitBound
                            $(
                                // TraitBound : `?`? ForLifetimes? TypePath | `( ?`? ForLifetimes? TypePath `)`
                                $(?)?
                                // ForLifetimes?
                                //TODO: ^
                                // TypePath
                                $enum_generics_lifetime_param_type_param_bound_traitbound_typepath:path
                            )?
                            // ( `+` TypeParamBound )*
                            $(
                                //FIXME: good one roost: `$enum_generics_lifetime_param_type_param_bound_traitbound_typepath:path` may be followed by `+`, which is not allowed for `path` fragments: not allowed after `path` fragments
                                +
                                // TypeParamBound : Lifetime | TraitBound
                            )*
                            // `+`?
                            $(+)?
                        //)?
                    )?
                    // ( `=` Type )?
                    $(
                        =
                        $enum_generic_typeparam_ident_type_1of2:ty
                    )?
                )?
                // ConstParam: `const` IDENTIFIER `:` Type ( `=` Block | IDENTIFIER | `-`?LITERAL )?
                $(
                    //FIXME:good one roost: `$enum_generic_typeparam_ident_type_1of2:ty` may be followed by `const`, which is not allowed for `ty` fragments: not allowed after `ty` fragments
                    const $enum_generics_constparam_ident_1of2:ident : $enum_generics_constparam_type_1of2:ty
                    $(
                        =
                        //TODO: rest
                    )?
                )?

                //TODO: more than the one(above) generics? go on then, dupe the above :/
                $(
                    ,
                    //XXX: duplicates the above! with _2of2 prefixes for metavars!
                    //TODO: here
                )*
                //optionally can end with one comma but only if there was an ident already!(already done, this is why there's 2 dup blocks for GenericParam !)
                $(,)?
            //)? //XXX:can have 0 or more generics, but due to everything inside this being optional, can't use this due to 'repetition matches empty token tree', but since everything inside is optional, it has the same effect as if this $()? was in use here!
            // `,`?
            //$(,)? //doneFIXME: lone comma possibly with this, but shouldn't be!
            >)?
         // Added support for a where clause
        //FIXME: make this match(verb) each possible thing in the where, individually, due to: local ambiguity when calling macro `enum_str`: multiple parsing options: built-in NTs tt ('where_clause') or 1 other option.
        $( where
            //yes 'where' with nothing following it is allowed in rust, oddly enough.
            //WhereClause: `where` ( WhereClauseItem `,` )* WhereClauseItem ?
            $(
                // WhereClauseItem : LifetimeWhereClauseItem  | TypeBoundWhereClauseItem
                //$( $where_clause:tt )+
                $(
                    // LifetimeWhereClauseItem : Lifetime `:` LifetimeBounds
                    $lifetime:lifetime
                    : //yeah this isn't optional here, but it is optional above, in the generics
                    // LifetimeBounds : ( Lifetime `+` )* Lifetime?
                    // so LifetimeBounds is itself optional, basically.
                    $(
                        $lifetime_bounds:lifetime
                        //+
                    )+*  //TODO: so is this $()+ and literal *, or $()+* aka 0 or more of + separated elements that don't end with + ?!
                )?
                $(
                    // TypeBoundWhereClauseItem :     ForLifetimes? Type `:` TypeParamBounds?
                    $(
                        // ForLifetimes : `for` GenericParams
                        for
                        //TODO: get genericparams right!
                        <$($generic_params:ident),*>
                    )?
                    // Type `:`
                    $where_type:ty
                    :
                    // TypeParamBounds?
                    $(
                        // TypeParamBounds : TypeParamBound ( `+` TypeParamBound )* `+`?
                        $(
                            // TypeParamBound : Lifetime | TraitBound
                            $(
                                // Lifetime
                                $where_type_param_bound_lifetime:lifetime
                            )?
                            // TraitBound
                            $(
                                // TraitBound : `?`? ForLifetimes? TypePath | `( ?`? ForLifetimes? TypePath `)`
                                $(?)?
                                // ForLifetimes?
                                //TODO: ^
                                // TypePath
                                $where_type_param_bound_traitbound_typepath:path
                            )?
                        )++
                        $(+)?
                    )?
                )?
            ),*
            //can end with optional comma, but this makes `where ,` be valid(even tho it isn't) because ofc. FIXME: if u can.
            $(,)?
        )?
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
                        //FIXME: maybe do it like their grammar? then won't match just the lone comma, and can use outer $()? block then! (their grammar refers to this line: TupleFields : TupleField (`,` TupleField)* `,`? )
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
        $(,)?
        } //enum's closing brace
    ) => {
//        $(#[$enum_outer_attr])*
//            $visibility enum $name
//            //generics TODO:
////            $(
////                <$($gen),*>
////            )?
//            $(where
//                $(
//                    $lifetime:$lifetime_bounds
//                )?
//                $(
//                    $(
//                        for <$generic_params>
//                    )?
//                )?
//                $where_type
//            )?
//            {
//                $(
//                    $( #[ $enumitem_outer_attr ] )*
//                    $variant
//                      $( ( $($enumitem_tuple_field),* ) )?
//                      $( { $($enumitem_struct_field_ident: $enumitem_struct_field_type),* } )?
//                      $( = $enumitem_discriminant )?
//                ),*
//            }//enum
//
//        //$crate::replace_with2!(
//        impl
//            //generics TODO:
//            //$(<$($gen),*>)? 
//            $name
//                //generics again, TODO:
//                //$(<$($gen),*>)? 
//                {
//            pub const fn variant_name_as_str(&self) -> &str {
//                match self {
//                    $(
//                        Self::$variant
//                        //below, .. is the Rest pattern https://doc.rust-lang.org/reference/patterns.html#rest-patterns
//                        //this handles both unit and tuple  enum variants:
//                        //"Each repetition in the transcriber must contain at least one metavariable to decide how many times to expand it. " src: https://doc.rust-lang.org/reference/macros-by-example.html#repetitions
//                        //this is why we must use $enumitem_tuple_field below to know whether to even create the whole line
//                        //and then know to place () if it's empty, or (..) if it has any $enumitem_tuple_field-s
//                        //TODO: can maybe use here(below) this instead: ${ignore(enumitem_tuple_field)} see: https://veykril.github.io/tlborm/decl-macros/minutiae/metavar-expr.html#ignoreident  which doesn't compile hmm! missing $ sign on inner metavar so: ${ignore($enumitem_tuple_field)} should work!
//                        $( ( $crate::place_first_arg_ignore_rest_if_any!(.., $($enumitem_tuple_field),* ) ) )?
//                        //below, _ is the Inferred type, https://doc.rust-lang.org/reference/types/inferred.html
//                        //so it's not the Wildcard pattern https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern
//                        //this handles only the struct enum variants:
//                        $( { $($enumitem_struct_field_ident: _),* } )?
//                        => stringify!($variant),
//                    )*
////                        #[allow(unreachable_patterns)]
////                        _ => {
////                            //that {{}} is actually expanded to: {} aka escaped, and this panic!() works in 'const fn' as opposed to unreachable!()
////                            panic!("Unreachable! This was only needed in case of empty enum like: enum Foo {{}}, because we can't conditionally not include the whole impl based on $variant due to macro saying it's already repeating at this depth");
////                        }
//                }//match
//            }//fn
//        }//impl
//        //| $variant)
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
        }=100,//yes this should work
        TupleVariant(i32)=101,//so this
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

enum_str! {
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
    //let empty=YourEmpty;//XXX: can't instantiate "In Rust, an empty enum, such as YourEmpty in your example (pub enum YourEmpty {}), can be defined, but it cannot be instantiated."

//    let error1 = MyError::AlreadyBorrowedOrRecursingError {
//        source: BorrowMutError,
//        location_of_instantiation: LocationInSource,
//        custom_message: NoAllocFixedLenMessageOfPreallocatedSize,
//    };
//    assert_eq!(
//        error1.variant_name_as_str(),
//        "AlreadyBorrowedOrRecursingError"
//    );
//
//    let error2 = MyError::TimeoutError {
//        location_of_instantiation: LocationInSource,
//        duration: Duration,
//        tid: 12345,
//        custom_message: NoAllocFixedLenMessageOfPreallocatedSize,
//    };
//    assert_eq!(error2.variant_name_as_str(), "TimeoutError");
//    println!("{:?}", error1);
//    let c=Color::Blue;
//    assert_eq!(c.variant_name_as_str(),"Blue");
//    let c2=Color2::<i128,&str>::Green("text",2);
//    assert_eq!(c2.variant_name_as_str(),"Green");
}
