//src: file://${HOME}/build/2nonpkgs/rust.stuff/book/second-edition/book/ch10-01-syntax.html#in-method-definitions
#[derive(Debug)]
struct Moo<T>(T);


#[derive(Debug)]
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
        //println!("{:?}", p1);//that's right, moved!
        //println!("{:?}", p2);//that's right, moved!
        println!("{:?}", p3);//into p3
    }
    println!("p3.x = {:?}, p3.y = {:?}", p3.x, p3.y);
}

