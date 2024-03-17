#![warn(clippy::all)]
#![warn(clippy::pedantic)]

#[macro_use]
extern crate lazy_static;

//https://github.com/rust-lang/rust/issues/57633#issuecomment-454668360

use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt::Display;

trait Value: Send + Display {
    fn box_clone(&self) -> Box<dyn Value>;
}

//works with just these:
/*impl Value for isize {
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new((*self).clone())
    }
}

impl Value for String {
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new((*self).clone())
    }
}*/

//XXX so this implements box_clone for Box too, and thus .clone() below ends up calling this .box_clone() !
//hence infinite recursion
impl<T: 'static + Send + Clone + Display> Value for T {
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new(
            (*self)
            .clone()
            )
    }
}

#[derive(Clone)]
struct S {
    value: Box<dyn Value>
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Box<dyn Value> {
        self.box_clone() //THIS
    }
}

// see other (proper) use of lazy_static! here: https://github.com/fortanix/rust-sgx/blob/jb/sgx-detect/sgxs-tools/src/sgx_detect/proc_macro.rs#L122-L294
lazy_static! {
    static ref REGISTRY: Mutex<HashMap<String, S>> = {
        Mutex::new(HashMap::new())
    };
}

impl REGISTRY {
    fn get(&self, key: &str) -> Option<S> {
        self.lock().unwrap().get(&String::from(key)).map(|s| s.clone())
    }

    fn set(&self, key: &str, value: S) -> Option<S> {
        self.lock().unwrap().insert(String::from(key), value)
    }
}

fn main() {
    REGISTRY.set("foo", S { value: Box::new(String::from("hello world")) });
    REGISTRY.set("bar", S { value: Box::new(123) });

    println!("{}", REGISTRY.get("foo").unwrap().value);
    println!("{}", REGISTRY.get("bar").unwrap().value);
}
