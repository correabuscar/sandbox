trait InCommon { // this doesn't work as I had hoped, but it does as they've envisioned it:)
    fn new() -> Self;
}

trait Wizard: InCommon {
    fn fly(&self);
    fn new() -> Self;
}

#[derive(Debug)]
struct Human {
    one: i32,
    two: String,
}

impl Wizard for Human {
    fn fly(&self) {
    }
    fn new() -> Self{
        Human { one:3, two:String::from("wizard") }
    }
}
impl InCommon for Human {
    fn new() -> Self{
        Human { one:2, two:String::from("three") }
    }
}

impl Human {
    fn new() -> Self{
        Human { one:1, two:String::from("two") }
    }
}

fn main() {
    let c=Human::new();
    println!("{:?}", c);
    let a=<Human as InCommon>::new();
    //let a=<Human as Human>::new();
    println!("{:?}", a);
    let b=<Human as Wizard>::new();
    println!("{:?}", b);
}
