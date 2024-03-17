fn main() {
    let x = 4;
    let y = false;
    //let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
