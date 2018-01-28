
// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
   // The file has been placed there by the build script.
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!("Hello, world! it's me {}", built_info::PKG_NAME);
    //let a=built_info.iterator();
    //src: https://github.com/lukaslueg/built
    println!("This is version {}{}, built for {} by {}.",
       built_info::PKG_VERSION,
       built_info::GIT_VERSION.map_or_else(|| "".to_owned(),
                                           |v| format!(" (git {})", v)),
       built_info::TARGET,
       built_info::RUSTC_VERSION);
    eprintln!("I was built with profile \"{}\", features \"{}\" on {} using {{}}",
       built_info::PROFILE,
       built_info::FEATURES_STR,
       built_info::BUILT_TIME_UTC
       //,built_info::DEPENDENCIES_STR //fail, not defined! https://github.com/lukaslueg/built/issues/10
       );

    println!("authors {}", built_info::PKG_AUTHORS);
}
