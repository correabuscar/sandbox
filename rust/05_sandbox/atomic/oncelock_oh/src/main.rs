//ok so this exists... which I could use in place of AtomicOnceOptionU32 and for other types too!
//but is it atomic ie. guarantees writes visiblity across cores?
use std::sync::OnceLock;
// "A synchronization primitive which can be written to only once."
// "This type is a thread-safe OnceCell, and can be used in statics."
// "Using OnceCell to store a function’s previously computed value (a.k.a. ‘lazy static’ or
// ‘memoizing’"
//src: https://doc.rust-lang.org/std/sync/struct.OnceLock.html

static VALUE: OnceLock<Option<u32>> = OnceLock::new();

fn initialize_value() -> Option<u32> {
    // Some expensive initialization logic
    Some(42)
}

fn get_value() -> &'static Option<u32> {
    VALUE.get_or_init(initialize_value)
}

fn main() {
    println!("Value: {:?}", get_value());
}

