fn main() {
}

#[test]
fn test1_correct() {
    let mut mutable_value = Some(5);
    println!();
    println!("Mutable value before: {:?}", mutable_value);

    // Reassigning the binding
    mutable_value = Some(10);

    // Pattern matching with mutable reference to the inner value
    if let Some(ref mut inner) = mutable_value {
        *inner += 1; // Mutating the inner value
        println!("Inner value: {}", inner); // Prints: Inner value: 11
    }

    assert_eq!(Some(11),mutable_value);
    println!("Mutable value after: {:?}", mutable_value); // Prints: Mutable value: Some(11)
}

#[test]
fn test2_wrong() {
    let immutable_value = Some(5);
    println!();
    println!("immutable value before: {:?}", immutable_value);

    // Reassigning the binding
    //immutable_value = Some(10);//can't

    // Pattern matching with mutable reference to the inner value
    if let Some(mut inner) = immutable_value {
        inner += 1; // Mutating the inner value
        println!("Inner value: {}", inner); // Prints: Inner value: 6
    }

    //XXX: welcome to the (implicit) Copy trait
    assert_eq!(Some(5),immutable_value);
    println!("Immutable value after: {:?}", immutable_value); // Prints: Mutable value: Some(5)
/* "Indeed, that's a significant gotcha in Rust, especially when dealing with types that implement the `Copy` trait.

When you pattern match against an immutable binding (`let binding`) and try to obtain a mutable value (`if let Some(mut inner) = binding`), Rust performs a partial move. It clones the value of the inner contents into the `inner` variable, allowing mutation within the scope of the `if let` block. However, any changes made to `inner` are local to the scope and do not affect the original value stored in the `binding` variable.

This behavior can sometimes lead to unexpected results if not carefully considered, especially when dealing with mutable operations on types that implement `Copy`. It's essential to be mindful of these nuances to avoid unintended behavior in Rust programs." -chatgpt 3.5 */
}


//XXX: non-Copy type prevents misuse
#[derive(Debug, PartialEq)]
struct My(i32);

#[test]
fn test3_preventative() {
    let mutable_value = Some(My(5));
    println!();
    println!("Mutable value before: {:?}", mutable_value);

    // Reassigning the binding
    //mutable_value = Some(10);//can't

    // Pattern matching with mutable reference to the inner value
    if let Some(mut inner) = mutable_value {
        inner.0 += 1; // Mutating the inner value
        println!("Inner value: {:?}", inner); // Prints: Inner value: 6
    }

    //Can't fall into the Copy trap here due to having used non-Copy type!
    //assert_eq!(Some(My(11)),mutable_value);//E0382: borrow of partially moved value: `mutable_value`
    //println!("Mutable value after: {:?}", mutable_value);//can't! E0382: borrow of partially moved value: `mutable_value`
}

#[test]
fn test4_correct() {
    let mut mutable_value = Some(My(5));
    println!();
    println!("Mutable value before: {:?}", mutable_value);

    // Reassigning the binding
    mutable_value = Some(My(10));

    // Pattern matching with mutable reference to the inner value
    if let Some(ref mut inner) = mutable_value {
        inner.0 += 1; // Mutating the inner value
        println!("Inner value: {:?}", inner); // Prints: Inner value: 6
    }

    assert_eq!(Some(My(11)),mutable_value);
    println!("Mutable value after: {:?}", mutable_value);//can't! E0382: borrow of partially moved value: `mutable_value`
}

