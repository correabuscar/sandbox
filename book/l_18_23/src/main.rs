fn main() {

    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    match origin {
        Point { x, y:_, z:_ } => println!("x is {}", x),
    }

/*    match origin {
        Point { x, _,_ } => println!("x is {}", x), //E0025: field `_` bound multiple times in the pattern  multiple uses of `_` in pattern
    }*/
}
