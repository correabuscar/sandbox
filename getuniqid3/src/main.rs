//note how this is checking for feature 'rand' even though we've only defined 'rand_num' in
//Cargo.toml  but rand_num is defined as depending on package 'rand' so this works.

#[cfg(feature = "uuid")]
#[cfg(feature = "rand")]
#[cfg(feature = "xid")]
compile_error!("Can't use more than one mutually exclusive features: uuid & rand & xid");
//Thanks stephaneyfx (from irc) for telling me which is the panic!() equivalent for compile time!

//new way, lifted from: https://github.com/rust-lang/rfcs/blob/master/text/1695-add-error-macro.md#motivation
#[cfg(all(feature = "uuid", feature = "rand"))]
compile_error!("Can't use both mutually exclusive features: uuid & rand");

#[cfg(not(any(feature = "uuid", feature = "rand", feature = "xid")))]
compile_error!("Must use at least one of the features: uuid, rand, xid");

#[cfg(feature = "xid")]
macro_rules! xid_as_block {
    () => {
        use libxid;
        const WHAT_TING: &'static str = "xid";
    };
}

#[cfg(feature = "uuid")]
macro_rules! uuid_as_block {
    () => {
        extern crate uuid;
        use uuid::Uuid;
        const WHAT_TING: &'static str = "uuid";
    };
}

#[cfg(feature = "rand")]
macro_rules! rand_as_block {
    () => {
        extern crate rand;
        use rand::Rng;
        const WHAT_TING: &'static str = "random number";
    };
}
//doneFIXME: find a way to use a block and only one cfg line here:
//#[cfg(feature = "uuid")]
//    extern crate uuid;
//#[cfg(feature = "uuid")]
//    use uuid::Uuid;
//#[cfg(feature = "uuid")]
//    const WHAT_TING: &'static str = "uuid";
//Thanks stephaneyfx (on irc) for the macro idea as a block replacement!
#[cfg(feature = "uuid")]
uuid_as_block! {}
//uuid_as_block!(); //or can use this variant, with "();" instead of "{}"

#[cfg(feature = "rand")]
rand_as_block! {}

#[cfg(feature = "xid")]
xid_as_block!();

use std::collections::HashMap;

fn main() {
    let n: u32 = 1000000;
    let mut all_so_far = HashMap::with_capacity(n as usize);
    let mut i: u32 = 1;

    #[cfg(feature = "rand")]
    let mut rng = rand::thread_rng();

    #[cfg(feature = "xid")]
    let g = libxid::new_generator();

    let mut dups = 0;
    //for i in 1..=n {
    loop {
        //XXX: had to use loop instead of 'for' so that I'd have access to |i| outside of loop!
        #[cfg(feature = "uuid")]
        let cur_ting = Uuid::new_v4();

        #[cfg(feature = "rand")]
        let cur_ting = rng.gen::<u32>();

        #[cfg(feature = "xid")]
        let cur_ting = g.new_id().unwrap().encode();

        if let Some(old_i) = all_so_far.insert(
            #[cfg(feature = "xid")]
            cur_ting.clone(),
            #[cfg(not(feature = "xid"))]
            cur_ting,
            i,
        ) {
            println!(
                "Duplicate {} detected:
            old: i='{}' {}='{}'
            cur: i='{}' {}='{}'",
                WHAT_TING, //all_so_far.get(cur_ting).expect("oldi")
                old_i,
                WHAT_TING,
                cur_ting,
                i,
                WHAT_TING,
                cur_ting
            );
            dups += 1;
        }
        if i >= n {
            break;
        }
        i += 1;
    }
    println!(
        "Done, generated {} {}s of which {} were unique and {} were dups",
        i,
        WHAT_TING,
        i - dups,
        dups
    );
}
