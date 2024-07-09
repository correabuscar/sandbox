use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    /// "As  typically  implemented, getenv() returns a pointer to a string within the environment list.  The caller must
    /// take care not to modify this string, since that would change the environment of the process.
    /// The implementation of getenv() is not required to be reentrant.  The string pointed to by the  return  value  of
    /// getenv()  may  be  statically  allocated,  and  can  be  modified  by  a subsequent call to getenv(), putenv(3),
    /// setenv(3), or unsetenv(3)." - man 3 getenv
    fn getenv(name: *const c_char) -> *const c_char;
}

/// get_env without heap allocations
fn get_env_var(name: &CStr) -> Option<&CStr> {
    unsafe {
        let value = getenv(name.as_ptr());
        if value.is_null() {
            None
        } else {
            Some(CStr::from_ptr(value))
        }
    }
}

#[cfg(test)] // we only need the macro below during test(s), otherwise it's not used or shouldn't be used!
//#[macro_export]
//^ that is u wanna make it public (like if this was a lib maybe), exports it to root crate::
/// Executes block based on the value of environment variable TEADEBUG
/// uses heap allocation(s) to get the env. var.!
/// eg. $ TEADEBUG=4 cargo run
/// set to 0 or unset, to not execute block;
/// set to 1 or anything non-number to execute blocks with level 1
/// set to any number >1 to execute blocks with that level or below it!
macro_rules! tea_ha {
    ($level:expr, $block:block) => {
        if let Ok(var_value) = std::env::var("TEADEBUG") {
            let level= var_value.parse::<u32>().unwrap_or(1);
            if level > 0 && $level <= level {
                    $block
            }
        }
    };
}

//#[macro_export]
//^ that is u wanna make it public (like if this was a lib maybe), exports it to root crate::
/// Executes block based on the value of environment variable TEADEBUG
/// does NOT use any heap allocation(s) to get the env. var.!
/// eg. $ TEADEBUG=4 cargo run
/// set to 0 or unset, to not execute block;
/// set to 1 or anything non-number to execute blocks with level 1
/// set to any number >1 to execute blocks with that level or below it!
macro_rules! tea {
    ($level:expr, $block:block) => {
        //use std::ffi::CStr;
        if let Some(value_cstr) = $crate::get_env_var(unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"TEADEBUG\0") }) {
            if let Ok(var_value) = value_cstr.to_str() {
                let level = var_value.parse::<u32>().unwrap_or(1);
                if level > 0 && $level <= level {
                    $block
                }
            }
        }
    };
}


#[cfg(test)]
fn try_teas(val:u32, should_be:bool) {
    let mut tea1:bool=false;
    let mut tea2:bool=false;
    tea!(val, { tea1=true });
    tea_ha!(val, { tea2=true });
    assert_eq!(tea1, tea2);
    assert_eq!(tea1, should_be);
}

#[test]
fn test_teas_work_the_same() {
    std::env::set_var("TEADEBUG","1");
    try_teas(1, true);
    try_teas(2, false);
    try_teas(3, false);
    try_teas(99, false);
    try_teas(100, false);
    std::env::set_var("TEADEBUG","4p");//it's 1 by default, if unparsable or empty
    try_teas(1, true);
    try_teas(2, false);
    try_teas(3, false);
    try_teas(99, false);
    try_teas(100, false);
    std::env::set_var("TEADEBUG","");
    try_teas(1, true);
    try_teas(2, false);
    try_teas(3, false);
    try_teas(99, false);
    try_teas(100, false);
    std::env::set_var("TEADEBUG","2");
    try_teas(1, true);
    try_teas(2, true);
    try_teas(3, false);
    try_teas(99, false);
    try_teas(100, false);
    std::env::set_var("TEADEBUG","3");
    try_teas(1, true);
    try_teas(2, true);
    try_teas(3, true);
    try_teas(99, false);
    try_teas(100, false);
    std::env::set_var("TEADEBUG","0");//0 is same as unset
    try_teas(1, false);
    try_teas(2, false);
    try_teas(3, false);
    try_teas(99, false);
    try_teas(100, false);
    std::env::remove_var("TEADEBUG");
    try_teas(1, false);
    try_teas(2, false);
    try_teas(3, false);
    try_teas(99, false);
    try_teas(100, false);
    std::env::set_var("TEADEBUG","99");
    try_teas(1, true);
    try_teas(2, true);
    try_teas(3, true);
    try_teas(99, true);
    try_teas(100, false);
}

fn main() {
    tea!(1,{
        println!("level 1 debug message: Environment variable is set to a non-zero value. {}", std::env::var("TEADEBUG").unwrap());
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

