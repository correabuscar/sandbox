use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    // Create a byte string literal
    //let bytes = b"not_a_f\0il\xffe"; //can't have nul
    let bytes = b"not_a_fil\xffe_but_a_dir";

    // Convert the byte string to an OsStr
    let os_str = OsStr::from_bytes(bytes);

    // Create a PathBuf from the OsStr
    let path_buf = PathBuf::from(os_str);
    //println!("{:?} vs {}", path_buf, path_buf.display());

    // Create the directory and file
    if let Err(err) = fs::create_dir_all(&path_buf) {
        panic!("Error creating directory: {}", err);
    }

    // src: https://github.com/antifuchs/chars
    //$ chars �
    //U+FFFD, &#65533; 0xFFFD, \0177775, UTF-8: ef bf bd, UTF-16BE: fffd
    //Width: 1 (2 in CJK context), prints as �
    //Quotes as \u{fffd}
    //Unicode name: REPLACEMENT CHARACTER

    let replacement_char = '\u{FFFD}';
    println!("Replacement character: {}", replacement_char);

    // Now, the directory should exist with the correct path
    println!("Directory created successfully: {:?} vs {}", path_buf, path_buf.display());
    assert_eq!(path_buf.display().to_string(), format!("not_a_fil{}e_but_a_dir", replacement_char));
    //assert_eq!(replacement_char, '\0o377'); //nvmFIXME: how?! can't use this "character literal may only contain one codepoint"
    assert_ne!(replacement_char, std::char::from_u32(0o377).unwrap()); // ok my bad, this is how!
    // in `ls`/bash shows like $'\377' aka '\u{FF}'
    //so this isn't it!
    //LATIN1 ff, 255, 0xff, 0377, bits 11111111
    //Width: 1, prints as ÿ
    //Lower case. Upcases to Ÿ
    //Quotes as \u{ff}
    //Unicode name: LATIN SMALL LETTER Y WITH DIAERESIS
}

