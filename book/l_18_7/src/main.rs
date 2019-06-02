fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn pc((x, y): (i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

//fn pc2(x, y: i32, i32) {
//    println!("Current location: ({}, {})", x, y);
//}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
    pc(point);
    //pc(3,6);
    pc((3,7));
}
