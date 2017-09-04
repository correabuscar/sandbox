//Contructing Magic Squares (Odd Order! - because Even Order are too complicated to contruct or something)
//Odd Order meaning x=y=odd number
//algo source: https://www.youtube.com/watch?v=-Tbd3dzlRnY&t=13m49s
fn main() {
    println!("Hello, world!");

    let n:usize=5; // must be odd
    //start point is top center
    let mut x:usize=(n-1)/2;
    //println!("{}",x);//2
    let mut y:usize=0;
    let to_be_filled=n*n;
    let mut filled=1;
    //square from https://stackoverflow.com/a/27984550
    //let mut square = [[0_u32; n]; n];//can't use non-constant!

    //dynamic 2d array: https://stackoverflow.com/a/36376568
    //base 1d array:
    let mut grid_raw = vec![0; n*n];
    //vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(n).collect();//jesus man
    // final 2d array:
    let square: &mut [&mut [_]] = grid_base.as_mut_slice();
    //
    square[x][y]=filled;
    while filled < to_be_filled {
        filled+=1;
        //rule 1: go up+right or wrap
        let oldx=x;
        let oldy=y;
        x=(x+1) % n;
        y=(n+y-1) % n;
        //rule 2: if rule1 lands on already filled value, cancel and move down instead
        if square[x][y] != 0 {
            x=oldx;
            y=(oldy+1)%n;
        }
        //
        square[x][y]=filled;
    }

    for y in 0..n {
        for x in 0..n {
            print!("{:3} ",square[x][y]);
        }
        println!();
    }
}
