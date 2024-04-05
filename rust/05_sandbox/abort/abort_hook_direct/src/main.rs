/*
 "When you call libc::abort() from within your program, Rust first searches for the abort() symbol within the current binary. If it finds a matching symbol (i.e., your custom implementation), it uses that instead of the libc::abort() function from the standard C library.

This behavior is consistent with Rust's symbol resolution rules and allows you to override standard library functions with custom implementations within your program. "  - chatgpt 3.5
*/
#[no_mangle]
pub extern "C" fn abort() {
    eprintln!("Abort intercepted!");
    std::process::exit(128+6); //134 is SIGABRT's exit code 128+6
}

fn main() {
    println!("Hello, world!");
    let e=std::panic::catch_unwind(|| {
        //this gets caught first, unless commented out
        std::process::abort(); //ahaFIXME: uncaught, why?! even tho it still calls libc::abort() eventually! doesn't it?! ok it's because of dead code elimination, my dynamic lib didn't get dynamically linked as rustc thought it's not used, hence no hook for abort()
        #[allow(unreachable_code)]
        unsafe {
            libc::abort(); //caught
            //abort(); //caught
        }
    });
    println!("e={:?}",e);
}
