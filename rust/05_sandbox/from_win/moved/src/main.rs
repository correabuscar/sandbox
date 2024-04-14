use std::ops::AddAssign;
use std::ops::Add;

#[derive(Debug)]
struct E32(i32);

struct Point {
    x: E32,

    y: E32,
}

impl AddAssign for E32 {
    fn add_assign(&mut self, other: Self) {
        self.0+=other.0;

    }
    // fn add_assign(&mut self, other: i32) {
    //     self.0+=other;

    // }
}

impl Add for E32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return E32(self.0+other.0);

    }
}


fn main() {
    let mut a=E32(1);
    let c=a;
    //a+=E32(1);//moved
    //a=a+E32(1);//moved
    a=E32(2);//not moved due to rebind ie. overwriting the old value(not reusing it)
    println!("c={c:?}");
    println!("a={a:?}");

    //ok so, values get moved, not bindings - obviously now.

    let mut a = Point {
        x: E32(1),
        y: E32(2),
    };

    a.x += E32(2);

    //let b = Point { y: e32(1), ..a };

    //let b = Point { y: e32(1), x: a.x }; // why a.x didn't move here? wtf



    let mut c = a.x; //  value moved here, makes sense, unless the next line is allowed:
    a.x = E32(30);//so if I change the binding a.x (after it moved) to another value, a.x is no longer moved, tf?!
    //a.x = a.x+E32(40);//moved because it uses(moves) the old value
    //a.x += E32(3);//but if I change the existing value (without rebinding), then it's still moved. (comment out the above two, obviously)


    c+=E32(10);
    println!("{:?}", c);
    //println!("{:?}", b.x);
    println!("{:?}", a.x)
}
