#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

//dump_stats code from: https://github.com/tikv/tikv/blob/30963e44de6854b036db9fab4b5629825b4a1bbe/components/tikv_alloc/src/jemalloc.rs
//use std::ffi::c_void;
use jemalloc_sys::malloc_stats_print;
use std::{ptr, slice};
use libc::{self, c_char, c_void};


#[allow(clippy::cast_ptr_alignment)]
extern "C" fn write_cb(printer: *mut c_void, msg: *const c_char) {
    unsafe {
        // This cast from *c_void to *Vec<u8> looks like a bad
        // cast to clippy due to pointer alignment, but we know
        // what type the pointer is.
        let buf = &mut *(printer as *mut Vec<u8>);
        let len = libc::strlen(msg);
        let bytes = slice::from_raw_parts(msg as *const u8, len);
        buf.extend_from_slice(bytes);
    }
}

pub fn dump_stats() -> String {
    let mut buf = Vec::with_capacity(1024);
    unsafe {
        malloc_stats_print(
            Some(write_cb),
            &mut buf as *mut Vec<u8> as *mut c_void,
            ptr::null(),
        )
    }
    String::from_utf8_lossy(&buf).into_owned()
}

fn main() {
    //println!("Hello, world!");
    let n=10000000;

    for i in 1..n {
        let tmp=format!("Something {} or another", i);
        println!("{}", tmp.as_str());
    }
    eprintln!("{}", dump_stats());
}
