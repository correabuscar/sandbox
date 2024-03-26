//chatgpt 3.5 generated code
fn concatenate_strings<'a>(pieces: &'a [&'static str], buffer: &'a mut [u8]) -> &'a str {
    let mut current_pos = 0;

    for piece in pieces {
        let bytes = piece.as_bytes();

        if current_pos >= buffer.len() {
            break; // Stop concatenating if buffer is full
        }

        let remaining_space = buffer.len() - current_pos;
        let bytes_to_copy = usize::min(bytes.len(), remaining_space);

        buffer[current_pos..current_pos + bytes_to_copy].copy_from_slice(&bytes[..bytes_to_copy]);
        current_pos += bytes_to_copy;
    }

    match std::str::from_utf8(&buffer[..current_pos]) {
        Ok(valid_str) => valid_str,
        Err(utf8_error) => {
            let valid_up_to = utf8_error.valid_up_to();
            println!("{}", valid_up_to);
            &std::str::from_utf8(&buffer[..valid_up_to]).expect("Invalid UTF-8")
        }
    }
}

fn main() {
    const BUFFER_SIZE: usize = 20; // Adjust this according to your needs
    let mut buffer = [0u8; BUFFER_SIZE];

    let emojis="❤❥웃유🍾☮✌☏☢☠✔☑♚▲♪฿Ɖ⛏♥❣♂♀⚤Ⓐ✍✉☣☤✘☒♛▼♫⌘⌛¡♡ღツ☼";
    let strings = ["Hello", " ", "World", "!", emojis];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    println!("{}", concatenated);
}

