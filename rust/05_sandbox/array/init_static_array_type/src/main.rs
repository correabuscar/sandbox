//XXX: T can't be Copy, else this would work simpler!

trait DefaultValue {
    const DEFAULT: Self;
    //const fn default() -> MyType;//can't have 'const fn'
}

#[derive(Debug)]
struct SomeArray<const N: usize, T> {
    inner_array: [T; N],
}

impl<const N: usize, T: DefaultValue> SomeArray<N, T> {
    const fn new() -> Self {
        Self { inner_array: [T::DEFAULT; N] }
    }
}

// Example implementation for i32
impl DefaultValue for i32 {
    const DEFAULT: Self = 2;
}

#[derive(Debug)]
struct MyType(i32);

impl DefaultValue for MyType {
    //const DEFAULT: Self = MyType(1);
    const DEFAULT: Self = MyType::new();
    /* can't have const fn from trait
    const fn default() -> MyType {
        Self::DEFAULT
    }*/
}

impl MyType {
    const fn new() -> MyType {
        //Self::DEFAULT
        MyType(1)
    }
}

fn main() {
    let my1: SomeArray<5, i32> = SomeArray::new();
    println!("{:?}", my1);
    let my2: SomeArray<5, MyType> = SomeArray::new();
    println!("{:?}", my2);
}

