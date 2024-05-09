use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Eq, Hash, PartialEq, Debug)]
struct MyType(i32); // Wrapper type for i32

// Custom smart pointer type with compile-time borrow checking
struct CustomRef<'a, T> {
    data: &'a T,
    _marker: PhantomData<&'a T>, // PhantomData to indicate borrowing relationship
}

impl<'a, T> CustomRef<'a, T> {
    fn new(data: &'a T) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }

    // Add methods for accessing and modifying the data
    fn get(&self) -> &'a T {
        self.data
    }
}

// Custom hash set with compile-time borrow checking
struct CustomHashSet<'a, T: 'a> {
    elements: HashMap<MyType, &'a T>, // Store references to values
}

impl<'a, T: 'a + Eq + Hash> CustomHashSet<'a, T> {
    fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    fn insert(&mut self, key: MyType, value: &'a T) {
        self.elements.insert(key, value);
    }

    fn get(&self, key: &MyType) -> Option<CustomRef<'a, T>> {
        if let Some(value) = self.elements.get(key) {
            Some(CustomRef::new(value))
        } else {
            None
        }
    }

    fn remove(&mut self, key: &MyType) {
        self.elements.remove(key);
    }
}

fn main() {
    let mut set = CustomHashSet::new();
    let value1 = MyType(42); // Using MyType wrapper for i32
    let value2 = MyType(43); // Using MyType wrapper for i32
    set.insert(value1, &MyType(42));
    set.insert(value2, &MyType(43));

    let elem_ref = set.get(&MyType(42)).unwrap(); // Using MyType wrapper for i32
    set.remove(&MyType(43)); // Using MyType wrapper for i32

    println!("{:?}", elem_ref.get()); // Ensure elem_ref is used
}

