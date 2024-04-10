static FORMAT_STRING: &[u8; 3] = b"%s\0";
static mut FORMAT_STRING_PTR: *const u8 = FORMAT_STRING.as_ptr();//no errors
static FORMAT_STRING_PTR2: *const u8 = FORMAT_STRING.as_ptr();// error, can't be shared between threads
                                                              //
//see: https://users.rust-lang.org/t/why-static-mut-can-be-shared-between-threads-safely-but-without-mut-it-cant/109688

fn main() {
    println!("Hello, world!");
}
