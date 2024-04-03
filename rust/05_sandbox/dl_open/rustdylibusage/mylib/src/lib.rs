pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub fn greet(name: &str) {
//#[no_mangle]
//pub extern "C" fn greet(name: *const std::os::raw::c_char) {
    println!("Hello, {}!", name);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
