//initial code from: file:///home/xftroxgpx/build/2nonpkgs/rust.stuff/book/second-edition/book/ch10-01-syntax.html#in-method-definitions
#[derive(Debug)]
struct Point<T> {
    x: T,
//    #[allow(dead_code)]
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
//    fn own_x(self) -> T {
//        self.x
//    }
    fn set_x(&mut self, new_x: T) {
        self.x = new_x;
    }
}

pub trait MyType {
    fn new(arg: u32) -> Self;
}

#[derive(Debug)]
struct Moo(u32);

impl MyType for Moo { //also thanks for udoprog for trying something with traits but it's different that what I wanted because it shows different error: https://play.rust-lang.org/?gist=2054123cbee5c0e4fe36b45cf4024d70&version=nightly
    fn new(arg: u32) -> Self {
        Moo(arg)
    }
}

#[derive(Debug, Copy, Clone)]
struct CopyMoo(u32);

impl MyType for CopyMoo {
    fn new(arg: u32) -> Self {
        CopyMoo(arg)
    }
}

//use std::fmt::Debug;

macro_rules! some_type { //made into macro by durka42 on irc, thanks!
    ($T:ident) => {
        let mut p = Point { x: $T::new(5), y: $T::new(55) };
        println!("p = {:?}", p);
        let x: $T = *p.x();
        p.set_x($T::new(10));
        let y: $T = *p.x();
        println!("x={:?} y={:?}", x,y);//x has obsolete value
        println!("p = {:?}", p);
    }
}

fn main() {
    //XXX: ok so this is supposed to show you that using a Copy type can bypass borrowing/ownership rules until you later use a non-Copy type(if ever) and that this bypassing can make you write incorrect Rust programs which could use stale values.

    some_type!(CopyMoo);//change this to Moo(which is non-Copy) to see compile error!
    //some_type!(Moo);//ie. change above to this line!
}
