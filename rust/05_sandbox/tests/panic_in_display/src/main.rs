#![feature(panic_always_abort)]
//
//src: https://github.com/rust-lang/rust/issues/97181
//this double panic no longer shows stacktrace due to https://github.com/rust-lang/rust/pull/110975

use std::fmt::{Display, self};

struct MyStruct;
struct MyStruct2;
impl Display for MyStruct2 {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        //todo!()
        let instance = MyStruct;
        panic!("oh1 no, '{}' was unexpected", instance);
    }
}

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instance2 = MyStruct2;
        panic!("oh2 no, '{}' was unexpected", instance2);
        //todo!()
    }
}

fn main() {
    let instance = MyStruct;

    std::panic::always_abort();//issue: https://github.com/rust-lang/rust/issues/122940
    println!("{}", instance);
    //assert!(false, "oh no, '{}' was unexpected", instance);
}
