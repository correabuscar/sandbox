const PWD_AT_COMPILETIME: &'static str = env!("PWD");
fn main() {
    println!("Hello, world! {}", PWD_AT_COMPILETIME);
}
