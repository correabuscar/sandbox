//pub const X: i64 = 2+ std::i64::MAX;
fn main() {
    const X: u32 = 33-44;//using src: https://github.com/rust-lang/rust/issues/43197#issuecomment-315016896
    println!("{}", X);//bug
}
