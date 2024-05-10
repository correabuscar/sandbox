use std::cell::{Ref, RefCell, RefMut};
use std::mem::ManuallyDrop;


#[derive(Debug)]
struct MyVector<T, const N:usize> {
    data: [ManuallyDrop<RefCell<Option<T>>>; N],
}

impl<T, const N:usize> MyVector<T,N> {
    const fn new() -> Self {
        //const ARRAY_REPEAT_VALUE: RefCell<Option<T>> = RefCell::new(None);//can't use generic parameters from outer item: use of generic parameter from outer item

        let mut data:[ManuallyDrop<RefCell<Option<T>>>; N]=unsafe { std::mem::zeroed() };
        //let mut data = core::array::from_fn(|_foo| RefCell::new(None::<T>));//non-const fn
        let mut index=0;
        while index < N {
            //std::mem::forget(&data[index]);
            // E0493: destructor of `RefCell<Option<T>>` cannot be evaluated at compile-time value is dropped here
            // problem is, it thinks it needs to drop() the prev value which is the mem::zeroed() one.
            data[index]=ManuallyDrop::new(RefCell::new(None));
            /* "In your case, you're using ManuallyDrop around RefCell<Option<T>>. This means that ManuallyDrop prevents the automatic dropping of the RefCell<Option<T>> instances themselves, but it doesn't prevent the automatic dropping of the T values stored within those RefCell<Option<T>> instances." -chatgpt 3.5 */
            index+=1;
        }
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
        *self.data[index].borrow_mut() = None;
    }

    fn insert(&self, index: usize, value: T) {
        *self.data[index].borrow_mut() = Some(value);
    }
}

#[derive(Debug)]
struct MyType(i32);

const VECTOR_SIZE: usize = 5;

fn main() {
    let my_vector = MyVector::<MyType, VECTOR_SIZE>::new();
    my_vector.insert(0, MyType(1));
    my_vector.insert(1, MyType(2));
    my_vector.insert(2, MyType(3));

    // Borrow an element
    let mut borrowed_element = my_vector.borrow_mut(0);

    // Define a value to insert
    let new_value = MyType(42); // Replace 42 with your desired value

    // Modify or remove other elements
    my_vector.remove(1);
    my_vector.insert(2, new_value);

    println!("{:?}", borrowed_element);
    if let Some(ref mut elem)=*borrowed_element{
        elem.0=200;
    }
    println!("{:?}", borrowed_element);
    drop(borrowed_element);//fails at runtime, w/o this!
    //my_vector.remove(0);//works too
    println!("{:?}", my_vector);
    my_vector.insert(0, MyType(101));

    println!("{:?}", my_vector);
    // Continue using borrowed_element
}

