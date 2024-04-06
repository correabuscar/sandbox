//mod lib2;

extern "C" {
    fn add(left: usize, right: usize) -> usize;
}

#[link(name = "unified_bin_and_lib")]
extern {}

fn main() {
    println!("Hello, world!{}",
             unsafe { add(1,2) }
             );
}
