extern crate reed_solomon;

use reed_solomon::Encoder;
use reed_solomon::Decoder;

const ECC_SIZE: usize = 22; // add this many Reed-Solomon codes after the original data, for
                           // protecting it from corruptions.

const MAX_INDEXGEN_TRIES: usize = 10; //shouldn't need to change it

//can't repair more than ECC_SIZE number of erased bytes if you tell it at which indexes they are,
//can repair half of ECC_SIZE number of erased bytes if you don't tell it where they are.
//if you do more, it says too many errors!
const CORRUPT_THIS_MANY_BYTES: usize = 11;

//if true, it can repair ECC_SIZE number of bytes (internally they're set to 0 before trying to
//repair the data)
//if false, it can only repair ECC_SIZE/2 number of bytes, so make sure CORRUPT_THIS_MANY_BYTES is
// <= than ECC_SIZE/2
const TELL_IT_WHICH_BYTES_ARE_CORRUPTED:bool = false;

fn main() {

    let data: &[u8] = 
        //b"The quick brown fox jumps over the lazy dog";
//b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. The quick brown foxy!!";
b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

    // Length of error correction code
    let ecc_len = ECC_SIZE;

    // Create encoder and decoder with 
    let enc = Encoder::new(ecc_len);
    let dec = Decoder::new(ecc_len);

    // Encode data
    let encoded = enc.encode(&data[..]);

    use rand::{thread_rng, Rng};
    let data_len=data.len();

//    let wanted_from=min(0,NOTHING_BELOW_INDEX);
//    let from=if wanted_from+ecc_len >= data_len {
//        data_len-ecc_len
//    }else{
//        wanted_from
//    };
//    let r=0..=data_len; //exclusive on the right hand side
//    assert!(&0 == r.start());
//    let a:bool=r.contains(&1); //this ends the iteration on 'r' !
//    assert!(&0 == r.start());//second call is fkd "When using an inclusive range for iteration, the
//                             //values of start() and end() are unspecified after the iteration
//                             //ended."
//    assert!((0..=ecc_len).contains(&from));
//    let to=max(ecc_len+from, data_len);
 //   //assert!(to.is_in(0..=ecc_len));
 //   panic!();

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    let mut index_rng = thread_rng(); // Initialize random number generator
    let mut data_rng = thread_rng(); // Initialize random number generator
    use std::collections::HashSet;
    let mut picked_indexes = HashSet::new();
    use std::cmp::min;
    let corrupt_how_many_bytes=min(ECC_SIZE, CORRUPT_THIS_MANY_BYTES);
    let picks_wanted=min(corrupt_how_many_bytes, data_len);
    // pick ECC_SIZE number of indexes, but data_len might be smaller than ECC_SIZE
    for picks in 1..=picks_wanted {
        let mut random_index: usize;
        let mut tries:usize=0; // note: not retries!
        'loop1: loop {
            random_index=index_rng.gen_range(0..data_len);//rightmost excluded
            if !picked_indexes.contains(&random_index) {
                //found an index we didn't already pick (before)
                break 'loop1;
            }
            println!("!! Caught a dup pick: '{}'. Retrying...", &random_index);
            tries+=1;
            if tries >= MAX_INDEXGEN_TRIES {
                panic!("!! Too many failed index tries: '{}'. Wanted '{}' indexes, got '{}' so far.", tries, picks_wanted, picks);
            };
            continue 'loop1; //too explicit?
        };//'loop1
        picked_indexes.insert(random_index);
        assert!(picked_indexes.contains(&random_index));
        corrupted[random_index] = data_rng.gen::<u8>();//making sure the type is the one I thought!
    };
    assert!(picked_indexes.len() == picks_wanted);

    //let from=2;
//    for i in from..max(data_len,ecc_len+from) {
//        let random_u8: u8 = rng.gen();//rand::random();
//        //corrupted[i] = 0x0;
//        corrupted[i] = random_u8;
//    }

    // Try to recover data
    //let known_erasures = [0,1,2,3,4,5,6,7];
//    let known_erasures: [u8; ECC_SIZE] = {
//		let mut idx=from;
//		[0; ECC_SIZE].map(|_| {
//            let val=idx;
//            idx+=1;
//            val as u8
//            })
//	};
    // Convert the contents of the HashSet<usize> into u8, panicking if usize doesn't fit
    let bytes: Vec<u8> = picked_indexes.iter().map(|&index| index as u8).collect();
    // Get a slice of the bytes
    let bytes_slice: &[u8] = &bytes;
    let known_erasures:&[u8] =if TELL_IT_WHICH_BYTES_ARE_CORRUPTED {
        bytes_slice
    }else {
        &[]
    };
    println!("known erasures at indexes(ie. these get zeroed internally):{:?}",known_erasures);
    //let known_erasures: [u8; ECC_SIZE] = {
    //    for i in 0..ECC_SIZE {

    //    };
    //};
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    let orig_str = std::str::from_utf8(data).unwrap();
    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    use codepage_437::{FromCp437, CP437_WINGDINGS};
    use std::borrow::Cow;
    use codepage_437::CP437_CONTROL;
    use codepage_437::BorrowFromCp437;
    // Input byte string encoded in CP437
    let cp437_bytes = data;
    //let data=vec![65];
    //let in_unicode = String::from_cp437(data, &CP437_CONTROL);
    // Convert CP437 bytes to UTF-8 string
    //let utf8_str = CP437_WINGDINGS.decode(cp437_bytes);
    //let in_cp437=
    // Convert each CP437 byte to UTF-8 string
    let utf8_str: String = cp437_bytes.iter()
        .map(|&b| CP437_WINGDINGS.decode(b))
        .collect();
    //use codepage_437::decode;
    //let utf8_str:String=decode(cp437_bytes);
    // Convert CP437 bytes to UTF-8 string
    //let utf8_str: String = cp437_bytes.iter().map(|&b| char::from_cp437(b).unwrap()).collect();
    //let in_unicode = Cow::borrow_from_cp437(data, &CP437_CONTROL);
    let ecc=encoded.ecc();
    let in_unicode = Cow::borrow_from_cp437(ecc, &CP437_CONTROL);
    let in_unicode=in_unicode.to_string();
    //use codepage_437::{CP437_CONTROL, FromCp437};
    let ecc_s=String::from_cp437(ecc.to_vec(), &CP437_WINGDINGS);
    let corrupted_s=String::from_cp437(corrupted.to_vec(), &CP437_WINGDINGS);
    let recv_str_s=String::from_cp437(recv_str.as_bytes().to_vec(), &CP437_WINGDINGS);


    println!("message:               {:?}", orig_str);
    println!("original data:         {:?}", data);
    println!("original data:         {:?}", utf8_str);
    println!("error correction code: {:?}", ecc);
    println!("error correction code: {:?}", in_unicode);
    println!("error correction code: {:?}", ecc_s);
    println!("corrupted:             {:?}", corrupted);
    println!("corrupted:             {:?}", corrupted_s);
    println!("repaired:              {:?}", recv_str);
    println!("repaired:              {:?}", recv_str_s);
}
