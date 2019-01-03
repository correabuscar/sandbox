#![allow(unused_assignments)]
#![allow(unused_variables)]

#[derive(Debug)]
struct B(u32, u8);

impl B {
    pub fn change(&mut self) {
        //(*self).0+=1;
        self.0+=1;
        //why both work?
    }
}
fn main() {
    //let mut a: u32=10;
    //a=11;
    let mut b:  B=B(10,1);
    b.change();
    println!("{:?}",b);
    b=B(20,2);//so mut means that binding can be rebound
    b.change();//but also that the value it's already bound to can change internally
    println!("{:?}",b);

    //let c:  mut B=B(10,1);//no can do
    let mut c: &B=&b;
    c=&b;
    //c.change(); //disallowed: cannot borrow `*c` as mutable, as it is behind a `&` reference
    println!("{:?}",c);

    let mut c: &mut B=&mut b;//same as b
    c=&mut b;
    c.change();
    println!("{:?}",c);

    let c: &mut B=&mut b;
    //c=&mut b;//disallowed
    c.change();
    println!("{:?}",c);

    let c: &mut B=&mut B(40,4);
    //c=&mut b;//disallowed (wanted)
    c.change();//value it's already bound to can change (wanted)
    println!("{:?}",c);// B(41, 4)
    let x = B(100,23); *c= x; //overwritten by raisin on irc
    println!("{:?}",c);// B(100, 23)
    /* 
     * Hi. Am I missing something or, a mut binding means not only that the binding can bind to a different value but also that the value that the binding binds to can change (ie. if it's a struct, one of its fields can change)

so what if I want to allow only the value to change but not allow rebinds?
     b=B(30,3); not allowed; but b.change() allowed (which changes the value of 30 to 31)
     ok i figured it out:
let c: &mut B=&mut B(40,4); will thus deny c=...; but allow c.change() which will increase 40 by one, as per: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b51b8e6c5f06bc852b9901890d9e235d

raisin:  i can do: `let x = B(100,23); *c= x;`, tada, overwritten

     * technically I guess 'c' is an immutable binding, in this: let c: &mut B=&mut B(40,4);   So the fact that '*c=...;' works is just because the value 'c' bound to (aka the mutable reference instance) is changeable. So 'c' points to the same reference instance, it's just that the reference's inner pointer gets changed. or something(i'm trying to get it)

     hmm the mut in &mut B  has the same meaning: the reference can be changed and the value the ref points to can change internally

otherwise *c=...; would work; if only the value that the ref refers to is allowed to change internally, instead of allowing the ref to refer to other new values

so let c: &mut B=; is like: let mut c: B=;  in that sense of what mut means

so it allows both the binding(or the ref) AND the value it binds to(or the ref refers to) to change internally
so it allows both to change:
1. the binding(or the ref) to change
AND
2. the value it binds to(or the ref refers to) to change internally

     *
     */
}
