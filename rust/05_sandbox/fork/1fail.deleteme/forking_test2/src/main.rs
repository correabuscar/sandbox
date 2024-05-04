#![feature(rt)]

//XXX: it's private!
//use std::rt::init;

//fn main() {
//    println!("Hello, world!");
//}
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_init() {
        // Mock command-line arguments
        let argc = 1;
        let arg0 = CString::new("test_program").expect("CString::new failed");
        let argv: [*const u8; 2] = [
            arg0.as_ptr() as *const u8,
            std::ptr::null(), // Null-terminated array
        ];

        // Call init
        unsafe {
            //XXX: it's private!
            std::rt::init(argc, argv.as_ptr(), 0); // Assuming sigpipe is 0 for the test
        }

        // Add assertions if necessary to check the behavior of init
        assert!(true); // Placeholder assertion
    }
}

