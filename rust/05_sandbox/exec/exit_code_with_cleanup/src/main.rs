use std::process::ExitCode;
struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        eprintln!("cleaning up stuff");
    }
}
fn main() -> ExitCode {
    let _f = Foo;
    //so return exit code different than 0 or 1, while also cleaning up!
    return ExitCode::from(3);
}
