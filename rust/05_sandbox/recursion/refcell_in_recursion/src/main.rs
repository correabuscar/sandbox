use std::cell::RefCell;

// Define a non-recursive struct
#[derive(Debug)]
struct CustomStruct {
    data: i32,
}

impl CustomStruct {
    // Constructor for CustomStruct
    const fn new(data: i32) -> Self {
        Self { data }
    }
}

// Function to recursively mutate RefCell<T>
fn recursive_mutation(refcell: &RefCell<CustomStruct>) {
    // Attempt to borrow the RefCell mutably
    if let Ok(mut borrowed) = refcell.try_borrow_mut() {
        // Mutate the inner value by dereferencing RefMut
        borrowed.data += 1;
        println!("About to recurse {:?}", borrowed);
        // Recursively call the function
        recursive_mutation(refcell);
    } else {
        // If borrow fails, we've reached the end of recursion or encountered a deadlock
        println!("End of recursion or deadlock {:?}", refcell);
    }
}

fn recursive_mutation2() {
    if let Ok(mut borrowed) = unsafe { FOO.try_borrow_mut() } {
        borrowed.data+=1;
        println!("About to recurse {:?}", borrowed);
        // Recursively call the function
        recursive_mutation2();
        borrowed.data-=1;
    } else {
        // If borrow fails, we've reached the end of recursion or encountered a deadlock
        println!("End of recursion or deadlock {:?}", unsafe { &FOO });
    }
}

fn recursive_mutation3() {
    if let Ok(mut borrowed) = unsafe { FOO.try_borrow_mut() } {
        borrowed.data+=1;
        println!("About to recurse {:?}", borrowed);
        // Recursively call the function
        recursive_mutation(unsafe {&FOO});
        borrowed.data-=1;
    } else {
        // If borrow fails, we've reached the end of recursion or encountered a deadlock
        println!("End of recursion or deadlock {:?}", unsafe { &FOO });
    }
}
static mut FOO:RefCell<CustomStruct>=RefCell::new(CustomStruct::new(0));

fn main() {
    // Create a RefCell containing a CustomStruct
    let refcell = RefCell::new(CustomStruct::new(0));

    // Perform mutable recursion
    recursive_mutation(&refcell);//XXX: detected because can access same refcell
    recursive_mutation2();//XXX: detected because can access same refcell
    recursive_mutation3();//XXX: detected because can access same refcell

    //let refcell_ref=unsafe { &FOO };
    let mut borrow=refcell.borrow_mut();
    borrow.data+=1;
    //drop(refcell);
}

