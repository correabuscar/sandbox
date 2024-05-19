mod some {
    #[macro_export]
    macro_rules! foo {
        () => {
            self::bar!();
        }
    }
    #[macro_export]
    macro_rules! bar {
        () => {
            println!("Hi");
        }
    }
}
