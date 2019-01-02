#![feature(nll)] //only needed when not passing: -Z nll or -Z borrowck=mir
//actually "-Z nll" is no longer a thing! but the mir one works!
//must allow the feature in code here, or how else am I gonna run `cargo clippy`?

fn main() {}

struct Static<T: 'static>(&'static [T]);

static FOO: Static<fn(&u32) -> &u32> = Static(&[|x| x]);
