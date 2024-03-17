use libxid;

fn main() {
    // initialize it once, reuse it afterwards
    let g = libxid::new_generator();

    for _ in 0..10 {
        let id = g.new_id().unwrap();

        println!(
            "id: {:?} encoded: {:?}    machine: {:?}    counter: {:?}    time: {:?}",
            id,
            id.encode(),
            id.machine(),
            id.counter(),
            id.time()
        );
    }
}
