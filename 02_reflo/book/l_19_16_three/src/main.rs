trait Pilot {
    fn new() -> Self;
}

#[allow(non_camel_case_types)]
trait Pilot_i32 {
    fn new(typ: i32) -> Self;
}

#[allow(non_camel_case_types)]
trait Pilot_u8 {
    fn new(typ: u8) -> Self;
}

trait Wizard {
    //fn fly(&self);
    fn new() -> Self;
}

#[derive(Debug)]
struct Human {
    one: i32,
    id: String,
}

impl Wizard for Human {
    //fn fly(&self) {
    //}
    fn new() -> Self {
        Human {
            one: 3,
            id: String::from("wizard human"),
        }
    }
}
impl Pilot for Human {
    fn new() -> Self {
        Human {
            one: 2,
            id: String::from("pilot human"),
        }
    }
}

impl Human {
    fn new() -> Self {
        Human {
            one: 1,
            id: String::from("normal human"),
        }
    }
}

impl Pilot_i32 for Human {
    fn new(typ: i32) -> Self {
        Human {
            one: typ,
            id: String::from("pilot_i32"),
        }
    }
}

impl Pilot_u8 for Human {
    fn new(typ: u8) -> Self {
        //Human { one: typ as i32, id:String::from("pilot_u8") } //works too
        Human {
            one: i32::from(typ),
            id: String::from("pilot_u8"),
        } //suggested by clippy
    }
}

fn main() {
    let c = Human::new();
    println!("{:?}", c);
    let a = <Human as Pilot>::new();
    //let a=<Human as Human>::new();
    println!("{:?}", a);
    let a: Human = Pilot::new();
    println!("{:?}", a);
    let b = <Human as Wizard>::new();
    println!("{:?}", b);
    let b: Human = Wizard::new();
    println!("{:?}", b);

    let d: Human = Pilot_i32::new(-32);
    println!("{:?}", d);
    let d: Human = Pilot_u8::new(255);
    println!("{:?}", d);

    //ok so this is basically the same as using differently named new() functions, but more work.
}
