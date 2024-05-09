use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;

struct CustomHashSet<T> {
    elements: HashMap<T, Rc<T>>,
}

impl<T: Clone + Eq + Hash> CustomHashSet<T> {
    fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    fn insert(&mut self, value: T) {
        let cloned_value = value.clone();
        self.elements.insert(value, Rc::new(cloned_value));
    }

    fn get(&self, key: &T) -> Option<Rc<T>> {
        self.elements.get(key).cloned()
    }

    fn remove(&mut self, key: &T) {
        self.elements.remove(key);
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MyType<T>(T);

fn main() {
    let mut set = CustomHashSet::new();
    set.insert(MyType(42));
    set.insert(MyType(43));

    let elem_ref = set.get(&MyType(42));
    //XXX: can remove an element while holding a ref to another elem.
    //normally hashset wouldn't allow this because the returned ref holds whole set as borrowed.
    println!("{:?}", set.get(&MyType(43)));
    set.remove(&MyType(43));
    println!("{:?}", set.get(&MyType(43)));

    println!("{:?}", elem_ref); // Ensure elem_ref is used
}
