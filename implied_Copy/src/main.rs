#![allow(non_snake_case)] //avoids: warning: crate `implied_Copy` should have a snake case name such as `implied_copy`


#[derive(Debug)]
//struct Moo(i32);
//struct Moo<'a> {
struct Moo {
    num: u32,
    //dropcount: &'a mut u32, //a ref to the global drop count, or something
}

struct Foo<'a> {
    x: &'a Moo,
}

////impl<'a> Drop for Moo<'a> {
//impl Drop for Moo {
//    fn drop(&mut self) {
//      //  *self.dropcount += 1;
//        println!("Dropping {:?}", self);
//    }
//}

fn main() {
    let g;
    {
        //let mut count = 0; //global drop count
        //let _m = Moo {
        //    num: 5,
        //    //dropcount: &mut count,
        //};
        //let y = &_m; //will not compile ok, error[E0597]: `_m` does not live long enough
        let y=&Moo{num:3};//compiles ok due to some kind of implied Copy trait!? unless you impl Drop; or let z=5; let y=&Moo{num:z}; - it's rvalue static promotion (thanks scottmcm (irc) )
        // ^ This is the same as `let _y = 5; let y = &_y;`.
        // according to scottmcm (irc):
        // "that looks like rvalue static promotion"
        // "yup, you can change the line to `let y: &'static _ =&Moo{num:5};` and it's still happy"
        // "if you do &SOME_CONST, rust will essentially go "oh, that's const.  Let me put that in an immutable static for you so it lives forever""
        // "so the difference is that the Release y lives forever, but the Debug one lives only for the inner block, and thus not long enough for g" (switching Mode between Release/Debug in this https://play.rust-lang.org/?gist=16f2a4bac7bea897136468f6cdb7405e&version=nightly )
        // if I do impl Drop for Moo, that let y=&Moo{num:5};  thing fails also
        // "yup, because rvalue static promotion doesn't run on things with Drop impls -- it would
        // be noticable when the drop ran, unlike with non-drop things"
        // " it happens if you have a & to a constant literal without a drop impl, regardless of
        // whether you write 'static"
        // XXX: let z=5; let y=&Moo{num:z};  this prevents rvalue static promotion
//        let z=3;let y=&Moo{num:z};// yep, doesn't compile now;
//        let y=&Box::new(Moo{num:5});//same


        let f = Foo { x: y };

        println!("{:?}", f.x);
        println!("{:?}", y);
        g = f;
    }
    println!("{:?}", g.x);
    //so, this is not true then?: "So why do we need a lifetime here? We need to ensure that any reference to a Foo cannot outlive the reference to an i32 it contains." src: https://doc.rust-lang.org/book/first-edition/lifetimes.html#in-structs
}

