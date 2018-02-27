#![feature(plugin)]
#![plugin(clippy)]

#![deny(clippy, clippy_pedantic, clippy_restrictions, clippy_internal, deref_addrof, needless_borrow)]
#![allow(missing_docs_in_private_items)]

#![feature(stmt_expr_attributes)]

// https://github.com/rust-lang/book/blob/master/first-edition/src/lifetimes.md#thinking-in-scopes
struct Foo<'a> {
    x: &'a i32,
}


fn main() {
    let mut x: &i32;                    // -+ `x` comes into scope.
//    let z;
                              //  |
    {                         //  |
        let y = &5;           // ---+ `y` comes into scope.
        let f = Foo { x: y }; // ---+ `f` comes into scope.
        #[allow(unused_assignments)]
        x = &f.x;             //  | | This causes an error. NOT ANYMORE, https://github.com/rust-lang/book/issues/455#issuecomment-368752929
//        z = &f.x;//error[E0597]: `f.x` does not live long enough
//^ what type is 'z' ?!
        //let _: () = z; //shows type of z, &&i32 (yes, two &) Thanks to durka42 (on irc)
//        #[allow(print_stdout)] {
//            println!("{}", x);
//        }
        x = f.x; //so x=&f.x is the same as x=f.x, great!
    }                         // ---+ `f` and y go out of scope.
                              //  |
    #[allow(print_stdout)] { //whaaa? needs block! prolly because macro?
        //println!("{} {}", x, z);        //  |
        println!("{}", x);        //  |
    }
//    #[allow(print_stdout)]
//    println!("{}", x); //why this fails w/o {} block?
}
