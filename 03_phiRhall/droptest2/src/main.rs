use std::sync::Arc;
use std::sync::Weak;

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

static mut UNI: Option<Weak<Droppable>> = None; // thanks to <Mutabah> on #rust irc.mozilla.org for the Arc Weak idea!

macro_rules! pln {
    // why 'ident' ? https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
    ($($arg:tt)*) => ({
        {
            let name;
            { //this whole block is non-thread safe
            let arc_weak=unsafe {
                UNI.take().expect("Tried to use macro 'pln' before UNI got set!")
            };
            let arc_strong=arc_weak.upgrade().unwrap();
            name=arc_strong.name;
            unsafe { UNI=Some(arc_weak) };
            }

        println!("{}:{} {}"
                 ,1 //reduced
                 ,name
                 ,format!($($arg)*)); //thanks to <UndeadLeech> on #rust irc.mozilla.org for the how-to-prepend idea!
        } //inner block for temp vars
    })
}

fn main() {
    let s = Droppable { name: "someting" };
    let arc_strong = Arc::new(s);
    let arc_weak = Arc::downgrade(&arc_strong);

    unsafe {
        assert!(UNI.is_none());
        UNI = Some(arc_weak);
    };
    pln!("test");
    pln!("test2");
    //let _s = unsafe { UNI.take().unwrap() }; //not needed anymoreXXX: uncomment this for drop() to work!
    println!("end");
}
