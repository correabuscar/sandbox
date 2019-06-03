fn main() {
    //let x = Some(5);
    ////let y = 10;
    //let y = 5;

    let x = Some(50);
    let y = 50;

    match x {
        Some(50) => { println!("Got 50") } // when braces comma not needed?!!
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    match x {
        Some(n) => {
            if 50 == n { println!("Got 50")
            } else if n == y {
                println!("Matched, n = {:?}", n)
            }
        } // when braces comma not needed?!!
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

