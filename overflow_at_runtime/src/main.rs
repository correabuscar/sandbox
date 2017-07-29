use std::io;

fn main() {
    let mut instr=String::new();
    println!("Hello, world!");
    io::stdin().read_line(&mut instr)
        .ok()
        .expect("Failed to read line");

    let innum: i8 = match instr.trim().parse() {
        Ok(num) => num,
        Err(x) => panic!(x),
    };


    println!("{}", std::i8::MAX+innum);

}
