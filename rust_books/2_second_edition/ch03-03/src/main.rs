fn a() -> () {
    let _x = 2;
}
fn b() -> () {
    let _x;
    _x = 2
}
fn c() -> () {
    let _x;
    _x = 2;
}
fn d() {
    let _x;
    _x = 2;
}
fn e() {
    let _x;
    _x = 2
}
fn f() {
    let _x = 2;
}
fn g() {
    1;
}
fn main() {
    assert!(a() == b() && b() == c() && c() == d() && d() == e() && e() == f()
            && f() == g());
    println!("{:?}", a());
}
