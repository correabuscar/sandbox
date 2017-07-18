//so this is fixed now by using the clippy rust plugin! to detect variable shadowing which would
//otherwise not be detected at all (I see there's a 100 char limit, rust.vim !)

#![forbid(non_shorthand_field_patterns)]

#![warn(dead_code)]

#![warn(trivial_casts)]

#![allow(missing_docs)]

#![warn(unsafe_code)]


#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

#![warn(box_pointers)]

#![warn(trivial_numeric_casts)]
#![warn(unstable_features)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

#![feature(plugin)]

#![plugin(clippy)] //thanks to arc on irc #rust for suggesting clippy!

#![deny(clippy)]  //this doesn't work (it should imply the below, but it doesn't, so the below is needed!)) well the readme was inconsistent!
//#![deny(shadow_unrelated)]
#![deny(clippy_pedantic)] //this includes the above.
#![allow(print_stdout)] //https://github.com/Manishearth/rust-clippy/wiki#print_stdout
#![allow(missing_docs_in_private_items)]
#![warn(shadow_unrelated)] //else is error via #![deny(clippy_pedantic)] above

fn main() {
    let x=10;
    println!("{}",x);
    let x=11; //done via clippy: want a (lint check) warning here
    println!("{}",x);
    {
        let x=12; //done via clippy: want a (lint check) warning here
        println!("{}",x);
    }
    println!("{}",x);
}

