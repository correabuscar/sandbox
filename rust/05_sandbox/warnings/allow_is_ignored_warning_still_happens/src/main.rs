//use std::alloc::Layout;
fn foo(s:&str) {
    println!("{}",s);
}
fn main() {
    panic!("foo");
    #[allow(unreachable_code)]
    { // Warning points to this block
        //let layout = Layout::from_size_align(1024, 8).unwrap();
        //std::alloc::handle_alloc_error(layout);
        foo("oh hi");
        foo("Hello, world!");
    }
    //No warnings if you comment these out:
    #[allow(unreachable_code)]
    foo("Hello, world!");
}

