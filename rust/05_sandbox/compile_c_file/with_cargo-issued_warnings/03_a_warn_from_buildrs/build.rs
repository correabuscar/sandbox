// build.rs

fn main() {
    println!("cargo:warning=Building C code..."); //this also works as a cargo warning hmm

    //cargo::warning is too new(end of 2023) https://github.com/rust-lang/cargo/commit/9ebe3b332a51cf413a2ee50d011339633bf2ed22
}

