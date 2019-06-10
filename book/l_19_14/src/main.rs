use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
    let p1=Point { x: 1, y: 0 };
    let p2=Point { x: 2, y: 3 };
    let p1c=p1.clone();
    let p2c=p2.clone();
    let p4=p1.clone()+p2.clone();
    let p3=p1+p2;
    assert_eq!(p3, p4);
    assert_eq!(p3, p2c+p1c);
}
