//pub const X: i64 = 2+ std::i64::MAX;
fn main() {
//    const X: u32 = 33-44;//using src: https://github.com/rust-lang/rust/issues/43197#issuecomment-315016896
    //about to get fixed by PR https://github.com/rust-lang/rust/pull/43568/files
    //the ICE is gone! commenting out the code so travis doesn't fail
//    println!("{}", X);//bug
}
