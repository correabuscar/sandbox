#![feature(rustc_private)]

//use std::os::raw::c_void;
//use std::ffi::CString;
//use std::ptr;
//
//// Define a custom abort handler
//extern "C" fn custom_abort(_msg: *const c_void) {
//    // Add custom behavior here (e.g., logging)
//    let msg = "Abort intercepted!";
//    let msg_cstr = CString::new(msg).expect("CString::new failed");
//    unsafe {
//        libc::puts(msg_cstr.as_ptr());
//    }
//
//    // Call the real abort function
//    //libc::abort();
//}

//// Override the abort function with the custom handler
//#[no_mangle]
//pub extern "C" fn abort() {
//    println!("Abort intercepted!");
//    //custom_abort(ptr::null());
//}

//use std::os::raw::c_void;
//use std::ffi::CString;

//// Define a custom abort handler
//#[no_mangle]
//pub extern "C" fn abort() {
//    // Add custom behavior here (e.g., logging)
//    let msg = "Abort intercepted!";
//    let msg_cstr = CString::new(msg).expect("CString::new failed");
//    unsafe {
//        libc::puts(msg_cstr.as_ptr());
//    }
//
//    // Terminate the process
//    //std::process::exit(1);
//}

//// Redefine the abort function as an external symbol
//#[link_section = ".text.startup"]
//#[no_mangle]
//pub extern "C" fn abort() {
//    let msg = "Abort intercepted!";
//    let msg_cstr = CString::new(msg).expect("CString::new failed");
//    unsafe {
//        libc::puts(msg_cstr.as_ptr());
//        //libc::abort();
//    }
//}

//use std::ffi::CString;

//extern crate libc;

// Define the custom abort function
#[no_mangle]
pub extern "C" fn abort() {
    eprintln!("Abort intercepted!");
//    let message = "Abort intercepted!";
//    let message_cstr = CString::new(message).expect("CString::new failed");
//    unsafe {
//        libc::puts(message_cstr.as_ptr());
//        //libc::abort();
//    }
    //panic!("panicking due to abort intercepted"); // this will infinite recuse, without a parent
                                                 // catch_unwind()! and even then FIXME
    // allowing this to fallthrough is bad, illegal instruction (if lucky)
    std::process::abort();//infinite recursion, FIXME: call the original one?
    //unsafe {
    //    libc::abort();
    //}
}

