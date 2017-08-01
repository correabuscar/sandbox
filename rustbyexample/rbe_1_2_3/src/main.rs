fn main() {
    //pad with zeroes afterwards AND truncate(well,round) the number!
    let formatted_number = format!("{num:0<width$.trunc$}", num=1.49462123, trunc=3,width=12); //thanks to cbreeden on #rust-beginners
    //shows: 1.4950000000

    println!("{}", formatted_number)
}
