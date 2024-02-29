#![feature(type_name_of_val)]

extern crate pkg_config;

use core::any;
use std::io::ErrorKind;

fn main() {
    let r=pkg_config::get_variable("x11", "libdir"); // ie. $ pkg-config --variable=libdir x11
    // match r {
    //     Err(e) => {
    //         match e {
    //             pkg_config::Error::Command { ref command, ref cause } => {
    //                 println!("command='{command}'");
    //                 //let a:i64=cause;
    //                 println!("cause='{} {} {:?}'", cause, any::type_name_of_val(&cause), cause);
    //                 match cause.kind() {
    //                     ErrorKind::NotFound => { println!("matchNF {e}")},
    //                     _ => {}
    //                 };
    //             },
    //             _ => {}
    //         };
    //     },
    //     Ok(_) => {
    //         println!("Hello, world! {r:?}");
    //     }
    // };

    if let Err(pkg_config::Error::Command {  command:ref _cmd, ref cause }) = r {
        if cause.kind() == ErrorKind::NotFound {
            panic!("You don't have the pkg-config command installed. ie. {r:#?}");
        }
    // } else if let Ok(variable) = r {
    //     println!("Hello, world! {variable:?}");
    // } else {
    //     // Handle any unexpected errors that don't match CommandError
    }
    println!("Hello, world! {r:?}");
}
