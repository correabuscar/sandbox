use std::os::unix::io::AsRawFd;

fn main() {
    let stderr_fd = std::io::stderr().as_raw_fd();
    println!("Standard Error File Descriptor: {}", stderr_fd);
}

