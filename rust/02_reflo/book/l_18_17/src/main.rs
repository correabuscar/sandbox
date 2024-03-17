fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn foo2(_x: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
    foo2(3, 4);
}
