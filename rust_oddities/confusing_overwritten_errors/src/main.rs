#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct NewType(i32);

#[derive(Debug)]
enum HeyHey {
    Int(i32),
    Float(f64),
}

fn main() {
    //let mut v:Vec<HeyHey>=Vec::new();
    let mut v:Vec<NewType>=Vec::new();


    for i in 1..3 {
        println!("iter{}",i);

        let a:NewType;
        //let mut a:NewType;
        //let a:NewType=NewType(3);
        //let mut a:NewType=NewType(3);
        change(&mut a);//true but only happens without the "v.push(a)" below : "borrow of possibly uninitialized variable: `a`"
        v.push(a); //if this is uncommented then true error "borrow of possibly uninitialized variable: `a`" is overwritten by "borrow of moved value: `a`" and "value moved here, in previous iteration of loop"

        //let a:HeyHey;//fake error: "borrow of moved value: `a`" below
        //println!("{:?}",a); // true: "borrow of possibly uninitialized variable: `a`" and "use of possibly uninitialized `a`"
        //let mut a:HeyHey;//fake error: "borrow of moved value: `a`" below
        //let a:HeyHey=HeyHey::Int(2);//true: "cannot borrow `a` as mutable, as it is not declared as mutable"
        //let mut a:HeyHey=HeyHey::Int(2);//no errors with this!

        //change2(&mut a);// if NOT in a for loop, then true: "borrow of possibly uninitialized variable: `a`" and "use of possibly uninitialized `a`"
        //v.push(a);//if in a for loop, fake error: "value moved here, in previous iteration of loop"
    }
}

fn change(i: &mut NewType) {
//    *i=NewType(1);
}

fn change2(i: &mut HeyHey) {
    //*i=HeyHey::Int(1);
}

