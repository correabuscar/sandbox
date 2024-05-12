struct Inner {
    name: String,
}

impl Inner {
    fn new(name: &str) -> Self {
        println!("Creating Inner: {}", name);
        Inner {
            name: name.to_string(),
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        println!("Dropping Inner: {}", self.name);
    }
}

struct Parent {
    inner1: Inner,
    inner2: Inner,
}

impl Parent {
    fn new() -> Self {
        println!("Creating Parent");
        Parent {
            inner1: Inner::new("Inner 1i"),
            inner2: Inner::new("Inner 2i"),
        }
    }
    fn new2(i1:Inner, i2:Inner) -> Self {
        println!("Creating Parent");
        Parent {
            inner1: i1,
            inner2: i2,
        }
    }
}

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping Parent");
    }
}

fn main() {
    {
        println!("Example1:");
        {
            let mut parent = Parent::new();
        }
        println!("Example2:");
        {
            //the order of the inners matter when dropping them
            let inner1 = Inner::new("New Inner 1");
            let inner2 = Inner::new("New Inner 2");
            //but it doesn't matter which parent/inners were created first
            //in either of the 3 examples in main()
            let mut parent = Parent::new2(inner1, inner2);
        }
        println!("Example3:");
        let mut parent = Parent::new();
        // Set inner fields to new Inner objects
        parent.inner1 = Inner::new("New Inner 1");
        parent.inner2 = Inner::new("New Inner 2");
        println!("Exiting inner scope");
    }
    println!("Exiting main scope");
}

