fn last_char_of_first_line(text: &str) -> Option<char> {
    return text.lines().next()?.chars().last();
    //return text.chars().last();
}

fn main() {
    println!("Hello, world!");
    let a=last_char_of_first_line("");
    println!("{a:?}");
    let a=last_char_of_first_line("\nzZz");
    println!("{a:?}");
    let a=last_char_of_first_line("aA\nzZz");
    println!("{a:?}");
}
