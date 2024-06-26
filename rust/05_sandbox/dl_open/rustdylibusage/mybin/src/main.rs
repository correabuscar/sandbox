//extern crate mylib;
//fn main() {
//    mylib::greet("World");
//}

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use libc; // https://github.com/rust-lang/rust/issues/121915

//this code was generated by a large language model (it's how it wanted to be credited, even though it said it was optional :) )

fn main() {
    let lib_name = CString::new("libmylib.so").expect("CString::new failed");
    let lib_handle = unsafe { libc::dlopen(lib_name.as_ptr(), libc::RTLD_NOW) };

    if lib_handle.is_null() {
        let error_msg = unsafe { CStr::from_ptr(libc::dlerror()) }.to_str().unwrap();
        println!("Error loading library: {}", error_msg);
        return;
    }

	//type GreetFn = unsafe extern "C" fn(*const c_char);
	type GreetFn = unsafe extern "C" fn(&str);//*const c_char);
	let greet_fn: GreetFn = unsafe {
        let symbol_name = CString::new("greet").expect("CString::new failed");
        let symbol = libc::dlsym(lib_handle, symbol_name.as_ptr());
        if symbol.is_null() {
            panic!("Failed to find symbol");
        }
        std::mem::transmute(symbol)
    };

    let name = "World";//CString::new("World").expect("CString::new failed");
    unsafe {
    greet_fn(name);
//        greet_fn(name.as_ptr());
    }

//    // Access functions from the library
//    // ...
//    // Get the symbol address
//    let symbol_name = CString::new("greet").unwrap();
//    let symbol_ptr = unsafe { libc::dlsym(lib_handle, symbol_name.as_ptr()) };
//    if symbol_ptr.is_null() {
//        panic!("Failed to get symbol address: {:?}", unsafe { libc::dlerror() });
//    }
//
//    // Convert the symbol pointer to a function pointer
//    type GreetFunctionType = extern "C" fn(*const c_char);
//    let greet_function: GreetFunctionType = unsafe { std::mem::transmute(symbol_ptr) };
//
//    // Convert the name to a C-style string
//    let name = CString::new("World").expect("CString::new failed");
//
//    // Call the function
//    greet_function(name.as_ptr());
//        // Get the symbol address
//    let symbol_name = CString::new("greet").unwrap();
//    let symbol_ptr = unsafe { libc::dlsym(lib_handle, symbol_name.as_ptr()) };
//    if symbol_ptr.is_null() {
//        panic!("Failed to get symbol address: {:?}", unsafe { libc::dlerror() });
//    }
//
//    // Convert the symbol pointer to a function pointer
//    type GreetFunctionType = extern "C" fn(*const c_char);
//    let greet_function: GreetFunctionType = unsafe { mem::transmute(symbol_ptr) };
//
//    // Call the function
//    let name = CString::new("World").expect("CString::new failed");
//    greet_function(name.as_ptr());

    unsafe { libc::dlclose(lib_handle) };
}
