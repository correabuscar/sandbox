//written by chatgpt-4o that's why if error, u don't see the filename in the error! but also because it's taught this way in the rust book chapter 9-2!
//use std::process::Command;

//fn cause_segfault() {
//    unsafe {
//        // Deliberately cause a segmentation fault
//        let p: *mut i32 = std::ptr::null_mut();
//        *p = 42; // Dereference null pointer, causing a segfault
//    }
//}

fn main() {
    // Write the cause_segfault function to a temporary executable
    let source_code = r#"
        fn main() {
            unsafe {
                let p: *mut i32 = std::ptr::null_mut();
                *p = 42;
            }
        }
    "#;

    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let mut file = File::create("/tmp/segfault.rs").expect("Unable to create file");
    file.write_all(source_code.as_bytes()).expect("Unable to write data");

    // Compile the Rust code
    Command::new("rustc")
        .arg("/tmp/segfault.rs")
        .args(["--out-dir", "/tmp"])
        .output()
        .expect("Failed to compile");

    // Execute the compiled binary
    let output = Command::new("/tmp/segfault")
        .output()
        .expect("Failed to execute command");

    match output.status.code() {
        Some(code) => println!("Exit status: {}", code),
        None => println!("Process terminated by signal"),
    }
}

