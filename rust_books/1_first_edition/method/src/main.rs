struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn area2(self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    //static method
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    //access static method via Circle:: not c.
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());
    println!("{}", c.area2());
//    println!("{}", Circle::area(&c));//moved
    let d = Circle::new(0.0, 0.0, 3.0);
    println!("{}", Circle::area(&d));
    println!("{}", Circle::area2(d));
//    println!("{}", d.area());//moved!
}
