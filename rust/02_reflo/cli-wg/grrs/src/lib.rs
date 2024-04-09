pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap_or_else(|e|
                panic!("Error occurred while writing: '{}'", e));

            //if let Err(e) = writeln!(writer, "{}", line) {
            //    panic!("writeln! failed with error: '{}'",e);
            //}

            //match writeln!(writer, "{}", line) {
            //    Err(e) => {
            //        panic!("writeln! failed with error: '{}'",e);
            //    },
            //    _ => {}
            //};
        };
    };
}
