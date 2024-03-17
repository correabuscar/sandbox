// src: https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

struct Human2;
impl Human2 {
    fn fly(&self) {
        println!("*waving arms furiously 2*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    <Human as Pilot>::fly(&person);
    Wizard::fly(&person);
    <Human as Wizard>::fly(&person);
    person.fly();
    Human::fly(&person);
    // FIXME: https://github.com/rust-lang/rust/issues/61729
    //<Human as Human>::fly(&person); // E0576: cannot find method or associated constant `fly` in `Human`  not found in `Human`
    //<Human as Human2>::fly(&person); // E0576: cannot find method or associated constant `fly` in `Human2`  not found in `Human2`
}
