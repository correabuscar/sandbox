// also posted in https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340?u=xftroxgpx
#[derive(Debug, PartialOrd, PartialEq)]
struct NonCopyI32(i32);

fn largest(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    let mut largest:Option<&NonCopyI32> = list.get(0);

    for item in list.iter() {
        if item > largest.unwrap() { //unwrap is safe here because 'for' can only be entered if vec has at least 1 item which means largest is not None here!
            largest = Some(item);
        }
        //let a:u32=item; //to see type of `item`
    }

    largest
}

fn main() {
    let number_list = vec![NonCopyI32(34), NonCopyI32(50), NonCopyI32(25), NonCopyI32(100), NonCopyI32(65)];

    let result = largest(&number_list);
    println!("The largest number is {:?}", result);

    let number_list = vec![NonCopyI32(102), NonCopyI32(34), NonCopyI32(6000), NonCopyI32(89), NonCopyI32(54), NonCopyI32(2), NonCopyI32(43), NonCopyI32(8)];

    let result = largest(&number_list);
    println!("The largest number is {:?}", result);

    let number_list = vec![];

    let result = largest(&number_list);
    println!("The largest number is {:?}", result);

    //is it non-Copy?
//    let a:NonCopyI32=NonCopyI32(10);
//    let mut b=a;// moved here,    = note: move occurs because `a` has type `NonCopyI32`, which does not implement the `Copy` trait
//    b.0=11;
//    println!("{:?}",a);
}
/*
fn main() {
    let number_list:Vec<i32> = vec![];

    let mut largest:Option<&i32> = number_list.get(0);

    for number in number_list {
        if number > *largest.unwrap() {
            largest = Some(&number);
        }/*
        if number > largest {
            largest=number;
        }*/
    }

    println!("The largest number is {:?}", largest);
}*/
