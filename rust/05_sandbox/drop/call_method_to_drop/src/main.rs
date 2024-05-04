struct MyStruct;
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct");
    }
}
impl MyStruct {
    // Define a method to be called instead of drop
    fn fancyname(self) {
        // Call any other methods or perform other actions here
        println!("Calling fancyname method");

        // Explicitly drop self
        drop(self);
        println!("Exiting fancyname method");
    }
}

fn main() {
    {
    let inst = MyStruct;
    // Call the fancyname method on inst
    inst.fancyname(); // This will drop inst after the method call
    //drop(inst);//E0382: use of moved value: `inst` value used here after move
    }//won't be re-dropped

}

