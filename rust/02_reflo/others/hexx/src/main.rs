//src: file:///home/user/build/2nonpkgs/rust.stuff/rust/rust.installed.dir/share/doc/rust/html/std/process/struct.Command.html
#![allow(unused)]
fn main() {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    println!("{:02X?}", hello)
}
