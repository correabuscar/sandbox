
struct Soo(String);

impl Drop for Soo {
    fn drop(&mut self) {
        //println!("!drop!");//it is dropped!
    }
}

fn main() {
    println!("Hello, world!");
    loop {
    loop {
        //no change in VIRT  26.6m   5.9m RES
        let mut a: Soo=Soo(String::with_capacity(10000000));
        print!("{}",a.0);
        //a.0.push('a');
        break;
    }
    }
}
