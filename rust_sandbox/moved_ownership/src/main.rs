#[derive(Debug)]
struct Moo<T>(T);


struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let dest;
    {
        let src=Moo("some");
        dest=src;//moved? seems so
    }
    println!("dest={:?}",dest);
    let p3;
    {
        let m = Moo(6);//what's the lifetime of this ?
        //let p1 = Point { x: Moo(5), y: 10.4 };
        let p1 = Point { x: m, y: 10.4 }; //what's the lifetime of x ?
        let p2 = Point { x: "Hello", y: Moo(3.14) }; //what's the lifetime of y or Moo(3.14) ?
        //presumably either 'static ? OR, more likely, their ownership is just simply moved, eventually to p3

        p3 = p1.mixup(p2);
    }
    println!("p3.x = {:?}, p3.y = {:?}", p3.x, p3.y);
}

