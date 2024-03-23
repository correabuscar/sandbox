//src: https://github.com/rust-lang/rust/pull/110975#issue-1689377609
struct Double;

impl Drop for Double {
    fn drop(&mut self) {
        // 2 panics are active at once, but this is fine since it is caught.
        std::panic::catch_unwind(|| panic!("twice"));
    }
}


fn main() {
   // println!("Hello, world!");
	let _d = Double;
	panic!("once");
}
