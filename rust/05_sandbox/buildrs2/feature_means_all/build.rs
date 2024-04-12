fn main() {
    //XXX: not valid way, chatgpt 3.5 hallucination!
//    #[cfg(feature="feature_means_all::one")]
//    let a=1;
//    #[cfg(feature="one")]
//    let b=1;
//
//    #[cfg(any(feature="one", feature="feature_means_all::one"))]
//    assert_eq!(a,b);

    #[cfg(feature="one")]
    eprintln!("used feature one");
    #[cfg(feature="two")]
    eprintln!("used feature two");

    //#[cfg(feature="ideally-excluded-from-all")] // using only this won't play well with arg --all-features

    #[cfg(all(feature="ideally-excluded-from-all", not(feature="used-all-features-detector")))]
    panic!("fancy testing")

}
