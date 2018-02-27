// https://github.com/rust-lang/book/blob/master/first-edition/src/lifetimes.md#thinking-in-scopes
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let x: &i32;                    // -+ `x` comes into scope.
    let z;
                              //  |
    {                         //  |
        let y = &5;           // ---+ `y` comes into scope.
        let f = Foo { x: y }; // ---+ `f` comes into scope.
        x = &f.x;             //  | | This causes an error. NOT ANYMORE
        z = &f.x;//error[E0597]: `f.x` does not live long enough
//^ what type is 'z' ?!
    }                         // ---+ `f` and y go out of scope.
                              //  |
    println!("{} {}", x, z);        //  |
}
