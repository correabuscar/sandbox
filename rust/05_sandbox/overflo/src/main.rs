fn main() {
    println!("Hello, world!");
    let mut max=0;
    let mut i:u64=1;
    //let a=2*9223372036854775808;
    while max<=i {
        println!("!! i={} max={}", i, max);
        i*=2;//overflow detected when cargo run (without --release !) thread 'main' panicked at 'attempt to multiply with overflow'
        if i>max {
            max=i;
        }
    }
    println!("!! Ended: i={} max={}",i,max);//reached when: cargo run --release
}
