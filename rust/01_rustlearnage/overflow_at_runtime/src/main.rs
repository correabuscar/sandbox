use std::io;

const A:i8 = 122*2;

fn main() {
    let mut instr=String::new();
    println!("Hello, world!");
    println!("{}", std::i8::MAX);
    //println!("{}", -1 as i8);
    io::stdin().read_line(&mut instr)
        .ok()
        .expect("Failed to read line");

    let innum: i8 = match instr.trim().parse() {
        Ok(num) => num,
        Err(x) => panic!(x),
    };


    println!("{}", 2*innum);

}
