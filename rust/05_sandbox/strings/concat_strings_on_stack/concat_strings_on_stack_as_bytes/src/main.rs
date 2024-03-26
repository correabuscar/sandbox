//chatgpt 3.5 generated code, initially.

//TODO: maybe allow a start index arg
//TODO: maybe allow the use of a separator

/// Fills up the buffer up until the last complete grapheme
/// So buffer may or may not be filled fully in the end
/// but you won't have incomplete graphemes aka invalid utf8
fn concatenate_strings<'a>(pieces: &'a [&'static str], buffer: &'a mut [u8]) -> &'a str {
    let mut current_pos = 0;
    let buf_len=buffer.len();

    for piece in pieces {
        let bytes = piece.as_bytes();

        let remaining_space = buf_len - current_pos;
        let bytes_to_copy = usize::min(bytes.len(), remaining_space);

        let old_pos=current_pos;
        current_pos += bytes_to_copy;
        //println!("current_pos={}",current_pos);
        buffer[old_pos..current_pos].copy_from_slice(&bytes[..bytes_to_copy]);

        //Because this is here, below, if buf_len was 0 initially, it does go over the above code once to
        //get here, but this is fine.
        if current_pos >= buf_len {
            break; // Stop concatenating if buffer is full
        }
    }

    match std::str::from_utf8(&buffer[..current_pos]) {
        Ok(valid_str) => valid_str,
        Err(utf8_error) => {
            let valid_up_to = utf8_error.valid_up_to();
            //println!("{}", valid_up_to);
            &std::str::from_utf8(&buffer[..valid_up_to]).expect("Invalid UTF-8")
        }
    }
}


fn main() {
    const BUFFER_SIZE: usize = 20; // Adjust this according to your needs
    let mut buffer = [0u8; BUFFER_SIZE];

    let emojis="❤❥웃유🍾☮✌☏☢☠✔☑♚▲♪฿Ɖ⛏♥❣♂♀⚤Ⓐ✍✉☣☤✘☒♛▼♫⌘⌛¡♡ღツ☼";
    let strings = ["He","llo", " ", "World", "!", emojis];
    //let strings = ["H", "e"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    println!("{}", concatenated);
}

#[cfg(test)]
const EMOJIS_FOR_TESTS : &str="❤❥웃유🍾☮✌☏☢☠✔☑♚▲♪฿Ɖ⛏♥❣♂♀⚤Ⓐ✍✉☣☤✘☒♛▼♫⌘⌛¡♡ღツ☼";

#[test]
fn test_doesnt_include_incomplete_grapheme() {
    const BUFFER_SIZE: usize = 20;
    let mut buffer = [0u8; BUFFER_SIZE];
    let strings = ["He","llo", " ", "World", "!", EMOJIS_FOR_TESTS];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "Hello World!❤❥");
}

#[test]
fn test_zero_byte_buffer() {
    const BUFFER_SIZE: usize = 0;
    let mut buffer = [0u8; BUFFER_SIZE];
    let strings = ["He","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "");

    let strings = [EMOJIS_FOR_TESTS];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "");
}

#[test]
fn test_one_byte_buffer() {
    const BUFFER_SIZE: usize = 1;
    let mut buffer = [0u8; BUFFER_SIZE];
    let strings = ["He","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "H");

    let strings = [ EMOJIS_FOR_TESTS ];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "");
}

#[test]
fn test_two_byte_buffer() {
    const BUFFER_SIZE: usize = 2;
    let mut buffer = [0u8; BUFFER_SIZE];
    let strings = ["He","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "He");

    let strings = ["Hello", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "He");

    let strings = [ EMOJIS_FOR_TESTS ];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "");
}

#[test]
fn test_3_byte_buffer() {
    const BUFFER_SIZE: usize = 3;
    let mut buffer = [0u8; BUFFER_SIZE];
    let strings = ["He","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "Hel");

    let strings = ["H","e","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "Hel");

    let strings = [ EMOJIS_FOR_TESTS ];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, &EMOJIS_FOR_TESTS[0..=2]);
}

#[test]
fn test_empty_strings_input() {
    const BUFFER_SIZE: usize = 3;
    let mut buffer = [0u8; BUFFER_SIZE];
    let strings = ["","H","","e","","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "Hel");

    let strings = ["H","","e","llo", " ", "World", "!"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, "Hel");

    let strings = [ "", EMOJIS_FOR_TESTS ];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    assert_eq!(concatenated, &EMOJIS_FOR_TESTS[0..=2]);
}
