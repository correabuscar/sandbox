//#![feature(decl_macro)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

//macro_rules! foo {
//    (
//        //macro can take no args
//        $(
//            enum $foo:ident
//            //macro can take just the ident
//            $(
//                <
//                $(
//                    $generics:tt
//                )?
//                >
//            )?
//        )?
//    ) => {
//        $(
//            enum $foo $(
//                $(
//                    $generics
//                )?
//            )? { }
//        )?
//    };
//}

macro_rules! foo {
    (
        //macro can take no args
        $(
            enum $foo:ident
            //macro can take just the ident
            $(
                // EBNF(?) grammar:
                // GenericParams : `<` `>` | `<` (GenericParam `,`)* GenericParam `,`? `>`
                <
                //macro can take empty generics <>
                $(
                    // GenericParam : OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )

                    // OuterAttribute : `#` `[` Attr `]`
                    $( #[ $enum_generics_outer_attr:meta ] )*

                    // XXX: we ignore "| TypeParam | ConstParam )" in this try(but we wouldn't be able to ensure to pick only 1 of the 3 because 0,2 and 3 of the 3 will also match)!
                    // So we assume LifetimeParam appears mandatorily below:

                    // LifetimeParam : LIFETIME_OR_LABEL ( `:` LifetimeBounds )?

                    // LIFETIME_OR_LABEL here and not LIFETIME_TOKEN(which is what 'lifetime' does)
                    // LIFETIME_OR_LABEL : `'` NON_KEYWORD_IDENTIFIER (not immediately followed by `'`)
                    // ^ can't do that because lone ' won't be accepted!, so XXX: 'lifetime' will do here because anything else will compile error at transcribe site.
                    $lif:lifetime

                    // ( `:` LifetimeBounds )?
                    $(
                        :
                        // FIXME: how to match this properly:
                        // LifetimeBounds : ( Lifetime `+` )* Lifetime?
                        // so LifetimeBounds is itself implicitly optional: can be none,one, or if more separated by +
                        $(
                            $lifebound:lifetime
                            +
                        )*
                    )?
                ),*

                // `,`? //FIXME: too bad it matches only the lone comma, as well
                $(,)?
                >
            )?
                        {
                           $( $rest:tt )*
                        }
        )?
    ) => {
        $(
            enum $foo
            $(
                <
                $(
                    $(#[$enum_generics_outer_attr])*
                    $lif
                    $(
                        :
                        $(
                            $lifebound
                            +
                        )*
                    )?
                ),*
                >
            )?
                    {
                        $( $rest )*
                    }
        )?

    }
}

foo!();
foo!(enum Foo_1{});
foo!(enum Foo0<>{});
foo!(enum Foo0ops<,>{});//FIXME: shouldn't match this!
foo!(enum Foo1<'a>{
_foo(&'a str),
});
foo!(enum Foo1ok<'a,>{
_foo(&'a str),
});
foo!(enum Foo2<'a:>{
_foo(&'a str),
});
foo!(enum Foo2oops<'a:,>{//FIXME: shouldn't match this!
_foo(&'a str),
});
foo!(enum Foo3<'a: 'b+,'b:>{//FIXME: shouldn't require the + ending!
_foo(&'a str),
_foo2(&'b str),
});
foo!(enum Foo4<'a: 'b + 'c+,'b:, 'c:>{//FIXME: shouldn't require the + ending!
_foo(&'a str),
_foo2(&'b str, &'c str),
});

fn main() {}

