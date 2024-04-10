const OLD_CARGO_WARNING_SYNTAX: &str = "OLD:";
const NEW_CARGO_WARNING_SYNTAX: &str = "NEW:";

fn main() {
    //let stdout = "OLD: Warning messageOLD";
    let stdout = "NEW: Warning messageNEW";

    if let Some(warning) = stdout
        .strip_prefix(OLD_CARGO_WARNING_SYNTAX)
        .or(stdout.strip_prefix(NEW_CARGO_WARNING_SYNTAX))
    {
        println!("Warning found: {}", warning);
    } else {
        println!("No warning found");
    }
}

