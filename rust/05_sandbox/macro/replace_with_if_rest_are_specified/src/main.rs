macro_rules! replace_with_if_exists {
    ($mandatory:expr $(, $optional:expr $(,)? )+) => {
        $mandatory
    };
    ($mandatory:expr $(,)?) => {
        //empty
    };
}

  /// just puts first arg
  #[macro_export]
  macro_rules! replace_with {
      (
          $mandatory:tt $(, $optional:tt)* $(,)?
       ) => {
          $mandatory
      };
  }

  /// just puts first arg, but has different first arg separator: |
  #[macro_export]
  macro_rules! replace_with2 {
      (
          $mandatory:tt | $($optional:tt),* $(,)?
       ) => {
          $mandatory
      };
  }

fn main() {
    println!("Hello, world!");
    println!("{:?}",
        replace_with_if_exists!("foo", 1));
    println!("{:?}",
        replace_with_if_exists!("foo", 1,));
    println!("{:?}",
        replace_with_if_exists!("foo", 1,2));
    println!("{:?}",
        replace_with_if_exists!("foo", 1,2,));
//    println!("{:?}",
//        replace_with_if_exists!("foo",)); //XXX: can't use in println!() since it expands to nothing!
    replace_with_if_exists!("foo",);
}
