use std::str::Utf8Error;

enum StrOrBytes<'a> {
    Str(&'a str),
    Bytes([u8; 5]), // Example: [u8; 5] is a static array of size 5
}

impl<'a> StrOrBytes<'a> {
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        match self {
            StrOrBytes::Str(s) => Ok(*s),
            StrOrBytes::Bytes(b) => std::str::from_utf8(b),
        }
    }
}

fn main() {
    let string_data = "hello";
    let byte_data: [u8; 5] = [104, 101, 108+20, 108, 111]; // corresponds to "hello" in bytes

    let s = StrOrBytes::Str(string_data);
    let b = StrOrBytes::Bytes(byte_data);

    // Example usage of as_str method:
    match s.as_str() {
        Ok(s) => println!("String: {}", s),
        Err(e) => println!("Error converting string: {:?}", e),
    }

    match b.as_str() {
        Ok(s) => println!("Bytes as String: {}", s),
        Err(e) => println!("Error converting bytes to string: {:?}", e),
    }
}

