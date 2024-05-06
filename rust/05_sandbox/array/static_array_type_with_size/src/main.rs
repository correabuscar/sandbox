#[derive(Debug)]
struct MyType<const N: usize> {
    data: [i32; N],
}

impl<const N: usize> MyType<N> {
    fn new() -> Self {
        Self {
            data: [0; N], // Initialize the array with default values (in this case, 0)
        }
    }

    // You can implement methods specific to MyType here
}


fn main() {
    let a:MyType<5>=MyType::new();
    println!("Hello, world! {:?}",a.data);
}
