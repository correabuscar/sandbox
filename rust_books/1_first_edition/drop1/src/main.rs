#[derive(Debug)]
struct Echo<'a> {
    num: u32,
    dropcount: &'a mut u32,//a ref to the global drop count, or something
}

impl<'a> Drop for Echo<'a> {
    fn drop(&mut self) {
        *self.dropcount += 1;
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let mut count = 0;//global drop count
    println!("Started");

    let _ = Echo {
        num: 5,
        dropcount: &mut count,
    }; //this is dropped here...
    assert_eq!(1, count);
    println!("After"); //...before this.
    let _a = Echo { //the _ prefix will stop a warning for var unused
        num: 6,
        dropcount: &mut count,
    }; //Is this dropped...
    //assert_eq!(1, count);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
    assert_eq!(1, *_a.dropcount);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
    let _a = 6; // ...after this line? nope  TODO: feature request Rust should drop the shadowed _a here;
    //assert_eq!(1, count);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
    {
        //assert_eq!(1, count);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
        let _a = 7; // it shouldn't be dropped here
        //assert_eq!(1, count);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
    } //or here
    //assert_eq!(1, count);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
    println!("After2");
    //assert_eq!(1, count);//error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
} //it is here where it's actually dropped!
