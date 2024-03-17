extern crate communicator;

fn main() {
    communicator::client::connect();
}
//We use the extern crate command to bring the communicator library crate into scope. Our package now contains two crates. Cargo treats src/main.rs as the root file of a binary crate, which is separate from the existing library crate whose root file is src/lib.rs. This pattern is quite common for executable projects: most functionality is in a library crate, and the binary crate uses that library crate. As a result, other programs can also use the library crate, and it’s a nice separation of concerns.
//src: file://${HOME}/build/2nonpkgs/rust.stuff/book/second-edition/book/ch07-02-controlling-visibility-with-pub.html

