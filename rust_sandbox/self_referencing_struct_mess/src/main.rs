struct Data<'a> {
    data_ref: Option<&'a String>,
    data: String,
}

impl<'b> Data<'b> {
    fn set_data(&mut self, s: &str) {
        self.data = s.to_string();
    }
    fn new(s:&str) -> Data<'b> {
        let data = s.to_string();
        Data {
            data: data,
            data_ref: None,
        }
    }

    fn fun1(&'b mut self, element: i32) -> () {
        self.data_ref = Some(&self.data);
        println!("fun1 from {} with data:{}", element, self.data);
        ()
    }
}

struct DataBuilder {
    // Probably lots of optional fields.
    data: String,
}

impl DataBuilder {
    fn new(s: &str) -> DataBuilder {
        // Set the minimally required fields of Foo.
        DataBuilder{data:s.to_string()}
    }

    /*fn named(mut self, name: &str) -> FooBuilder {
        // Set the name on the builder itself, and return the builder by value.
    }

    // More methods that take `mut self` and return `FooBuilder` setting up
    // various aspects of a Foo.
    ...
*/
    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the builder as a template for constructing
    // many Foos.
    fn finish(&self) -> Data{
        // Create a Foo from the FooBuilder, applying all settings in FooBuilder to Foo.
        let mut e= Data { data: self.data.clone(), data_ref:None };
        //e.data_ref=Some(&self.data);
        e
    }
}

fn main() -> () {
    let mut d: Data = DataBuilder::new("mydata").finish();
    let v: Vec<i32> = vec![1, 2];

    d.fun1(v[0]);
    d.fun1(v[1]);
    //d = *d.fun1(v[1]);
    /*for element in v {
        d.fun1(element);
        d.fun1(element);
    }*/
}

