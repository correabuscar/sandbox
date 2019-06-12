fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
    let a = 1;
    match a {
        1 => println!("1"),
        _ => println!("nah"),
    }
}
