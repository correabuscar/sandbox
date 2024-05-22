use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let current_dir = env::current_dir().unwrap(); //needed else temp value dropped compiler error!
    let current_dir_str = current_dir.to_str().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("project_dir.rs");
    let mut file = File::create(&dest_path).unwrap();

    writeln!(
        &mut file,
        "pub const PROJECT_DIR: &str = \"{}/\";",
        current_dir_str
    )
    .unwrap();
}
