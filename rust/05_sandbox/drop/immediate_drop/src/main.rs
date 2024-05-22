struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with name: {}", self.name);
    }
}

fn main() {
    println!("hi");

    // Create another instance of MyStruct and bind it to _var
    let _var = MyStruct { name: String::from("_var") };
    println!("after _var");

    // Create another instance of MyStruct and bind it to _
    let _ = MyStruct { name: String::from("_") };
    println!("after _ (see, was insta dropped above)");

    // Create an instance of MyStruct and bind it to var
    #[allow(unused_variables)]
    let var = MyStruct { name: String::from("var") };
    println!("after var");

    // The scope ends here, and all instances will be dropped
    println!("before bye");
}

