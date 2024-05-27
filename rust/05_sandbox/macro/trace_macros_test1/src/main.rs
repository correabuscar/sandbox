//src: https://veykril.github.io/tlborm/decl-macros/minutiae/debugging.html
#![feature(trace_macros)]

macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest:tt)*) => {each_tt!($($rest)*);};
}


fn main() {
    println!("Hello, world!");
    each_tt!(foo bar baz quux);
    trace_macros!(true);
    each_tt!(spim wak plee whum); // if you see this red in vim ignore it, it's rust-analyzer
    trace_macros!(false);
    each_tt!(trom qlip winp xod);
}
