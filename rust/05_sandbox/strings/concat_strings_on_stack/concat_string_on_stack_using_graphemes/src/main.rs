//chatgpt 3.5 generated code
use unicode_segmentation::UnicodeSegmentation;

fn concatenate_strings<'a>(pieces: &'a [&'static str], buffer: &'a mut [u8]) -> &'a str {
    let mut current_pos = 0;
    for piece in pieces {
        for grapheme in piece.graphemes(true) {
            let grapheme_bytes = grapheme.as_bytes();
            if current_pos + grapheme_bytes.len() >= buffer.len() {
                break;
            }
            buffer[current_pos..current_pos + grapheme_bytes.len()].copy_from_slice(grapheme_bytes);
            current_pos += grapheme_bytes.len();
        }
    }
    std::str::from_utf8(&buffer[..current_pos]).expect("Invalid UTF-8")
}

fn main() {
    const BUFFER_SIZE: usize = 21;
    let mut buffer = [0u8; BUFFER_SIZE];

    let strings = ["Hello", " ", "World", "!", "❤❥웃유🍾☮✌☏☢☠✔☑♚▲♪฿Ɖ⛏
♥	❣	♂	♀	⚤	Ⓐ	✍	✉	☣	☤	✘	☒	♛	▼	♫	⌘	⌛	¡
♡	ღ	ツ	☼"];
    let concatenated = concatenate_strings(&strings, &mut buffer);
    println!("{}", concatenated);
}

