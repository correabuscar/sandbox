use std::collections::HashSet;

type Position = (usize, usize);

const SIZE: usize = 19;

fn is_valid_position(p: &Position) -> bool {
    let x = p.0;
    let y = p.1;
    x < SIZE && y < SIZE
}

fn get_neighbours(x: usize, y: usize) -> HashSet<Position> {
    let mut possible: HashSet<Position> = HashSet::new();
    possible.insert((x.wrapping_sub(1), y));
    possible.insert((x + 1, y));
    possible.insert((x, y.wrapping_sub(1)));
    possible.insert((x, y + 1));
    //works:
    return possible.drain().filter(is_valid_position).collect();
    //fails:
    //possible.drain().filter(is_valid_position).collect()
        //^error[E0597]: `possible` does not live long enough
        //XXX: https://github.com/rust-lang/rust/issues/43837
}

fn main() {
    println!("{:?}", get_neighbours(0, 1))
}

