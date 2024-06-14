#[macro_export]
macro_rules! macro_two {
    () => {
        // Call macro_one from mod1 using $crate::
        println!("This is macro two from mod2, calling macro one next:");
        $crate::macro_one!();
    };
}

