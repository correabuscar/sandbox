use std::fs::{self};
use unicode_segmentation::UnicodeSegmentation;
//use std::fs::File;
//use std::io::{self, BufRead};
//use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("/etc/hosts")?;
//    let mut f = OpenOptions::new().read(true).write(false).open("/etc/hosts")?;
//    for line in f.lines() {
    let mut max_line_len=0;
    let mut current_line_number=0;
    let mut line_num_of_maxlinelen=0;
    let mut len_in_chars=0;
    for line in contents.lines() {
        current_line_number+=1;
        //println!("{}",line);
        let ll=line.len();
        if ll > max_line_len {
            max_line_len=ll;
            line_num_of_maxlinelen=current_line_number;
            len_in_chars=line.chars().count();
        }
    }
    println!("Line number {} is {} bytes(not chars and not graphemes) and {} chars(not bytes and not graphemes)", line_num_of_maxlinelen, max_line_len, len_in_chars);
    println!("Please ensure lines are not longer than 990 bytes, or else Firefox(67.0a1 2019-03-08 64bit changeset: 517224:456bdb1cc727 tip) ignores everything after it!");
    //let s="💑";
    //println!("'{}' len={} chars.count={} graphemes.count={:?} {:?}", s, s.len(), s.chars().count(), UnicodeSegmentation::grapheme_indices(s, true).collect::<Vec<(usize, &str)>>(), s.unicode_words().collect::<Vec<&str>>());
    // chars detects this differently(ie. width 2 not 1 !):
    // $ chars 💑
    //U+0001F491, &#128145; 0x0001F491, \0372221, UTF-8: f0 9f 92 91, UTF-16BE: d83ddc91
    //Width: 2, prints as 💑
    //Quotes as \u{1f491}
    //Unicode name: COUPLE WITH HEART

    Ok(())
}
