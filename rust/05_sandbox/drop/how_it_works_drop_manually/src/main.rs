use std::mem::ManuallyDrop;

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
    inner1: ManuallyDrop<Inner>,
    inner2: ManuallyDrop<Inner>,
}

impl Parent {
    fn new() -> Self {
        println!("Creating Parent");
        Parent {
            inner2: ManuallyDrop::new(Inner::new("Inner 2i")),
            inner1: ManuallyDrop::new(Inner::new("Inner 1i")),
        }
    }
    fn new2(i1:Inner, i2:Inner) -> Self {
        println!("Creating Parent");
        Parent {
            inner1: ManuallyDrop::new(i1),
            inner2: ManuallyDrop::new(i2),
        }
    }
}

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping Parent, manually dropping begins:");
        //Dropping these in reverse order (manually), just for kicks
        unsafe { std::mem::ManuallyDrop::drop(&mut self.inner2) };
        unsafe { std::mem::ManuallyDrop::drop(&mut self.inner1) };
        //technically can still see/use them hereafter still, but it's UB?!
        println!("Dropping Parent done manually dropping");
    }
}

fn main() {
    {
        println!("Example1:");
        {
            let _parent = Parent::new();
        }
        println!("Example2:");
        {
            //the order of the inners creation does not matter when dropping them, they're dropped in field order
            let inner2 = Inner::new("New Inner 2");
            let inner1 = Inner::new("New Inner 1");
            //but it doesn't matter which parent/inners were created first
            //in either of the 3 examples in main()
            let _parent = Parent::new2(inner1, inner2);
        }
        println!("Example3:");
        let mut parent = Parent::new();
        // Set inner fields to new Inner objects
        // FIXME: the old ones are leaked:
        parent.inner2 = ManuallyDrop::new(Inner::new("New Inner 2"));
        parent.inner1 = ManuallyDrop::new(Inner::new("New Inner 1"));
        println!("Exiting inner scope");
    }
    println!("Exiting main scope");
}

