// Re-export macros to control their visibility and ensure consistent import paths
// doesn't work, chatgpt-4o ! ;p
//pub use mod1::macro_one;
//pub use mod2::macro_two;

mod mod1;
mod mod2;
//mod mod3;//XXX: won't let me redefine it which is good!

// Local macro with the same name (for testing conflicts)
macro_rules! macro_one {
    () => {
        println!("This is a local macro named macro_one.");
    };
}

fn main() {
    // Using the re-exported macros
    crate::macro_one!(); // Calls the macro_one from mod1

    // Using the local macro
    macro_one!(); // Calls the local macro_one

    macro_two!(); // Calls macro_two from mod2, which calls macro_one from mod1

}

