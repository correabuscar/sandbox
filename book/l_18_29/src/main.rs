// file:///home/user/build/2nonpkgs/rust.stuff/rust/rust.installed.dir/share/doc/rust/html/book/ch18-03-pattern-syntax.html#-bindings
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => { // Using @ lets us test a value and save it in a variable within one pattern.
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
