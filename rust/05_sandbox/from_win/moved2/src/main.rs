use std::ops::AddAssign;

#[derive(Debug)]
struct E32(i32);

struct Point {
    x: E32,

    y: E32,
}

impl AddAssign for E32 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}


fn main() {
    let mut a = Point {
        x: E32(1),
        y: E32(2),
    };

    a.x += E32(1);

    let b = Point { y: E32(1), ..a }; // value moved here
    a.x += E32(1);//value borrowed here after move

    println!("{:?}", a.x);

}
