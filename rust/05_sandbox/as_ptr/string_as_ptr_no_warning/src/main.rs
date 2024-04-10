// see: https://github.com/rust-lang/rust/issues/90449
//
unsafe fn foo(p: *const u8) {
    println!("{:?}", *p as char);
    println!("{:?}", *p);
}
fn main() {
    let pointer = String::from("Hello world!").as_ptr(); //no warning is bad

    unsafe {
        println!("{:?}", *pointer as char);
        foo(pointer);
        foo(String::from("Hello world!").as_ptr());// correctly no warning here tho.
    }
}
