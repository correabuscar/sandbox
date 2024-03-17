//src: file://${HOME}/build/2nonpkgs/rust.stuff/book/second-edition/book/ch10-02-traits.html#fixing-the-largest-function-with-trait-bounds
// implemented ^ without the Copy trait!!

//with Copy trait:
fn largest_ct<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//without Copy trait:
fn largest<T: PartialOrd >(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list.iter() { //item is &T
        //let _:() = item;
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = largest_ct(&number_list);
    println!("The largest number is {}", result);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let result = largest_ct(&char_list);
    println!("The largest char is {}", result);
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

