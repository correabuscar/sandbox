// also posted in https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340
#![allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
struct NonCopyI32(i32);

/* original, working for i32 due to Copy trait
fn largest_1(list: &[i32]) -> Option<i32> {
    if list.is_empty() {
        return None;
    }
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;;
        }
    }

    Some(largest)
}*/

fn largest_1(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    if list.is_empty() {
        return None;
    }
    let mut largest: &NonCopyI32 = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;;
        }
    }

    Some(largest)
}

fn largest_1_2(list: &[NonCopyI32]) -> Option<NonCopyI32> {
    //using Clone instead of ref to it, idea from: https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340/10
    if list.is_empty() {
        return None;
    }
    let mut largest: NonCopyI32 = list[0].clone(); //this need derive: Clone

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }

    Some(largest)
}

fn largest0(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    let mut largest: Option<&NonCopyI32> = list.get(0);

    for item in list.iter() {
        if item > largest.unwrap() {//unwrap is safe here because 'for' can only be entered if vec has at least 1 item which means largest is not None here!
            largest = Some(item);
        }
    }

    largest
}

//#[derive(Debug)] //error: `derive` may only be applied to structs, enums and unions
fn largest1(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    //genius-level from: https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340/2
    list.iter().fold(None, |largest, item| match largest {
        // if `item` is bigger as `largest` return `item`
        Some(largest) if item > largest => Some(item),
        // else return `largest`
        largest @ Some(_) => largest,
        // first `item` return it
        None => Some(item),
    })
}

//#[derive(Debug)]
fn largest2(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    //avoid checking first item twice, but adds one more 'if' for each item checked
    let mut largest: Option<&NonCopyI32> = None; //list.get(0);

    for item in list.iter() {
        if None == largest {
            largest = Some(item);
            continue;
        } else {
            if item > largest.unwrap() {
                largest = Some(item);
            }
        }
    }

    largest
}

//#[derive(Debug)]
fn largest2_2(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    //avoid checking first item twice
    let mut largest: Option<&NonCopyI32> = None; //list.get(0);

    for item in list.iter() {
        match largest {
            //this seems worse :)
            Some(x) if x < item => largest = Some(item),
            Some(_) => (),
            None => largest = Some(item),
        }
    }

    largest
}

//#[derive(Debug)]
fn largest2_3(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    //avoid checking first item twice
    let mut largest: Option<&NonCopyI32> = None; //list.get(0);

    for item in list.iter() {
        match largest {
            Some(x) => {
                if x < item {
                    largest = Some(item)
                }
            }
            None => largest = Some(item),
        }
    }

    largest
}

//#[derive(Debug)]
fn largest3(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    let mut iterator = list.iter();
    let mut largest: Option<&NonCopyI32> = iterator.next(); //skip first

    for item in iterator {
        if item > largest.unwrap() {
            largest = Some(item);
        }
        //println!("!! {:?}",item);
    }

    largest
}

fn largest4(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    let mut iterator = list.iter();
    let mut largest: Option<&NonCopyI32> = iterator.next(); //skip first

    loop {
        let item = iterator.next();
        //println!("!! {:?}",item);
        match item {
            None => break,
            i => {
                //need a better way here
                if i > largest {
                    largest = item;
                }
            }
        }
    }

    largest
}

fn largest4_1(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    let mut iterator = list.iter();
    let mut largest: Option<&NonCopyI32> = iterator.next(); //skip first

    loop {
        let item = iterator.next();
        //println!("!! {:?}",item);
        match item {
            i @ Some(_) => {
                // thanks for the '@' to: https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340/7
                if i > largest {
                    largest = item;
                }
            }
            None => break,
        }
    }

    largest
}

fn largest5(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    // from: https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340/5
    let mut largest = None;

    for item in list.iter() {
        largest = Some(item).max(largest); //needs extra derives: Ord and Eq
                                           //looks cleaner but I don't like that it assigns to largest for each element! :o
    }

    largest
}

fn largest6(list: &[NonCopyI32]) -> Option<&NonCopyI32> { // from: https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340/11
    list.iter().max()
}

fn largest7(list: &[NonCopyI32]) -> Option<&NonCopyI32> { // from: https://users.rust-lang.org/t/solved-largest-item-in-a-possibly-empty-vector-without-early-return/24340/11
    let mut iter = list.iter();
    iter.next().map(|first| {
        iter.fold(first, |a, b| a.max(b))
    })
}

//#[derive(Debug)]
#[allow(unused_variables)]
fn smallest(list: &[NonCopyI32]) -> Option<&NonCopyI32> {
    return None; // TODO: //to be done
}

//TODO: make list of all of the above largest* functions and see that they return the same result!

//HOW to, src: https://stackoverflow.com/questions/27459640/how-to-create-a-static-array-of-strings/32383866#32383866
/*const ALL_LARGEST_FUNCS_OF_TYPE1: &'static [Sarnold] = &[
    Sarnold::Type1(largest_1),
    Sarnold::Type1(largest0),
    Sarnold::Type1(largest1),
    Sarnold::Type2(largest_1_2),//aww yeah!
    Sarnold::Type1(largest2), Sarnold::Type1(largest2_2), Sarnold::Type1(largest2_3), Sarnold::Type1(largest3), Sarnold::Type1(largest4_1),
    Sarnold::Type1(largest5),
];

const ALL_LARGEST_FUNCS_OF_TYPE2: &'static [Sarnold] =
    &[Sarnold::Type2(largest_1_2)];


enum Sarnold { //the sarnold enum thanks to person with this nick on irc :)
    Type1(fn(&[NonCopyI32]) -> Option<&NonCopyI32>),
    Type2(fn(&[NonCopyI32]) -> Option<NonCopyI32>),
}*/

const ALL_LARGEST_FUNCS_OF_TYPE1: &'static [fn(&[NonCopyI32]) -> Option<&NonCopyI32>] = &[
    largest_1,
    largest0,
    largest1,
    //largest_1_2,//different sig
    largest2,
    largest2_2, largest2_3, largest3, largest4_1,
    largest5, largest6, largest7
];

const ALL_LARGEST_FUNCS_OF_TYPE2: &'static [fn(&[NonCopyI32]) -> Option<NonCopyI32>] =
    &[largest_1_2];
//TODO: make macro so that the 'for' block only occurrs once; the realize how else can the macro concept be used only on the call to largest, without any need to wrap the 'for' block, but still no need to dup the 'for' block!

fn main() {

    for largest in ALL_LARGEST_FUNCS_OF_TYPE1 { //.iter().chain(ALL_LARGEST_FUNCS_OF_TYPE2.iter()) {
        //let largest = largest_1;
        //let largest = dbg!(largest2_3); //error[E0277]: `for<'r> fn(&'r [NonCopyI32]) -> std::option::Option<&'r NonCopyI32> {largest2_3}` doesn't implement `std::fmt::Debug`

        //println!("{:?}",largest); //error[E0277]: `for<'r> fn(&'r [NonCopyI32]) -> std::option::Option<&'r NonCopyI32> {largest2_3}` doesn't implement `std::fmt::Debug`
        let number_list = vec![
            NonCopyI32(34),
            NonCopyI32(50),
            NonCopyI32(25),
            NonCopyI32(100),
            NonCopyI32(65),
        ];
        //let l=Box::new(largest);
        /*let result = match largest { //but ofc: error[E0308]: match arms have incompatible types
           Sarnold::Type1(func) => Box::new(func),
           //func(&number_list),
           Sarnold::Type2(func) => Box::new(func),
           //func(&number_list),
        };*/
        let result=largest(&number_list);
        println!("The largest number is {:?}", result);
        println!("smallest {:?}", smallest(&number_list));

        let number_list = vec![
            NonCopyI32(102),
            NonCopyI32(34),
            NonCopyI32(6000),
            NonCopyI32(89),
            NonCopyI32(54),
            NonCopyI32(2),
            NonCopyI32(43),
            NonCopyI32(8),
        ];

        let result = largest(&number_list);
        println!("The largest number is {:?}", result);
        println!("smallest {:?}", smallest(&number_list));

        let number_list = vec![];

        let result = largest(&number_list);
        println!("The largest number is {:?}", result);
        println!("smallest {:?}", smallest(&number_list));

        //is it non-Copy?
        //    let a:NonCopyI32=NonCopyI32(10);
        //    let mut b=a;// moved here,    = note: move occurs because `a` has type `NonCopyI32`, which does not implement the `Copy` trait
        //    b.0=11;
        //    println!("{:?}",a);
    }
}


