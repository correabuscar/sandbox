fn main() {
    let mut s1 = String::from("hello");

    //let len:i32;//ok so this used to work?
    let mut len:usize=1;
    {
      //(s1, len) = calculate_length(s1); // how to do this without a 'let s1' or without a let in general? - i don't remember what i wanted to do here?! was it how do I NOT-shadow s1? but instead assign to it? yep, that must be it
      //s1=s2;
        let (s1_, len_) = calculate_length(&s1);
        //s1=*s1_;//if we assume returned s1_ is different than s1, then how to assign s1? hmm
        len=len_;
      println!("s1={} len={}",s1,len);
    }

    println!("The length of '{}' is {}.", s1, len);

    //ok new thing:
    let mut s1 = String::from("hello");
    let mut s2: usize = 1;
    println!("Before s1='{}' s2='{}'.", s1, s2);
    {
        let (s11, s22) = reassign();//how do I assign existing s1 and s2, without using temp vars? apparently it's not currently possible!
        s1=s11;
        s2=s22;
        println!("Middle s1='{}' s2='{}'.", s1, s2);
    }
    println!("After s1='{}' s2='{}'.", s1, s2);//should have same values as Middle
}

fn calculate_length(s: &String) -> (&String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (&s, length)
}

fn reassign() -> (String, usize) {
    (String::from("boo"),2)
}

