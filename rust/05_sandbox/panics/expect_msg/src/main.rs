use std::fmt::{Display, self};

struct MyStruct;

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        None::<i32>.expect("oh snap");//FIXME: no message
        todo!();//ignore this, it's for return
    }
}

fn main() {
    //None::<u32>.expect("unexpected None");// correctly shows
    let instance = MyStruct;
    assert!(false, "oh no, '{}' was unexpected", instance);
}
