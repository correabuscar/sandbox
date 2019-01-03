#![allow(unused)]

//https://github.com/rust-lang/rust/issues/57307

pub trait Man {
    fn manual_is_ascii_punctuation(self) -> bool;
}

impl Man for char {
#[inline]
    fn manual_is_ascii_punctuation(self) -> bool {
        self.is_ascii() && (self as u8).is_ascii_punctuation()
        //    match self {
        //        '0'..='9' => true,
        //        c if c > '\x7f' => general_category::N(c),
        //        _ => false,
        //    }
    }
}

fn main() {
    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
    assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar"); //ok
    assert_eq!(
        ".,\"foo1bar\".,';".trim_matches(char::manual_is_ascii_punctuation),
        "foo1bar"
    ); //works because the func sig matches
    assert_eq!(
        ".,\"foo1bar\".,';".trim_matches(char::is_ascii_punctuation),
        "foo1bar"
    ); //XXX fail
    // expected signature of `fn(char) -> _`
    // found signature of `for<'r> fn(&'r char) -> _`
    //   = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `for<'r> fn(&'r char) -> bool {std::char::methods::<impl char>::is_ascii_punctuation}`

    assert_eq!("\"foo1bar\"".trim_matches(|x| x == '"'), "foo1bar"); //ok
    assert!('"'.is_ascii_punctuation());

    let x: &[_] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
}

