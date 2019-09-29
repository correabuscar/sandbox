use futures::executor::block_on;

use futures::{
    future,
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};

async fn task_one() { /* ... */
}
async fn task_two() -> i32 {
    /* ... */
    return 1;
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
    () = t1 => println!("task one completed first"),
          a = t2 => println!("task two completed first '{}' ",a),
              }

    select! { // with _ compiler won't notify you if return type of task_two changed and you forgot to also change stuff here! (Bad)
    _ = t1 => println!("task one completed first"),
          _ = t2 => println!("task two completed first"),
              }
}

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
    }
    assert_eq!(total, 10);
}

fn main() {
    block_on(race_tasks());
    block_on(count());
}
