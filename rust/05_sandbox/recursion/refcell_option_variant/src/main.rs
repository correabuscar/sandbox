use std::cell::{Ref, RefCell, RefMut};
use std::mem::ManuallyDrop;

//XXX: just to see if it calls drop() on it if panic!() in happens in certain places!
#[derive(Debug)]
struct Wrapper<T:std::fmt::Debug> {
    inner: RefCell<T>,
}

impl<T:std::fmt::Debug> Wrapper<T> {
    const fn new(inner: T) -> Self {
        Wrapper {
            inner: RefCell::new(inner),
        }
    }

    // Add methods to interact with the inner RefCell<T> here
    fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

}

impl<T:std::fmt::Debug> Drop for Wrapper<T> {
    fn drop(&mut self) {
        // Perform any necessary cleanup logic here
        // For example, you might want to borrow_mut() the RefCell and perform cleanup
        //self.inner.borrow_mut().cleanup();
        //drop(self.inner);
        println!("Dropping {:?}", self);
    }
}


#[derive(Debug)]
struct MyVector<T:std::fmt::Debug, const N:usize> {
    //data: [ManuallyDrop<RefCell<Option<T>>>; N],
    data: [ManuallyDrop<Wrapper<Option<T>>>; N],
}

// XXX: this isn't actually true here, so we're lying, it's not thread-safe! (it would be if we'd be using 2 atomics arrays to restrict access for each index to only a specific thread(done this elsewhere))
// needed to can be used as a type for an immutable 'static' !
unsafe impl<T: std::fmt::Debug, const N: usize> Sync for MyVector<T, N> {}

impl<T:std::fmt::Debug, const N:usize> Drop for MyVector<T,N> {
    fn drop(&mut self) {
        let mut count:usize=0;
        for each in &mut self.data {
            println!("Dropping element with index: {}",count);
            //drop(*each)
            // Access the inner RefCell<Option<T>> and then drop it
            unsafe { ManuallyDrop::drop(each) };//XXX: else they won't be dropped, doh
            //*each=None;
            count+=1;
        }
        //println!("end of vector drop {:?}", self);//XXX: this prints the stale ones but shouldn't even be working here, kinda, i mean that's why the above is 'unsafe'
        println!("end of vector drop");
    }
}

// Implement Display for MyVector
impl<T: std::fmt::Debug, const N: usize> fmt::Display for MyVector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Start building the string representation
        write!(f, "MyVector {{ ")?;

        // Iterate through each element in the array
        for (i, item) in self.data.iter().enumerate() {
            // Print the index and the content of the element
            write!(f, "[{}: {:?}]", i, item)?;
            // Add a comma and space unless it's the last element
            if i < N - 1 {
                write!(f, ",\n")?;
            }
        }

        // End the string representation
        write!(f, " }}")
    }
}

//the `Drop` trait may only be implemented for local structs, enums, and unions: must be a struct, enum, or union in the current crate
//impl<T> Drop for RefCell<T> {
//    fn drop(&mut self) {
//        println!("Dropping RefCell {:?}", self);
//        // Perform any necessary cleanup logic here
//        // For example, you might want to call `take()` to release the inner value
//        // self.take();
//    }
//}

impl<T:std::fmt::Debug, const N:usize> MyVector<T,N> {
    const fn new() -> Self {
        //const ARRAY_REPEAT_VALUE: RefCell<Option<T>> = RefCell::new(None);//can't use generic parameters from outer item: use of generic parameter from outer item

        //let mut data:[ManuallyDrop<RefCell<Option<T>>>; N]=unsafe { std::mem::zeroed() };
        let mut data:[ManuallyDrop<Wrapper<Option<T>>>; N]=unsafe { std::mem::zeroed() };
        //let mut data = core::array::from_fn(|_foo| RefCell::new(None::<T>));//non-const fn
        //panic!("foo");
        let mut index=0;
        while index < N {
            //std::mem::forget(&data[index]);
            // E0493: destructor of `RefCell<Option<T>>` cannot be evaluated at compile-time value is dropped here
            // problem is, it thinks it needs to drop() the prev value which is the mem::zeroed() one.
            data[index]=ManuallyDrop::new(Wrapper::new(None));
            /* "In your case, you're using ManuallyDrop around RefCell<Option<T>>. This means that ManuallyDrop prevents the automatic dropping of the RefCell<Option<T>> instances themselves, but it doesn't prevent the automatic dropping of the T values stored within those RefCell<Option<T>> instances." -chatgpt 3.5 */
            index+=1;
            //panic!("foo");
        }
        //panic!("foo");
        MyVector {
            data,//: [ARRAY_REPEAT_VALUE; N],//Default::default(),
        }
    }

    fn borrow(&self, index: usize) -> Ref<Option<T>> {
        self.data[index].borrow()
    }

    fn borrow_mut(&self, index: usize) -> RefMut<Option<T>> {
        self.data[index].borrow_mut()
    }

    fn remove(&self, index: usize) {
        //old one gets auto dropped by rust
        *self.data[index].borrow_mut() = None;
    }

    fn insert(&self, index: usize, value: T) {
        *self.data[index].borrow_mut() = Some(value);
    }

    //fn unset(&self, index:usize) {
    //    drop(self.data[index]);
    //}
}

//#[derive(Debug)]
struct MyType(usize);

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping MyType({})",self.0);
        //panic!();//for tests, see the stack
    }
}

use std::fmt;
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here you define how MyType should be formatted when displayed.
        // For example, let's say you want to display it as "MyType(value)",
        // where "value" is the value inside the struct.
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here you define how MyType should be formatted when displayed.
        // For example, let's say you want to display it as "MyType(value)",
        // where "value" is the value inside the struct.
        write!(f, "{}", self.0)
    }
}

//only traits defined in the current crate can be implemented for types defined outside of the crate: impl doesn't use only types from inside the current crate
//impl fmt::Display for Option<MyType> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//            Some(inner) => write!(f, "{}", inner),
//            None => write!(f, "None"),
//        }
//    }
//}


const VECTOR_SIZE: usize = 5;

static A_STATIC:MyVector<MyType, VECTOR_SIZE> = MyVector::new();
//^ `RefCell<Option<MyType>>` cannot be shared between threads safely: `RefCell<Option<MyType>>` cannot be shared between threads safely

fn main() {
    let my_vector = MyVector::<MyType, VECTOR_SIZE>::new();
    //panic!("foo");
    my_vector.insert(0, MyType(1));
    my_vector.insert(1, MyType(2));
    my_vector.insert(2, MyType(3));
    //panic!("foo");

    println!("{}", my_vector);
    // Borrow an element
    let mut borrowed_element = my_vector.borrow_mut(0);

    // Define a value to insert
    let new_value = MyType(42);

    // Modify or remove other elements
    println!("Manually removing 1 element and inserting a 2nd");
    my_vector.remove(1);
    my_vector.remove(1);//no bad effects
    my_vector.insert(2, new_value);

    println!("{}", my_vector);
    println!("borrowed is:{:?}", borrowed_element);
    if let Some(ref mut elem)=*borrowed_element{
        elem.0=200;
    }
    println!("borrowed is:{:?}", borrowed_element);
    drop(borrowed_element);
    //my_vector.remove(0);//works too
    println!("{}", my_vector);
    my_vector.insert(0, MyType(101));

    println!("{}", my_vector);
}

