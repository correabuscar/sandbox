//XXX: T can't be Copy, else this would work simpler!
//XXX: trying to use trait named Default, failing!
//XXX: using unsafe mem::zeroed() success :))


#[derive(Debug)]
struct SomeArray<const N: usize, T> {
    inner_array: [T; N],
}

//impl<const N: usize, T: Default> SomeArray<N, T> {
impl<const N: usize, T> SomeArray<N, T> {
    const fn new() -> Self {
        //Self { inner_array: [T::default(); N] }//XXX: fail, need T:Copy
        Self { inner_array: unsafe { std::mem::zeroed() }}
    }
}

//#[derive(Debug,Default)] //works the same, too
#[derive(Debug)]
struct MyType(i32);

//impl Default for MyType {
//    fn default() -> Self {
//        MyType(0) // Assuming the default value is 0
//    }
//}

fn main() {
    let my1: SomeArray<5, i32> = SomeArray::new();
    println!("{:?}", my1);
    let my2: SomeArray<5, MyType> = SomeArray::new();
    println!("{:?}", my2);
}

