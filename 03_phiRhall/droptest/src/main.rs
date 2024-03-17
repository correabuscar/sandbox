struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

static mut UNI: Option<Droppable> = None;
fn main() {
    let s = Droppable { name: "someting" };
    unsafe {
        UNI = Some(s);
    };
    //let _s = unsafe { UNI.take().unwrap() }; //XXX: uncomment this for drop() to work!
    println!("end");
}
