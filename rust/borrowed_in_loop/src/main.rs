fn a_func(_s:String) {
}
//no this isn't it, I need it to say: "Value moved here, in previous iteration of loop"?
fn main() {
//    println!("Hello, world!");
//    for i in 1..=10 {
//        let s:String="moo".to_string().clone(); //fail1
//        a_func(s); //fail2
//        println!("{i}{}",s); //fail3
//        println!("{i}{}",s);
//    }
//    for i in 1..=10 {
//        let s:String="moo".to_string();
//        a_func(s.clone());
//        println!("{i}{}",s);
//        println!("{i}{}",s);
//    }


//This still isn't it, but I guess close enough:
    //src: https://stackoverflow.com/questions/28800121/what-do-i-have-to-do-to-solve-a-use-of-moved-value-error
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10001 {
        num += 2;
        //primes=Vec::new();
        if vector_is_prime(num, primes) {
            count += 1;
            //primes=Vec::new();
            primes.push(num);
        }
    }
}

fn vector_is_prime(num: u64, p: Vec<u64>) -> bool {
    for i in p {
        if num > i && num % i != 0 {
            return false;
        }
    }

    true
}
