// src: https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html#try_join

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

use futures::future::TryFutureExt; //for map_err
use futures::try_join; // need features: 'nightly' which is on futures-preview crate  v0.3.0-alpha.17 not on .19 !  maybe also needs feature 'async-await' ?

use futures_timer::Delay;
#[allow(unused)]
use std::thread;
use std::time;

//async fn Book() -> i32 {
//    return 1;
//}
//struct Music();
//struct Book(); //bad!
#[derive(Debug)]
struct Book; //good
#[derive(Debug)]
struct Music; //good

async fn get_book() -> Result<Book, ()> {
    /* ... */
    println!("in get_book");
    //static mut a: bool=false;
    //if !a {
    //    a=true;
    //    //return Err("blah");
    //}
    //thread::sleep(time::Duration::from_secs(2)); //this blocks get_music's run too! so bad!
    let now = Delay::new(time::Duration::from_secs(2)).await; //this doesn't; so good!
    println!("in get_book done sleep '{:?}'", now);
    return Ok(Book);
}
async fn get_music() -> Result<Music, String> {
    /* ... */
    println!("in get_music");
    //thread::sleep(time::Duration::from_secs(1));//bad
    let now = Delay::new(time::Duration::from_secs(1)).await; //this doesn't; so good!
    println!("in get_music done sleep '{:?}'", now);
    Ok(Music)
}

async fn get_book_and_music() -> Result<(Book, Music), String> {
    println!("in get_book_and_music before");
    let book_fut = get_book().map_err(|()| "Unable to get boot".to_string());
    let music_fut = get_music();
    let r = try_join!(book_fut, music_fut);
    println!("in get_book_and_music after");
    r
}

fn main() {
    println!("Hello, world!");
    println!("Bye, world! {:#?}", block_on(get_book_and_music()));
}
