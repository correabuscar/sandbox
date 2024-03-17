fn main() {
use std::ffi::OsString;

//let mut os_string = "foo".into_os_string();//not a method
//let mut os_string = Into::into::<OsString>("foo");// can't:  error: expected 0 type parameters
let mut os_string = Into::<OsString>::into("foo");// thanks to jsimmmons on freenode
//let mut os_string = <OsString as Into>::into("foo");//nope!
//let mut os_string = <OsString as Into<OsString>>::into("foo");//nope!
//let mut os_string: OsString = "foo".into();//works
//let mut os_string = OsString::from("foo");//works

assert_eq!(&os_string, "foo");

os_string.clear();
assert_eq!(&os_string, "");
}
