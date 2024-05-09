use std::collections::hash_map::RandomState;
use std::hash::Hash;
use std::marker::PhantomData;
use std::fmt::Debug;

// Define the wrapper type MyType<T>
#[derive(Debug, Eq, PartialEq, Hash)]
struct MyType<T> {
    data: T,
}

// Implementing Hash for MyType<T> requires Hash for T
//impl<T> Hash for MyType<T>
//where
//    T: Hash,
//{
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        self.data.hash(state);
//    }
//}

// Custom hash set using compile-time borrow checks
struct CustomHashSet<'a, T> {
    data: std::collections::HashMap<T, (), RandomState>,
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
    fn insert(&mut self, element: T) {
        self.data.insert(element, ());
    }

    // Method to remove an element from the set
    fn remove(&mut self, element: &T) {
        self.data.remove(element);
    }

    // Method to print elements in the set
    fn print_elements(&self) {
        for key in self.data.keys() {
            println!("{:?}", key);
        }
    }
    // Method to get a borrow of an element
    fn get(&self, key: &T) -> Option<&T> {
        self.data.get_key_value(key).map(|(k, _)| k)
    }
}

fn main() {
    let mut set = CustomHashSet::new();

    let a = MyType { data: 1 };
    let b = MyType { data: 2 };

    set.insert(a);
    set.insert(b);

        //let borrowed_a = MyType { data: 1 };
        let borrowed_a = set.get(&MyType { data: 1 });
        //set.remove(&borrowed_a);
        set.remove(&MyType { data: 2 });
        println!("borrowed still: {:?}", borrowed_a);

    set.print_elements();
}

