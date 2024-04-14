#![feature(never_type)]

fn some0() -> () {
    return {
        let _y = 6;
    };

}
fn some1(_x:i32) -> () {
    return ();
}
fn some2(_x:i32) {

}
fn some3(_x:i32) {
    return {
        let _y = 6;
        ()
    }
}
fn some4(_x:i32) {
    #[allow(unreachable_code)]
    return return ();
}
#[allow(unreachable_code)]
fn some5(_x:i32) {
    println!("aaa{}",return ());
}

/*
i see
 */

fn some6() {
    #![allow(unreachable_code)] // applies to this function's contents only.

    #[allow(unused_variables)]
    let r#return:! = return; // the 'never' type
    return ();//r#return; // this works, points correctly.
    return r#return;//XXX:rustbug: this points to this r#return instead of the above '= return;' one, as the unreachable after THIS.
}

// fn aa() {
//     return return;
// }

fn some7() -> i32 {
    let r#return:i32 = 1;
    return r#return;
}
fn main() {
    some0();
    some1(1);
    some2(2);
    some3(3);
    some4(4);
    some5(5);
    assert_eq!( (), some6() );
    some7();
}
