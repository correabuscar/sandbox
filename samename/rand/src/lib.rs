

#[cfg(test)]
mod tests {
    extern crate rand;
    #[test]
    fn it_works() {
        let tuple = rand::random::<(f64, char)>();
        println!("{:?}", tuple)

    }
}
