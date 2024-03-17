use rand::prelude::*;

const HOW_MANY_HEX_GROUPS:u8 = 6;

fn main() {
    let mut rng = rand::thread_rng();
    for i in 1..=HOW_MANY_HEX_GROUPS {
        let y: u8 = rng.gen::<u8>() & if i==1 {0xFC_u8} else {0xFF_u8};
        //^ first two LSB bits should be zero, see: https://en.wikipedia.org/wiki/MAC_address#Unicast_vs._multicast and https://en.wikipedia.org/wiki/MAC_address#Universal_vs._local
        let colon=if HOW_MANY_HEX_GROUPS != i {":"} else {""};
        print!("{:02X}{}", y, colon);
    }
}
