fn main() {
    #[cfg(not(set_this))]
    panic!("!!!!!! Attempted to compile build.rs without having set the #[cfg(set_this)]");

    #[cfg(set_this)]
    eprintln!("!!! success from build.rs");//not seen, unless '-vv'
}
