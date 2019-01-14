#![allow(unused)]
use std::env;
use std::fs;


fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let subdir="subdir";
    fs::remove_dir(subdir);//don't care if fails
    fs::create_dir(subdir)?;
    let path=path.join(subdir);
    env::set_current_dir(path)?;
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    println!("Removing directory '{}'", path.display());
    fs::remove_dir(&path)?;
    println!("Done removed directory. Next, getting current(now-inexistent) directory:");
    let strpath=path.to_str().unwrap();
    let path = env::current_dir().expect(
        //path.to_str().unwrap()
        strpath
        );//Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    println!("The current directory is {}", path.display());
    Ok(())
}
