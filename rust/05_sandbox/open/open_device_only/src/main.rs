use libc::{stat, S_IFBLK, S_IFCHR, S_IFMT};
use std::fs::File;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// Custom error type for errors that can occur in `open_device`.
#[derive(Debug)]
pub enum DeviceOpenError {
    Io(io::Error, PathBuf),
    NotADevice(PathBuf),
}

//impl std::fmt::Display for DeviceOpenError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        match self {
//            DeviceOpenError::Io(err, path) => {
//                write!(f, "Error accessing file '{}': {}", path.display(), err)
//            }
//            DeviceOpenError::NotADevice(path) => {
//                write!(f, "The file '{}' is not a device", path.display())
//            }
//        }
//    }
//}
//
//impl std::error::Error for DeviceOpenError {}
//
//impl From<io::Error> for DeviceOpenError {
//    fn from(err: io::Error) -> Self {
//        DeviceOpenError::Io(err, PathBuf::new())
//    }
//}
impl std::fmt::Display for DeviceOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceOpenError::Io(err, path) => write!(f, "Error accessing file '{}': {}", path.display(), err),
            DeviceOpenError::NotADevice(path) => write!(f, "The file '{}' is not a device", path.display()),
        }
    }
}

impl std::error::Error for DeviceOpenError {}

impl From<DeviceOpenError> for io::Error {
    fn from(err: DeviceOpenError) -> io::Error {
        match err {
            DeviceOpenError::Io(err, path) => io::Error::new(err.kind(), format!("Error accessing file '{}': {}", path.display(), err)),
            DeviceOpenError::NotADevice(path) => {
                io::Error::new(io::ErrorKind::Other, format!("The file '{}' is not a device", path.display()))
            }
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
                Some(c) if (*byte >=0x20) && (*byte <= 0x7E) => {
                    formatted_path.push(c);
                },
                _ => {
                    formatted_path.push_str(&format!("\\x{:02X}", byte));
                }
            }
        }
        return formatted_path;
    }
}

/// Opens a file and checks if it is a device.
/// Returns an error if the file is not a character or block device.
pub fn open_device<P: AsRef<Path>>(path: P) -> io::Result<File> {
    // Convert the path to a C string
    let path = path.as_ref();
    //FIXME: this uses heap! does File::open use heap?
//    let path_cstr = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| {
//        DeviceOpenError::Io(
//            io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"),
//            path.to_path_buf(),
//        )
//    })?;
    let os_str_path=path.as_os_str();
    let path_cstr = std::ffi::CString::new(os_str_path.as_bytes())
        .map_err(|e| {
            let visible_path = humanly_visible_os_chars(os_str_path);
            io::Error::new(io::ErrorKind::InvalidInput,
                format!("Invalid path '{}', error: {}",
                    //path.display()
                    visible_path
                    , e)
                )
        })?;


    // Open the file
    let file = File::open(&path).map_err(|e| DeviceOpenError::Io(e, path.to_path_buf()))?;
//    let file = File::open(&path)?;

    // Use the stat function to get file type
    let mut stat_info: stat = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::stat(path_cstr.as_ptr(), &mut stat_info) };

    if result != 0 {
        return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to stat '{}': {}", path.display(), io::Error::last_os_error())));
//        return Err(io::Error::last_os_error());
//        return Err(DeviceOpenError::Io(
//            io::Error::last_os_error(),
//            path.to_path_buf(),
//        )).map_err(|e| DeviceOpenError::Io(e, path.to_path_buf()));
    }

    // Check if the file is a character or block device
    if (stat_info.st_mode & S_IFMT) == S_IFCHR || (stat_info.st_mode & S_IFMT) == S_IFBLK {
        Ok(file)
    } else {
        //Err(DeviceOpenError::NotADevice(path.to_path_buf()))
        Err(io::Error::new(io::ErrorKind::Other, DeviceOpenError::NotADevice(path.to_path_buf())))
    }
}

fn example() -> io::Result<()> {
    let _file = open_device("/dev/null")?;
    // Use file as needed
    Ok(())
}



fn main() { //-> Result<(), std::io::Error> {
    match open_device("/dev/null") {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }

    match open_device("src/main.rs") {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    match open_device("this is the name of a non exiting file here") {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    match open_device("heart emoji ♥ containing one") {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    match open_device(
        //PathBuf::from("/dev/n\0ull")
        "foo\0null"
        ) {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    match open_device("heart emoji ♥ containing one, and a \0nul!") {
        Ok(file) => println!("Successfully opened device: {:?}", file),
        Err(e) => println!("Failed to open device: {}", e),
    }
    //open_device("this is the name of a non exiting file here")?;
    match example() {
        Ok(_) => println!("Successfully opened device"),
        Err(e) => println!("Failed to open device: {}", e),
    }

    //Ok(())
}

