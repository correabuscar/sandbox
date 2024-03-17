//http://doc.rust-lang.org/book/crates-and-modules.html#importing-modules-with-use
//
pub use self::greetings::hello as hi;
pub use self::farewells::goodbye;
//^ "The pub use declaration brings the function into scope at this part of our module hierarchy. Because we’ve pub used this inside of our japanese module, we now have a phrases::japanese::hello() function and a phrases::japanese::goodbye() function, even though the code for them lives in phrases::japanese::greetings::hello() and phrases::japanese::farewells::goodbye(). Our internal organization doesn’t define our external interface."

pub mod greetings;

mod farewells;

