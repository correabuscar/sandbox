macro_rules! increment_static {
    (local) => {
        {
            static mut COUNTER: i32 = 0;
            unsafe {
                COUNTER += 1;
                println!("Static counter: {}", COUNTER);
            }
        }
    };
    (global) => {
        {
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

