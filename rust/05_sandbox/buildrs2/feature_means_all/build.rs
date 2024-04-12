fn main() {
    #[cfg(feature="one")]
    eprintln!("used feature one");
    #[cfg(feature="two")]
    eprintln!("used feature two");
    #[cfg(feature="ideally-excluded-from-all")]
    panic!("fancy testing")

}
