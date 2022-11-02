#![deny(clippy::all, clippy::pedantic, clippy::nursery, warnings, future_incompatible, nonstandard_style, non_ascii_idents, clippy::restriction, rust_2018_compatibility, rust_2021_compatibility, unused)]
#![allow(clippy::print_stdout, clippy::use_debug, clippy::missing_docs_in_private_items)]

#![allow(clippy::blanket_clippy_restriction_lints)] //workaround clippy


#[derive(Debug)]
struct MyBool(bool);
fn main() {
    let mut my_has_spawned:MyBool=MyBool(false);
    //...
    let handler=std::thread::spawn(move || {
        println!("Before {my_has_spawned:?}!"); //MyBool(false)
        //my_has_spawned=MyBool(true);
        my_has_spawned.0=true;
        println!("Set {my_has_spawned:?}!"); // MyBool(true)
    });
    #[allow(clippy::unwrap_used)]
    handler.join().unwrap();
    println!("Current {my_has_spawned:#?}!"); // value borrowed here after move, XXX: this is what
                                              // I wanted!
}
