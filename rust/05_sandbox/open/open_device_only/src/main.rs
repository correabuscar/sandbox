use libc::{stat, /*S_IFBLK,*/ S_IFCHR, S_IFMT};
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
                write!(f, "Error accessing file '{}': {}", path.display(), err)
            }
            DeviceOpenError::NotADevice(path) => {
                write!(f, "The file '{}' is not a device", path.display())
            }
        }
    }
}

impl std::error::Error for DeviceOpenError {}

impl From<DeviceOpenError> for io::Error {
    fn from(err: DeviceOpenError) -> io::Error {
        match err {
            DeviceOpenError::Io(err, path) => io::Error::new(
                err.kind(),
                format!("Error accessing file '{}': {}", path.display(), err),
            ),
            DeviceOpenError::NotADevice(path) => io::Error::new(
                io::ErrorKind::Other,
                format!("The file '{}' is not a device", path.display()),
            ),
        }
    }
}

fn humanly_visible_os_chars(os_path: &std::ffi::OsStr) -> String {
    if let Some(arg_str) = os_path.to_str() {
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
        for byte in os_path.as_bytes() {
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
pub fn open_char_device<P: AsRef<Path>>(path: P) -> io::Result<File> {
    // Convert the path to a C string
    let path = path.as_ref();
    //FIXME: this uses heap! does File::open use heap?
    let os_str_path = path.as_os_str();
    let path_cstr = std::ffi::CString::new(os_str_path.as_bytes()).map_err(|e| {
        let visible_path = humanly_visible_os_chars(os_str_path);
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Invalid path '{}', error: {}",
                //path.display()
                visible_path,
                e
            ),
        )
    })?;

    // Open the file
    let file = File::open(os_str_path).map_err(|e| DeviceOpenError::Io(e, path.to_path_buf()))?;

    // Use the stat function to get file type
    let mut stat_info: stat = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::stat(path_cstr.as_ptr(), &mut stat_info) };

    if result != 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to stat '{}': {}",
                path.display(),
                io::Error::last_os_error()
            ),
        ));
    }

    // Check if the file is a character device:
    // "/dev/null is indeed classified as a character device when inspected with the stat command."
    // "Device type: 1,3 further confirms that it's a character device. The first number (1) indicates the major device number, which represents the device type (character device), and the second number (3) is the minor device number."
    // - chatgpt 3.5
    if (stat_info.st_mode & S_IFMT) == S_IFCHR
    /* || (stat_info.st_mode & S_IFMT) == S_IFBLK*/
    {
        Ok(file)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            DeviceOpenError::NotADevice(path.to_path_buf()),
        ))
    }
}

fn example() -> io::Result<()> {
    let _file = open_char_device("/dev/null")?;
    // Use file as needed
    Ok(())
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

    let res = open_char_device("this is the name of a non exiting file here");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);

    let res = open_char_device("heart emoji ♥ containing one");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);

    let res = open_char_device("foo\0null");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);

    let res = open_char_device(PathBuf::from("/dev/n\0ull"));
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);

    let res = open_char_device("heart emoji ♥ containing one, and a \0nul!");
    match &res {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_err(), "{:?}", res);

    let res = example();
    match &res {
        Ok(_) => println!("Successfully opened device"),
        Err(e) => println!("Failed to open device: {}", e),
    }
    assert!(res.is_ok(), "{:?}", res);
}

fn main() {
    any_test_open_char_device();
}
