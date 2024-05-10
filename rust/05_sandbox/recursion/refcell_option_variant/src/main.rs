use std::cell::{Ref, RefCell, RefMut};

const VECTOR_SIZE: usize = 5;

#[derive(Debug)]
struct MyVector<T> {
    data: [RefCell<Option<T>>; VECTOR_SIZE],
}

impl<T> MyVector<T> {
    fn new() -> MyVector<T> {
        MyVector {
            data: Default::default(),
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

fn main() {
    let my_vector = MyVector::new();
    my_vector.insert(0, MyType(1));
    my_vector.insert(1, MyType(2));
    my_vector.insert(2, MyType(3));

    // Borrow an element
    let borrowed_element = my_vector.borrow(0);

    // Define a value to insert
    let new_value = MyType(42); // Replace 42 with your desired value

    // Modify or remove other elements
    my_vector.remove(1);
    my_vector.insert(2, new_value);
    //my_vector.remove(0);//XXX: fails at runtime, obviously!

    println!("{:?}", borrowed_element);
    println!("{:?}", my_vector);
    // Continue using borrowed_element
}

