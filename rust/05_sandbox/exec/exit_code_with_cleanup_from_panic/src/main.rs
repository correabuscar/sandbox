#![feature(panic_update_hook)]
#![feature(panic_always_abort)]
#![feature(rt)]

use std::process::ExitCode;
struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        eprintln!("!!!!!! cleaning up stuff");
    }
}
fn main() -> ExitCode {
    //if std::env::var("RUST_BACKTRACE").is_err() {
    //    std::env::set_var("RUST_BACKTRACE", "1");
    //}

    std::panic::update_hook(move |prev, info| { // E0658: use of unstable library feature 'panic_update_hook'
                                                //println!("Print custom message and execute panic handler as usual");
        prev(info);
        //println!("fooooooooooooo");//yes this is reached
        // Manually flush stdout, else any printed that didn't end in newline won't be seen! (untrue if using std::process:exit() which does call cleanup)
        //use std::io::Write;//else can't see: no method named `flush` found for struct `Stdout` in the current scope: method not found in `Stdout`
        //std::io::stdout().flush().unwrap();
        // Manually flush stderr, else any printed that didn't end in newline won't be seen!
        //std::io::stderr().flush().unwrap();
        /// set the exit code (override the 101 one!)
        std::rt::EXIT_CODE_ON_PANIC.store(21, std::sync::atomic::Ordering::Relaxed);
        //std::process::exit(20);//XXX: this calls rt::cleanup() to flush, internally! it just doesn't call the drop()/destructors for Foo for example!
        //ahwellFIXME: was cleanup executed tho?! the drop() isn't if I exit here! but anyway the cleanup func is https://github.com/rust-lang/rust/blob/59a4f02f836f74c4cf08f47d76c9f6069a2f8276/library/std/src/rt.rs#L105 and executed by line 146 below. But something still flushes stdout/stderr even if I exit here!(it's exit() itself which calls rt::cleanup() )
    });
    let _f = Foo;
    print!("!!!!!! no eol print");//still gets printed(even w/o the flushes from above!), even tho after the panic! interesting! so maybe cleanup() from library/std/src/rt.rs does get executed?
    //std::process::exit(6);//XXX: even this somehow flushes that noeol print!() ok i see why now, because this exit() is purposefully calling rt::cleanup() which does the flushing!
    //std::panic::always_abort();//XXX: no flushing when abort-ing during panic!
    panic!("wtw man");
    //so return exit code different than 0 or 1, while also cleaning up!
    return ExitCode::from(3);
}
