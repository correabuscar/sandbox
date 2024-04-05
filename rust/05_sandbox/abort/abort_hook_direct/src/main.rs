#[no_mangle]
pub extern "C" fn abort() {
    eprintln!("Abort intercepted!");
    std::process::exit(128+6); //134 is SIGABRT's exit code 128+6
}

fn main() {
    println!("Hello, world!");
    let e=std::panic::catch_unwind(|| {
        unsafe {
            std::process::abort(); //ahaFIXME: uncaught, why?! even tho it still calls libc::abort() eventually! doesn't it?! ok it's because of dead code elimination, my dynamic lib didn't get dynamically linked as rustc thought it's not used, hence no hook for abort()
            //libc::abort(); //caught
            //abort(); //caught
        }
    });
    println!("e={:?}",e);
}
