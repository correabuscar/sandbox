#![allow(unused_variables)]
fn main() {
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        let mut i2 = 1;
        let a=for (i, &item) in bytes.iter().enumerate() {
            i2 = i;
            if item == b' ' {
                break; //return &s[0..i];
            }
        };
        println!("!{:?}!",a);// ()

        let mut i3=0;
        loop {
            //(i, &item) in bytes.iter().enumerate() {
            //i2 = i;
            //let item=bytes[i3];
            if bytes[i3] == b' ' || i3 >= bytes.len()-1 {
                break;// i; //return &s[0..i];
            }
            i3+=1;
        };
        let i4=bytes.iter().position(|&b| b == b' ');
        println!("{} {} {:?}", i2, i3, i4);
        &s[..i2]
    }
    println!("{:?}", first_word(&String::from("helloworldtwo!")));
    //println!("!{}!", format!("{:?}", first_word(&String::from("hello world two!"))).trim_matches('"'));
    //println!("!{}!", format!("{:?}", first_word(&String::from("hello world two!"))).trim_matches(char::is_ascii_punctuation));//fail, but see https://github.com/rust-lang/rust/issues/57307 or project is_ascii_punctuation
    assert!('"'.is_ascii_punctuation());
    println!("!{}!", format!("{:?}", first_word(&String::from("hello world two!"))).trim_matches(|x| x == '"'));

}

