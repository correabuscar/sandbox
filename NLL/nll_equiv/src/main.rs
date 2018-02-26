//#![feature(nll)] //only needed when not passing: -Z nll or -Z borrowck=mir

fn main() {}

struct Static<T: 'static>(&'static [T]);

static FOO: Static<fn(&u32) -> &u32> = Static(&[|x| x]);
