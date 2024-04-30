use std::io::IsTerminal;
fn main() {
    if std::io::stdout().is_terminal() {
        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }
}
