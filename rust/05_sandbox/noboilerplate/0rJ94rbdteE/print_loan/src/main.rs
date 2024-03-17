fn capitalize1(s:String) -> String {
    s.to_uppercase()
}

fn capitalize2(s:&str) -> String {
    s.to_uppercase()
}

fn main() {
    let name:&str="tris";
    let capitalized_name = capitalize2(name);
    println!("{}",capitalized_name);
    println!("{}",name);

    let name:&str="tris";
    let capitalized_name = capitalize1(name.to_string());
    println!("{}",capitalized_name);
    println!("{}",name);

    //only this variant fails:
    let name:String="tris".to_string();
    let capitalized_name = capitalize1(name); // moved here
    println!("{}",capitalized_name);
    println!("{}",name); // error[E0382]: borrow of moved value: `name`
}
