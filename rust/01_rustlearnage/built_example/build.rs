extern crate built;
//pub fn write_built_file_with_opts(options: &Options) -> io::Result<()> {
//    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
//    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
//    built::write_built_file_with_opts(options, &src, &dst)?;
//    Ok(())
//}
fn main() {
//    let opts=built::Options::default() //temporary value does not live long enough
//        .set_dependencies(true);
    let mut opts=built::Options::default();
    opts.set_dependencies(true);//FIXME: thread 'main' panicked at 'Failed to acquire build-time information: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:945:5  // this most likely cannot find Cargo.lock file, because we're inside a workspace!
    built::write_built_file_with_opts2(&opts).expect("Failed to acquire build-time information");
}
