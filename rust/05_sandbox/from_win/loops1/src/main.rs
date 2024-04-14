fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0_f64;

    #[allow(unused_labels)]
    let result =
        'dummy: loop {
            'foo: loop {
                counter += 1.2;

                if counter >= 10_f64 {
                    break 'dummy counter * 2.0;
                }
            }
        };

    println!("The result is {result}");
}