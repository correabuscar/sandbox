use std::io::Write; //XXX: needed for flush() to be seen in scope!
//macro_rules! fflush { () => ({ std::io::stdout().flush().ok().expect(stringify!(Could not flush $name)); }) }
macro_rules! moosh { ($x:expr) => ({ $x.expect(stringify!(Could not flush $x)); }) }
//FIXME: how to place $name into str


fn main() {
    let x:Result<u8,u8> =Result::Err(102);
    moosh!(std::io::stdout().flush().ok());
    moosh!(x);//good: thread 'main' panicked at 'Could not flush x: 102', src/libcore/result.rs:860:4

}
