use std::convert::TryInto;

fn main() {
    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset((mid as isize).try_into().unwrap()), len - mid),
            )
        }
    }
    let mut a = vec![1, 2, 3, 4];
    println!("{:?}",split_at_mut(&mut a, 2));
    //println!("{:?}",split_at_mut(&mut a, 5));
}
