#![feature(core_intrinsics)] //must be crate-wide ie. #![]
fn print_type_of<T>(_: &T) { //src: https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust/29168659#29168659
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    //this is fine:
    let a=1.0;// defaulting to f64
    print_type_of(&a);// f64
    println!("{}",a);// 1

    //this isn't:
    let b=-4000000000;// defaulting to i32 despite the 'literal out of range for i32'
    //Actually, nevermind, since this behaviour is documented as such: "Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type."
    //see https://github.com/rust-lang/rust/issues/47739#issuecomment-377392401
    //let b=-4000000000_i64;// now properly inferred as i64
    print_type_of(&b);// i32
    println!("{}",b);// 294967296
}

