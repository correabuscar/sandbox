use codepage_437::{FromCp437, Cp437Dialect};

// Generic function that converts a CP437-encoded byte array to a String
fn convert_cp437_to_string<T: AsRef<[u8]>>(data: T, dialect: &Cp437Dialect) -> String {
    // Convert the input byte slice to a string using the FromCp437 trait
    return String::from_cp437(data.as_ref(), dialect); //fail
}

fn main() {
    // Example CP437-encoded bytes
    let data1 = [0x14, 0x15, 0x16]; // Array of length 3
    let data2 = [0x14, 0x15, 0x16, 0x17, 0x18]; // Array of length 5

    // Convert CP437-encoded bytes to UTF-8 string using Wingdings encoding
    let in_unicode1 = convert_cp437_to_string(&data1, &Cp437Dialect::Wingdings);
    let in_unicode2 = convert_cp437_to_string(&data2, &Cp437Dialect::Wingdings);

    println!("UTF-8 string (data1): {}", in_unicode1);
    println!("UTF-8 string (data2): {}", in_unicode2);
}

