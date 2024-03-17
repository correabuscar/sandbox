#![allow(unused)]

#[derive(Debug)]
enum Opt<T> { //a non-Copy Option<T> XXX actually Option<T> is Copy only when T is Copy ; not so for this Opt<T> which is never Copy
    Some(T),
    None,
}

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    fn plus_one2(x: Opt<i32>) -> Opt<i32> {
        match x {
            Opt::None => Opt::None,
            Opt::Some(i) => Opt::Some(i + 1),
        }
    }

    let a= Some("a");
    let b=a;//NOTtakes ownership, only if String, not if just &str ie. just "a"
    println!("{:?}",a);
    let c= Some("c".to_string());
    let d=c;//takes ownership, only if String, not if just &str ie. just "a"
    println!("{:?}",c);
    let a2= Opt::Some("a2");
    let b2=a2;//takes ownership because Opt isn't Copy, only if String, not if just &str ie. just "a"
    println!("{:?}",a2);
    let c2= Opt::Some("c2".to_string());
    let d2=c2;//takes ownership because Opt isn't Copy but also T is the non-Copy String
    println!("{:?}",c2);
    let five = Some(5);
    let f5=five;//doesn't take ownership
    let five2= Opt::Some(5);
    let f5_2=five2;//takes ownership
    let six = plus_one(five);
    let six2 = plus_one2(five2);
    let none = plus_one(None);
    println!("{:?}",f5);
    println!("{:?}",f5_2);
}

