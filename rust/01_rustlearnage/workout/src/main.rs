extern crate cursive;

//use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
	println!("Hello, world!");
    // src: https://github.com/gyscos/Cursive
	// Creates the cursive root - required for every application.
	//let mut siv = Cursive::new();
	let mut siv = cursive::default();

	// Creates a dialog with a single "Quit" button
	siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
				  .title("Cursive")
				  .button("Quit", |s| s.quit()));

	// Starts the event loop.
	siv.run();
	//println!("{}",c);
}
