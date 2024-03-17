#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_export]
macro_rules! fflush {
    () => ({
        fflush!(stdout);
        fflush!(stderr);
    });
    ($stdwhat:ident) => ({
        use std::io::Write; //XXX: needed for flush() to be seen in scope!
        std::io::$stdwhat().flush().ok().expect(stringify!(Could not flush $stdwhat));
//XXX: how to place $name into str; find a better way?
    });
}
