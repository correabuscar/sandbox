extern crate reed_solomon;

use reed_solomon::Encoder;
use reed_solomon::Decoder;

//macro_rules! create_array {
//    ($n:expr) => {
//        concat!("[", (0..=$n).map(|x| x.to_string()).collect::<Vec<_>>().join(","), "]")
//    };
//}
//
//const SOME_CONST: usize = 7;
//
//
const ECC_SIZE: usize = 8;

fn main() {
//let my_array: [i32; SOME_CONST + 1] = [i32::from_str_radix(&create_array!(SOME_CONST).as_str()[1..], 10).unwrap()];

//println!("{:?}", my_array); // Output: [0, 1, 2, 3, 4, 5, 6, 7]

    let data = b"The quick brown fox jumps over the lazy dog";

    // Length of error correction code
    let ecc_len = ECC_SIZE;

    // Create encoder and decoder with 
    let enc = Encoder::new(ecc_len);
    let dec = Decoder::new(ecc_len);

    // Encode data
    let encoded = enc.encode(&data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    let from=2; // or 0
    for i in from..ecc_len+from {
        let random_u8: u8 = rand::random();
        //corrupted[i] = 0x0;
        corrupted[i] = random_u8;
    }

    // Try to recover data
    //let known_erasures = [0,1,2,3,4,5,6,7];
    let known_erasures: [u8; ECC_SIZE] = {
		let mut idx=from;
		[0; ECC_SIZE].map(|_| {
            let val=idx;
            idx+=1;
            val as u8
            })
	};
    println!("{:?}",known_erasures);
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
    let in_unicode = Cow::borrow_from_cp437(data, &CP437_CONTROL);
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
