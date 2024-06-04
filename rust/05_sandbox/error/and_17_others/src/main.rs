trait Reconcile {
    fn reconcile(&self);
}

// Implementing the trait for some types
impl Reconcile for bool {
    fn reconcile(&self) {
        println!("Reconciling bool");
    }
}

impl Reconcile for i8 {
    fn reconcile(&self) {
        println!("Reconciling i8");
    }
}

impl Reconcile for i16 {
    fn reconcile(&self) {
        println!("Reconciling i16");
    }
}

impl Reconcile for i32 {
    fn reconcile(&self) {
        println!("Reconciling i32");
    }
}

impl Reconcile for i64 {
    fn reconcile(&self) {
        println!("Reconciling i64");
    }
}

impl Reconcile for u8 {
    fn reconcile(&self) {
        println!("Reconciling u8");
    }
}

impl Reconcile for u16 {
    fn reconcile(&self) {
        println!("Reconciling u16");
    }
}

impl Reconcile for u32 {
    fn reconcile(&self) {
        println!("Reconciling u32");
    }
}

impl Reconcile for i128 {
    fn reconcile(&self) {
        println!("Reconciling u32");
    }
}

impl Reconcile for u128 {
    fn reconcile(&self) {
        println!("Reconciling u32");
    }
}

fn process<T: Reconcile>(item: T) {
    item.reconcile();
}

fn main() {
    let value = String::from("This will cause an error");
    process(value); // This line will cause a compilation error
}

