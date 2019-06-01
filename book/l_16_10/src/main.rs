//src: file:///home/user/build/2nonpkgs/rust.stuff/rust/rust.installed.dir/share/doc/rust/html/book/ch16-02-message-passing.html#sending-multiple-values-and-seeing-the-receiver-waiting
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
