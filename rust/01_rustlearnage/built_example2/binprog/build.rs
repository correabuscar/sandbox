//extern crate built;
//pub fn write_built_file_with_opts(options: &Options) -> io::Result<()> {
//    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
//    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
//    built::write_built_file_with_opts(options, &src, &dst)?;
//    Ok(())
//}
fn main() {
//    let opts=built::Options::default() //temporary value does not live long enough
//        .set_dependencies(true);
    //let mut opts=built::Options::default();
    //opts.set_dependencies(true);//FIXME: thread 'main' panicked at 'Failed to acquire build-time information: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:945:5  // this most likely cannot find Cargo.lock file, because we're inside a workspace!
    //built::write_built_file_with_opts(&opts).expect("Failed to acquire build-time information");

	// Path to the Cargo.lock file relative to the build.rs file
	//let cargo_lock_path = "../Cargo.lock";

	//// Check if the Cargo.lock file exists
	//if let Ok(metadata) = std::fs::metadata(&cargo_lock_path) {
	//	// Check if the file is a regular file
	//	if metadata.is_file() {
	//		// Retrieve the current permissions
	//		if let Ok(mut permissions) = std::fs::metadata(&cargo_lock_path).map(|m| m.permissions()) {
	//			// Remove the read attribute from the file's permissions
	//			//permissions.set_readonly(false); //chatgpt 3.5 fail, that does a+w
    //            // Remove the read attribute from the file's permissions
    //            use std::os::unix::fs::PermissionsExt;
    //            permissions.set_mode(permissions.mode() & !0o444);

	//			// Set the new permissions for the file
	//			if let Err(err) = std::fs::set_permissions(&cargo_lock_path, permissions) {
	//				panic!("Failed to set permissions: {}", err);
	//			}
	//		} else {
	//			panic!("Failed to retrieve file permissions");
	//		}
	//	} else {
	//		panic!("{} is not a regular file", cargo_lock_path);
	//	}
	//} else {
	//	panic!("{} does not exist", cargo_lock_path);
	//}
    built::write_built_file().expect("Failed to acquire build-time information")
}
