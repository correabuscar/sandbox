//src: https://users.rust-lang.org/t/why-does-return-extend-borrowing-to-the-entire-function/5101
//issue: https://github.com/rust-lang/rust/issues/30223
//rfc: https://github.com/rust-lang/rfcs/issues/811
use std::str::FromStr;

fn borrow_test(string: &mut String) -> &str {
    {
        let str = string.as_mut_str();
        if str.starts_with('H') {//workaround here: https://users.rust-lang.org/t/why-does-return-extend-borrowing-to-the-entire-function/5101/5?u=xftroxgpx
            //XXX uncomment this:
            //return str;//"Why does returning the str extend the borrowing scope to the entire function?"
            //commented out so travis doesn't fail!
            //str
            //^same effect with implicit return (even tho they are supposedly different https://github.com/rust-lang/rust/issues/43837#issuecomment-322030201  )
        }
        //}else {
    }
    string.push('!');//XXX error[E0499]: cannot borrow `*string` as mutable more than once at a time
    return string.as_str();
//}
}

fn main() {
    let mut string = String::from_str("Hello").unwrap();
    println!("{}", borrow_test(&mut string));
}
