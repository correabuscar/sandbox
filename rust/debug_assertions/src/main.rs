fn main() {
    print!("Hello, world! debug_assertions is ");
    #[cfg(debug_assertions)]
    println!("true");
    #[cfg(not(debug_assertions))]
    println!("false");

    debug_assert!(1==2,"debug_assertions is true");
}
