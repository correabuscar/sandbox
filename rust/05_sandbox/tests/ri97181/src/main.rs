use std::fmt::{Display, self};

struct MyStruct;

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_does_stuff() {
        let instance = MyStruct;

        assert!(false, "oh no, '{}' was unexpected", instance);
    }
}
