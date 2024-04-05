extern "C" {
    fn abort();
}

//#[used]
#[link(name = "custom_abort")]
extern {}

extern {
    fn dummy();
}

fn main() {
    unsafe {dummy();}
    println!("Hello, world!");
    let e=std::panic::catch_unwind(|| {
        std::process::abort(); //ahaFIXME: uncaught, why?! even tho it still calls libc::abort() eventually! doesn't it?! ok it's because of dead code elimination, my dynamic lib didn't get dynamically linked as rustc thought it's not used, hence no hook for abort()
    #[allow(unreachable_code)]
        unsafe {
            libc::abort(); //caught
            //abort(); //caught
        }
    });
    println!("e={:?}",e);


}
