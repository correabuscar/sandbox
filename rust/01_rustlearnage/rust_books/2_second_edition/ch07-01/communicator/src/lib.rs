//#![allow(dead_code)]
//^ The default state of all code in Rust is private: no one else is allowed to use the code. If you don’t use a private function within your program, because your program is the only code allowed to use that function, Rust will warn you that the function has gone unused.
//After we specify that a function like client::connect is public, not only will our call to that function from our binary crate be allowed, but the warning that the function is unused will go away. Marking a function as public lets Rust know that the function will be used by code outside of our program. Rust considers the theoretical external usage that’s now possible as the function “being used.” Thus, when a function is marked public, Rust will not require that it be used in our program and will stop warning that the function is unused.
//src: file://${HOME}/build/2nonpkgs/rust.stuff/book/second-edition/book/ch07-02-controlling-visibility-with-pub.html

pub mod client; //aka client.rs

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

mod network2;
mod network3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        //client::connect(); //fail, paths are always relative to the current module, which here is tests; The only exception is in a use statement, where paths are relative to the crate root by default. XXX: now it won't fail due to 'use super::client;' below!
        ::client::connect();
        //Or, we can use super to move up one module in the hierarchy from our current module, like this:
        super::client::connect();
        //src: file://${HOME}/build/2nonpkgs/rust.stuff/book/second-edition/book/ch07-03-importing-names-with-use.html#using-super-to-access-a-parent-module
    }
    //The super:: functionality changes the path you give to use so it is relative to the parent module instead of to the root module.
    //For these reasons, in the tests module especially, use super::something is usually the best solution.
    use super::client;//this affects the whole scope, so fn it_works() too!
    #[test]
    fn something() {
        client::connect();
    }
}
