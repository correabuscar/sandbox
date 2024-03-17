fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            // The syntax .. will expand to as many values as it needs to be.
            println!("Some numbers: {}, {}", first, last);
        }
    }

    //warning: irrefutable if-let pattern
    #[allow(irrefutable_let_patterns)]
    {
        // attributes are not yet allowed on `if` expressions
        if let (first, .., last) = numbers {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    {
        let (first, .., last) = numbers;
        println!("Some numbers: {}, {}", first, last);
    }

    /*    //let num2=(1);
    let num2=(1,);
    //let (first, .., last) = num2; //fail: E0308: mismatched types  expected a tuple with 1 element, found one with 2 elements  note: expected type `({integer},)`
    //println!("Some numbers: {}, {}", first, last);
    match num2 {
        (first, .., last) => { // still fail: E0308: mismatched types  expected a tuple with 1 element, found one with 2 elements  note: expected type `({integer},)`
            println!("Some numbers: {}, {}", first, last);
        },
        _ => panic!(),
    } */
    let num3 = (1, 2);
    let (first, .., last) = num3;
    println!("Some numbers: {}, {}", first, last);
}
