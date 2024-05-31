//#![feature(decl_macro)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

macro_rules! foo {
    (
        //macro can take no args
        $(
            enum $foo:ident
            //macro can take just the ident, so the following are optional:
            $(
                // EBNF(?) grammar:
                // GenericParams : `<` `>` | `<` (GenericParam `,`)* GenericParam `,`? `>`
                <
                //macro can take empty generics <>
                //$(


                    // XXX: not happy about this GenericParam dual duplication that's needed to avoid lone comma!
                    // GenericParam : OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )

                    // OuterAttribute : `#` `[` Attr `]`
                    $( #[ $enum_generics_outer_attr_1of2:meta ] )*

                    // XXX: we DON'T ignore "| TypeParam | ConstParam )" in this try(but we wouldn't be able to ensure to pick only 1 of the 3 because 0,2 and 3 of the 3 will also match)!
                    // So we assume LifetimeParam appears mandatorily below:
                    $(
                    // LifetimeParam : LIFETIME_OR_LABEL ( `:` LifetimeBounds )?

                    // LIFETIME_OR_LABEL here and not LIFETIME_TOKEN(which is what 'lifetime' does)
                    // LIFETIME_OR_LABEL : `'` NON_KEYWORD_IDENTIFIER (not immediately followed by `'`)
                    // ^ can't do that because lone ' won't be accepted!, so XXX: 'lifetime' will do here because anything else will compile error at transcribe site.
                    $lif_1of2:lifetime

                    // ( `:` LifetimeBounds )?
                    $(
                        :
                        // perfectFIXME: how to match this properly:
                        // LifetimeBounds : ( Lifetime `+` )* Lifetime?
                        // so LifetimeBounds is itself implicitly optional: can be none,one, or if more separated by +
                        $(
                            $lifebound_1of2:lifetime
                            $(+ $morelifebounds_1of2:lifetime)*
                        )?
                    )?

                    $(
                        ,
                        // GenericParam : OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )

                        // OuterAttribute : `#` `[` Attr `]`
                        $( #[ $enum_generics_outer_attr_2of2:meta ] )*

                        // XXX: we ignore "| TypeParam | ConstParam )" in this try(but we wouldn't be able to ensure to pick only 1 of the 3 because 0,2 and 3 of the 3 will also match)!
                        // So we assume LifetimeParam appears mandatorily below:

                        // LifetimeParam : LIFETIME_OR_LABEL ( `:` LifetimeBounds )?

                        // LIFETIME_OR_LABEL here and not LIFETIME_TOKEN(which is what 'lifetime' does)
                        // LIFETIME_OR_LABEL : `'` NON_KEYWORD_IDENTIFIER (not immediately followed by `'`)
                        // ^ can't do that because lone ' won't be accepted!, so XXX: 'lifetime' will do here because anything else will compile error at transcribe site.
                        $lif_2of2:lifetime

                        // ( `:` LifetimeBounds )?
                        $(
                            :
                            // perfectFIXME: how to match this properly:
                            // LifetimeBounds : ( Lifetime `+` )* Lifetime?
                            // so LifetimeBounds is itself implicitly optional: can be none,one, or if more separated by +
                            $(
                                $lifebound_2of2:lifetime
                                $(+ $morelifebounds_2of2:lifetime)*
                            )?
                        )?
                     )*

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
                                    // TraitBound : `?`? ForLifetimes? TypePath | `(` `?`? ForLifetimes? TypePath `)`
                                    $(?)?
                                    // ForLifetimes?
                                    //TODO: ^
                                    $(
                                        // ForLifetimes : `for` GenericParams
                                        for
                                        //TODO: get genericparams right!
                                        // FIXME: well, we're inside genericparams and we're supposed to parse another one, whoopsies! so recursion but how exactly?!
                                        < /* TODO */ >
                                    )?
                                    // TypePath
                                    $enum_generics_lifetime_param_type_param_bound_traitbound_typepath:path
                                    //TODO: `(` `?`? ForLifetimes? TypePath `)`
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

                     //optionally can end with one comma but only if there was an ident already!
                     $(,)?
                     )? //LifetimeParam
                //)? // we use $()? on each of the 3(the workaround), so we can't have this outter one!
                >
            )?
                        {
                           $( $rest:tt )*
                        }
        )?
    ) => {
        $(
            foo! {
            @enum_name $foo //,
            ( // parens required else multiple parsing options tt ...
                @generics
                $(
                    <
                    $(
                        $(#[$enum_generics_outer_attr_1of2])*
                        $lif_1of2
                        $(
                            :
                            $(
                                $lifebound_1of2
                                $(+ $morelifebounds_1of2)*
                            )?
                        )?
                        $(
                            ,
                            $(#[$enum_generics_outer_attr_2of2])*
                            $lif_2of2
                            $(
                                :
                                $(
                                    $lifebound_2of2
                                    $(+ $morelifebounds_2of2)*
                                )?
                            )?
                        )*
                    )?
                    >
                )?
            ) // generics
            @rest
                    {
                       $( $rest )*
                    }
            } //call to macro foo!()
        )?

    };
    (
        //this is some attempt to do it like nerditation did here: https://users.rust-lang.org/t/enum-variant-name-as-str-without-debug-display-proc-macro-or-heap-allocation/111876/5?u=correabuscar
        //but to be fair I don't understand that example, so I did it with only the part that I think I understood!
        //but the goal here is to use only one arg for the generics even tho 2 are used due to lone-comma-matching avoidal.
        @enum_name $name:ident
       (@generics $($generics:tt)*)
        @rest {$($rest:tt)*}) => {
        enum $name
            $($generics)*
            {
                $($rest)*
            }
    }

}

foo!();
foo!(enum Foo_1{});
foo!(enum Foo0<>{});
//foo!(enum Foo0ops<,>{});//doneFIXME: shouldn't match this lone comma
foo!(enum Foo1<'a>{
_foo(&'a str),
});
foo!(enum Foo1ok<'a,>{
_foo(&'a str),
});
foo!(enum Foo2<'a:>{
_foo(&'a str),
});

enum Foo2oops_raw<'a:,>{//XXX: yeah it should match hmm
    _foo(&'a str),
}

foo!(enum Foo2oops<'a:,>{//actuallyitshouldFIXME: shouldn't match this lone comma
_foo(&'a str),
});

foo!(enum Foo3<'a: 'b,'b:>{
_foo(&'a str),
_foo2(&'b str),
});
foo!(enum Foo4<'a: 'b + 'c,'b:, 'c:>{
_foo(&'a str),
_foo2(&'b str, &'c str),
});

fn main() {}

