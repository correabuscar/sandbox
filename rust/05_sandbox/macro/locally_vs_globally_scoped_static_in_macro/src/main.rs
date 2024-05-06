macro_rules! increment_static {
    (local) => {
        {
            static mut COUNTER: i32 = 0;
            /* "When you define a static variable within a macro, Rust ensures that each invocation
             * of the macro generates a unique static variable by mangling the name of the static.
             * This mangling process ensures that each static variable has a distinct name,
             * allowing multiple invocations of the macro to coexist without conflicts." - chatgpt 3.5
             */
            unsafe {
                COUNTER += 1;
                println!("Static counter: {}", COUNTER);
            }
        }
    };
    (global) => {
        {
            /*can't call this macro more than once due to the no_mangle*/
            #[no_mangle]
            static mut COUNTER: i32 = 0;
            unsafe {
                COUNTER += 1;
                println!("Static counter: {}", COUNTER);
            }
        }
    };
}

fn main() {
    // First invocation
    increment_static!(local);
    increment_static!(global);
    increment_static!(local);
    //increment_static!(global);//XXX: can't, redefinition attempt!
}

