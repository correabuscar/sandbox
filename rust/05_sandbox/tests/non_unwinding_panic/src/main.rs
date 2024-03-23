#![feature(panic_always_abort)]

//use std::fmt::{Display, self};
//
//struct MyStruct;
//
//impl Display for MyStruct {
//    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
//        todo!(); // good but let's try infinite panics:
//        //let instance = MyStruct;
//
//        //this double panic used to be catchable, ie. https://github.com/rust-lang/rust/issues/97181#issuecomment-1132157218
//        //println!("{} {} {}", false, "oh no, '{}' was unexpected", instance); //this is caught
////        static BEEN_HERE_TIMES:AtomicU64=AtomicU64::new(0);
////        BEEN_HERE_TIMES.fetch_add(1, Ordering::SeqCst);
////        let i = BEEN_HERE_TIMES.load(Ordering::SeqCst);
////        assert!(false, "oh no displaynum={:?}, '{}' was unexpected", i,instance);
//        //panic!("unreachable");
//    }
//}

fn main() {
    println!("Hello, world!");
    //std::process::abort();

    // Set the custom panic handler
    //panic::set_hook(Box::new(custom_panic_handler));

    // Trigger a panic
    //panic!("This panic will abort without unwinding the stack");
	let _d = Double;
	panic!("once");
}

#[should_panic]
#[test]
fn test_should_panic() {
    //let inst=MyStruct;
    //std::panic::always_abort();
    // Cause a non-unwinding panic using std::panic::abort()
    //std::process::abort();
    //std::panic::abort();
    //panic!("on purpose {} foo", inst);
	let _d = Double;
	panic!("once");
}

//use std::panic;
//
//// Custom panic handler that aborts the program without unwinding the stack
//fn custom_panic_handler(info: &panic::PanicInfo<'_>) {
//    // Your panic handling logic goes here
//    //info.can_unwind();
//    panic!("Custom panic handler invoked: {:?}", info);
//
//    // Abort the program without unwinding the stack
//    //std::process::abort();
//}


//src: https://github.com/rust-lang/rust/pull/110975#issue-1689377609
struct Double;

impl Drop for Double {
    fn drop(&mut self) {
        //std::panic::catch_unwind(|| panic!("twice"));
        panic!("in Double's drop");//so just panic in Drop and it will cause a non-unwinding panic!
    }
}


