use std::mem;

fn main() {
  println!("{} {}", mem::size_of::<usize>(), <usize>::max_value());
}
