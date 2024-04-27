mod submod {
    use super::foo::my_module;
    pub fn foo() {
        my_module::private_function(); //fails, good, this is expected!
        super::foo::my_module::private_function();
    }
}
mod foo {
    pub mod my_module {
        pub(super) fn private_function() {
            println!("This function is now public (previously private)");
        }

        pub(crate) fn public_function() {
            println!("This is a public function");
            private_function(); // Now allowed because private_function is public
        }
    }

    pub fn main() {
        my_module::public_function();
        my_module::private_function();
        super::submod::foo();
    }
}

fn main() {
    foo::main();
}
