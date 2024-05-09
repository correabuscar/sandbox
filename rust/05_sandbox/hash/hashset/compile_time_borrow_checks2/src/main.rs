use std::collections::hash_map::RandomState;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::fmt::Debug;

// Define the wrapper type MyType<T>
#[derive(Debug, Eq, PartialEq)]
struct MyType<T> {
    data: T,
}

// Implementing Hash for MyType<T> requires Hash for T
impl<T> Hash for MyType<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

// Custom hash set using compile-time borrow checks
struct CustomHashSet<'a, T> {
    data: std::collections::HashMap<MyType<T>, (), RandomState>,
    _marker: PhantomData<&'a T>,
}

// Implementation for CustomHashSet
impl<'a, T:Debug> CustomHashSet<'a, T>
where
    T: 'a + Hash + Eq,
{
    // Constructor for CustomHashSet
    fn new() -> Self {
        CustomHashSet {
            data: std::collections::HashMap::default(),
            _marker: PhantomData,
        }
    }

    // Method to insert an element into the set
    fn insert(&mut self, element: MyType<T>) {
        self.data.insert(element, ());
    }

    // Method to remove an element from the set
    fn remove(&mut self, element: &MyType<T>) {
        self.data.remove(element);
    }

    // Method to print elements in the set
    fn print_elements(&self) {
        for key in self.data.keys() {
            println!("{:?}", key);
        }
    }
}

fn main() {
    let mut set = CustomHashSet::new();

    let a = MyType { data: 1 };
    let b = MyType { data: 2 };

    set.insert(a);
    set.insert(b);

        let borrowed_a = MyType { data: 1 };
        //set.remove(&borrowed_a);
        set.remove(&MyType { data: 2 });
        println!("borrowed still: {:?}", borrowed_a);

    set.print_elements();
}

