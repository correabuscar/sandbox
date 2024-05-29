use std::fs::File;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// Custom error type for errors that can occur in `open_char_device`.
#[derive(Debug)]
pub enum DeviceOpenError {
    Io(io::Error, PathBuf),
    NotADevice(PathBuf),
}

impl std::fmt::Display for DeviceOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceOpenError::Io(err, path) => {
                write!(
                    f,
                    "Error accessing character device '{}': {}",
                    humanly_visible_os_chars(path),
                    err
                )
            }
            DeviceOpenError::NotADevice(path) => {
                write!(
                    f,
                    "The file '{}' is not a character device",
                    //path.display(),
                    humanly_visible_os_chars(path),
                )
            }
        }
    }
}

impl std::error::Error for DeviceOpenError {}

impl From<DeviceOpenError> for io::Error {
    fn from(err: DeviceOpenError) -> io::Error {
        //doneTODO: this code repeats twice here and above in Display
        //println!("FROM:{}",err.to_string());
        match &err {
            DeviceOpenError::Io(inner_err, _path) => {
                io::Error::new(inner_err.kind(), err.to_string())
            }
            DeviceOpenError::NotADevice(_path) => {
                io::Error::new(io::ErrorKind::NotFound, err.to_string())
            }
        }
    }
}

fn humanly_visible_os_chars<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref().as_os_str();
    if let Some(arg_str) = path.to_str() {
        // If the argument is valid UTF-8,
        if arg_str.contains('\0') {
            // show the nuls as \x00, keep the rest like ♥ as they are
            let mut formatted_path = String::new();
            for c in arg_str.chars() {
                if c == '\0' {
                    formatted_path.push_str("\\x00");
                } else {
                    formatted_path.push(c);
                }
            }
            return formatted_path;
        } else {
            // there are no nuls so keep it as it is, like with ♥ instead of \xE2\x99\xA5
            return format!("{}", arg_str);
        }
    } else {
        let mut formatted_path = String::new();
        //not fully utf8
        //then we show it as ascii + hex
        for byte in path.as_bytes() {
            match std::char::from_u32(u32::from(*byte)) {
                Some(c) if (*byte >= 0x20) && (*byte <= 0x7E) => {
                    formatted_path.push(c);
                }
                _ => {
                    formatted_path.push_str(&format!("\\x{:02X}", byte));
                }
            }
        }
        return formatted_path;
    }
}

// XXX: this can replace a cargo File::open call(presumably), but for rustc variants we need something else!
// cargo: https://github.com/rust-lang/cargo/blob/cbc12a2ebe0a99836249de0f80f025192e58cb4b/credential/cargo-credential/src/stdio.rs#L11
// rustc: https://github.com/rust-lang/rust/blob/4cf5723dbe471ef0a32857b968b91498551f5e38/library/std/src/sys/pal/unix/process/process_common.rs#L479-L486
// .. https://github.com/rust-lang/rust/blob/a83f933a9da258cf037e3cab37cd486bfd861a7d/library/std/src/sys/pal/unix/fs.rs#L1160-L1171
// and for my code I might want things like this: [1] [2]
// [1] https://github.com/rust-lang/rust/blob/a83f933a9da258cf037e3cab37cd486bfd861a7d/library/std/src/sys/pal/unix/fs.rs#L1157
// [2] https://github.com/rust-lang/rust/blob/a83f933a9da258cf037e3cab37cd486bfd861a7d/library/std/src/sys/pal/common/small_c_string.rs#L36
/// Opens a file and checks if it is a char device.
/// Returns an error if the file is not a character device.
/// meant to be a File::open() replacement for cases when /dev/null is used in cargo/rustc code!
pub fn open_char_device<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let path = path.as_ref();
    let metadata =
        std::fs::metadata(path).map_err(|e| DeviceOpenError::Io(e, path.to_path_buf()))?;

    // Check if the file is a character device:
    // "/dev/null is indeed classified as a character device when inspected with the stat command."
    // "Device type: 1,3 further confirms that it's a character device. The first number (1) indicates the major device number, which represents the device type (character device), and the second number (3) is the minor device number."
    // - chatgpt 3.5
    use std::os::unix::fs::FileTypeExt;
    if metadata.file_type().is_char_device() {
        // Open the file as usual
        File::open(path).map_err(|e| DeviceOpenError::Io(e, path.to_path_buf()).into())
    } else {
        // Return an error indicating that the file is not a character device
        Err(DeviceOpenError::NotADevice(path.to_path_buf()).into())
    }
}

fn example() -> io::Result<File> {
    let file = open_char_device("/dev/null")?;
    // Use file as needed
    Ok(file)
}
fn example_err() -> io::Result<File> {
    let file = open_char_device("./src/main.rs")?;
    // Use file as needed
    Ok(file)
}

#[test]
fn test_open_char_device() {
    any_test_open_char_device();
}
fn any_test_open_char_device() {
    //TODO: ensure the contents of it match
    let res = open_char_device("/dev/null");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(
        res.is_ok(),
        "you're in chroot? or don't have /dev/null ? '{:?}'",
        res
    );

    let res = open_char_device("src/main.rs");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::NotFound);

    let res = open_char_device("this is the name of a non exiting file here");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::NotFound);

    let res = open_char_device("heart emoji ♥ containing one");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::NotFound);

    let res = open_char_device("foo\0null");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::InvalidInput);

    let res = open_char_device(PathBuf::from("/dev/n\0ull"));
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::InvalidInput);

    let res = open_char_device("heart emoji ♥ containing one, and a \0nul!");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::InvalidInput);

    let res = example();
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_ok(), "{:?}", res);

    let res = example_err();
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);
    assert_eq!(res.err().unwrap().kind(), io::ErrorKind::NotFound);
}

fn main() {
    any_test_open_char_device();
}
