#![allow(dead_code)]

mod client; //aka client.rs

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

mod network2;
mod network3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
