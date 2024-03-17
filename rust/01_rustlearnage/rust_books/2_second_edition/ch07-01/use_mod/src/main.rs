//src: file://${HOME}/build/2nonpkgs/rust.stuff/book/second-edition/book/ch07-03-importing-names-with-use.html#bringing-names-into-scope-with-the-use-keyword

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();

    let _red = Red; //see below (PgDn)
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
    //We’re still specifying the TrafficLight namespace for the Green variant because we didn’t include Green in the use statement.
}
//The use keyword brings only what we’ve specified into scope: it does not bring children of modules into scope. That’s why we still have to use of::nested_modules when we want to call the nested_modules function.

//use a::series::of::nested_modules;
//
//fn main() {
//    nested_modules();
//}

//Because enums also form a sort of namespace like modules, we can bring an enum’s variants into scope with use as well. For any kind of use statement, if you’re bringing multiple items from one namespace into scope, you can list them using curly brackets and commas in the last position, like so:

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

//use TrafficLight::*;

//The * will bring into scope all the visible items in the TrafficLight namespace. You should use globs sparingly: they are convenient, but this might also pull in more items than you expected and cause naming conflicts.

