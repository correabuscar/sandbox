//#[macro_export]
//^ that is u wanna make it public (like if this was a lib maybe), exports it to root crate::
macro_rules! tdbg {
    ($block:block) => {
        if let Ok(var_value) = std::env::var("TEADEBUG") {
            if var_value != "0" {
                $block
            }
        }
    };
}

fn main() {
    tdbg!({
        println!("Environment variable is set to a non-zero value.");
    });
}

