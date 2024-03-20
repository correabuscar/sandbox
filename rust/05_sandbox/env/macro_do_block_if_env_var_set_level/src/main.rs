//#[macro_export]
//^ that is u wanna make it public (like if this was a lib maybe), exports it to root crate::
/// Executes block based on the value of environment variable TEADEBUG
/// eg. $ TEADEBUG=4 cargo run
/// set to 0 or unset, to not execute block;
/// set to 1 or anything non-number to execute blocks with level 1
/// set to any number >1 to execute blocks with that level or below it!
macro_rules! tea {
    ($level:expr, $block:block) => {
        if let Ok(var_value) = std::env::var("TEADEBUG") {
            let level= var_value.parse::<u32>().unwrap_or(1);
            if level > 0 && $level <= level {
                    $block
            }
        }
    };
}

fn main() {
    tea!(1,{
        println!("Environment variable is set to a non-zero value. {}", std::env::var("TEADEBUG").unwrap());
    });
    tea!(2,{
        println!("level 2 debug message");
    });
    tea!(3,{
        println!("level 3 debug message");
    });
    tea!(99,{
        println!("level 99 debug message");
    });
}

