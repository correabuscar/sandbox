use codepage_437::{FromCp437, CP437_WINGDINGS};

fn main() {
    // Example CP437-encoded bytes
    let data = &[0x14, 0x15, 0x16]; // Example bytes representing Wingdings symbols

    // Convert CP437 bytes to UTF-8 string using Wingdings encoding
    let in_unicode = String::from_cp437(data.to_vec(), &CP437_WINGDINGS);

    println!("UTF-8 string: {}", in_unicode);
}

