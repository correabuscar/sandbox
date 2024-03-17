//XXX: this is obsolete, please use lib macraa
#![feature(trace_macros)]
#![feature(log_syntax)]
//use std::io::Write; //XXX: needed for flush() to be seen in scope!
//macro_rules! fflush { () => ({ std::io::stdout().flush().ok().expect(stringify!(Could not flush $name)); }) }
macro_rules! moosh { ($x:expr) => ({
    $x.expect(stringify!(Could not flush $x));
}) }

macro_rules! fflush {
    () => ({
        fflush!(stdout);
    });
    ($stdwhat:ident) => ({
		use std::io::Write; //XXX: needed for flush() to be seen in scope!
		std::io::$stdwhat().flush().ok().expect(stringify!(Could not moosh $stdwhat)); });
}


fn main() {
    let x:Result<u8,u8> =Result::Err(102);
    {
        use std::io::Write; //XXX: needed for flush() to be seen in scope!
        moosh!(std::io::stdout().flush().ok());
    }
    trace_macros!(true); //src: file://${HOME}/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/book/first-edition/macros.html#debugging-macro-code
	fflush!();
    trace_macros!(false);
	log_syntax!(fflush!());//FIXME: looks like this does nothing?
	fflush!(stdout);
	fflush!(stderr);
    moosh!(x);//good: thread 'main' panicked at 'Could not flush x: 102', src/libcore/result.rs:860:4
}
