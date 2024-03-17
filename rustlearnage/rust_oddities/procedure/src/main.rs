fn moo() {
}

fn moo2() -> (){
}

fn main() {
    //panic!("Hello, world!");
    let _f:() =moo();
    println!("{:#?} {:?} {:?}", _f, moo(), moo2());
}
