// see TODO here: https://gist.github.com/CExftNSroxORgpxED/c34b67bb43ef3ca6fbd75971c460f541
// prepend this https://web.archive.org/web/20191018200449/ tot he url and remove the CENSORED
fn main() {
//    let in_num:i32=1289346750; // 4CD9DEBE
//    println!("{:?}", 2*in_num);
    //let innum: i8 = 122;
    let in_num:i8=122;

    //let a = 2_i8.checked_mul(in_num);
    let a=0;
    //let b= 2_i8.overflowing_mul(in_num);
    let b=0;
    println!("{:?} / {:?}", a, b);
//    let _c:i8=2*in_num;
    //println!("{:?}", 2*in_num);
    let v=vec![1,2];
    let _e=v[in_num as usize];
}

