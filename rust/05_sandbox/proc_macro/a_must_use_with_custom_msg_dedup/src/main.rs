const MUST_USE_MSG: &str = "Custom message here"; //FIXME: failed to make it use this!

//use my_macro::must_use_with_msg;
use a_must_use_with_custom_msg_dedup::must_use_with_msg;

macro_rules! must_use_with_const_msg {
    ($msg:expr, $item:item) => {
        #[must_use_with_msg($msg)]
        $item
    };
}


//#[must_use_with_msg("MUST_USE_MSG")]
//#[must_use_with_msg(MUST_USE_MSG)]
must_use_with_const_msg!(MUST_USE_MSG,
fn must_use_function() -> i32 {
    42
});

fn main() {
    println!("Hello, world!");
    must_use_function();
}
