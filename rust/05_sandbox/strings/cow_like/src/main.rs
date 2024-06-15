use std::borrow::Cow;

enum StrOrBytes<'a> {
    Str(Cow<'a, str>),
    Bytes(Cow<'a, [u8]>),
}

impl<'a> StrOrBytes<'a> {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            StrOrBytes::Str(cow_str) => Some(cow_str.as_ref()),
            StrOrBytes::Bytes(cow_bytes) => std::str::from_utf8(cow_bytes.as_ref()).ok(),
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            StrOrBytes::Str(cow_str) => Some(cow_str.as_bytes()),
            StrOrBytes::Bytes(cow_bytes) => Some(cow_bytes.as_ref()),
        }
    }
}

fn main() {
    let string_data = "hello".to_string();
    let byte_data: Vec<u8> = vec![104, 101, 108, 108, 111]; // corresponds to "hello" in bytes

    let s = StrOrBytes::Str(Cow::Owned(string_data));
    let b = StrOrBytes::Bytes(Cow::Owned(byte_data));

    if let Some(s) = s.as_str() {
        println!("String: {}", s);
    }
    if let Some(s) = s.as_bytes() {
        println!("bytes: {:?}", s);
    }

    if let Some(b) = b.as_bytes() {
        println!("Bytes: {:?}", b);
    }
    if let Some(b) = b.as_str() {
        println!("str: {}", b);
    }

    let string_data = "hello".to_string();
    let valid_byte_data: Vec<u8> = vec![104, 101, 108, 108, 111]; // corresponds to "hello" in bytes
    let invalid_byte_data: Vec<u8> = vec![0, 159, 146, 150]; // invalid UTF-8 sequence

    let s = StrOrBytes::Str(Cow::Owned(string_data));
    let valid_b = StrOrBytes::Bytes(Cow::Owned(valid_byte_data));
    let invalid_b = StrOrBytes::Bytes(Cow::Owned(invalid_byte_data));

    if let Some(s) = s.as_str() {
        println!("String: {}", s);
    }

    if let Some(b) = valid_b.as_str() {
        println!("Valid Bytes as String: {}", b);
    } else {
        println!("Valid Bytes are not valid UTF-8");
    }

    if let Some(b) = invalid_b.as_str() {
        println!("Invalid Bytes as String: {}", b);
    } else {
        println!("Invalid Bytes are not valid UTF-8");
    }

    if let Some(b) = valid_b.as_bytes() {
        println!("Valid Bytes: {:?}", b);
    }

    if let Some(b) = invalid_b.as_bytes() {
        println!("Invalid Bytes: {:?}", b);
    }
}

