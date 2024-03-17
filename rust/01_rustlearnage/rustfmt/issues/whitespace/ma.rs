macro_rules! log {
    ($state:expr) => {
        
            println!("log({}): {}", 3, $state);
            println!("log({}): {}", 4, $state);
        
    };
}

fn main() {
    let state: &str = "reticulating splines";
    if 1==2 {    log!(state);
    }
}