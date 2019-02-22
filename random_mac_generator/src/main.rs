use rand::prelude::*;

const HOW_MANY_HEX_GROUPS:u8 = 6;

fn main() {
    let mut rng = rand::thread_rng();
    let y: u8 = rng.gen::<u8>() & 0xFC_u8;
    //^ first two LSB bits should be zero, see: https://en.wikipedia.org/wiki/MAC_address#Unicast_vs._multicast and https://en.wikipedia.org/wiki/MAC_address#Universal_vs._local
    print!("{:02X}:", y);
    for i in 2..=HOW_MANY_HEX_GROUPS {
        let y: i8 = rng.gen();
        let colon=if HOW_MANY_HEX_GROUPS != i {":"} else {""};
        print!("{:02X}{}", y, colon);
    }
}
