extern "C" {
    fn abort();
}

#[link(name = "custom_abort")]
extern {}

fn main() {
    println!("Hello, world!");
    let e=std::panic::catch_unwind(|| {
        unsafe {
            libc::abort();
            //abort();
            //std::process::abort(); //FIXME: uncaught, why?! even tho it still calls libc::abort() eventually! doesn't it?!
        }
    });
    println!("e={:?}",e);


}
