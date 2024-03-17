fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    //let new_setting_value = None;

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            // so: None,Some() , Some(),None and None,None
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}
