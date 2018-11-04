#![allow(unused_variables)]

#[derive(Clone,Debug)]//non Copy
struct A(i32);

fn main() {
    let x:i32 = 5;
    let y = x;//copy, instead of move!
    println!("{:?}",x);//can! it was only copied, implicitly!
    let y = x.clone();//clone
    println!("{:?}",x);//still can, obviously!

    let x2:A = A(5);
    let y2 = x2.clone();//clone
    println!("{:?}",x2);//can! it was cloned!
    let y2 = x2;//move
    //println!("{:?}",x2);//can't! it's moved!
}

