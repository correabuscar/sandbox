const BUILD_DATE: &'static str = env!("BUILD_DATE"); //set by build.rs

fn main() {
    println!("BUILD_DATE={}", BUILD_DATE);
}
