#![feature(internal_output_capture)]
#![allow(internal_features)]
#![feature(print_internals)]

#[allow(unused_imports)]
use std::io::set_output_capture;
use std::io::_print;
#[allow(unused_imports)]
use std::io::_eprint;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    println!("Hello, world!");
    let prev=std::io::set_output_capture(None);
    assert!(prev.is_none());
    // Create a channel to capture stdout
    //let (tx, rx) = mpsc::channel();

    // Set up output capture
    //let _output_captured = set_output_capture(Some(std::io::LocalStream::new(tx)));

    // Create another thread that prints from 11 to 20
    let thread0 = thread::spawn(|| {
        //thread::sleep(std::time::Duration::from_millis(1000));
        //let data = Arc::new(Mutex::new(Vec::new()));
        //let _prev=std::io::set_output_capture(Some(data));
        for i in 11..=20 {
            _print(format_args!("Thread 0: {}\n", i));
            thread::sleep(std::time::Duration::from_millis(200));
        }
        /*thread::sleep(std::time::Duration::from_millis(2000));
        let prev=std::io::set_output_capture(None);
        assert!(prev.is_some());
        let captured=prev.unwrap();
        _print(format_args!("Captured the following:\n"));
        // Convert captured data to a String
        let captured_string = {
            let captured_mutex = captured.lock().unwrap();
            String::from_utf8_lossy(&captured_mutex[..]).into_owned()
        };

        // Print the captured output as a string
        //print!("{}", captured_string);//works too
        _print(format_args!("{}", captured_string));
        */
    });

    // Create a thread that prints from 1 to 5
    let thread1 = thread::spawn(|| {
        let data = Arc::new(Mutex::new(Vec::new()));
        //let _prev=std::io::set_output_capture(Some(Default::default()));
        //let _prev=std::io::set_output_capture(Some(data));
        //let _prev=std::io::set_output_capture2(Some(data),std::time::Duration::from_secs(3));
        let _prev=std::io::set_output_capture2(Some(data),std::time::Duration::from_millis(300));
        //assert_eq!(None, prev);
        for i in 1..=5 {
            _print(format_args!("Thread 1: {}\n", i));
            //thread::sleep(std::time::Duration::from_millis(100));
        }
        thread::sleep(std::time::Duration::from_millis(2000));
        let prev=std::io::set_output_capture(None);
        assert!(prev.is_some());
        let captured=prev.unwrap();
        _print(format_args!("Captured the following:\n"));
        // Convert captured data to a String
        let captured_string = {
            let captured_mutex = captured.lock().unwrap();
            String::from_utf8_lossy(&captured_mutex[..]).into_owned()
        };

        // Print the captured output as a string
        //print!("{}", captured_string);//works too
        _print(format_args!("{}", captured_string));
    });

    // Create another thread that prints from 6 to 10
    let thread2 = thread::spawn(|| {
        let data = Arc::new(Mutex::new(Vec::new()));
        //let _prev=std::io::set_output_capture(Some(data));
        let _prev=std::io::set_output_capture2(Some(data),std::time::Duration::from_millis(300));
        for i in 6..=10 {
            _print(format_args!("Thread 2: {}\n", i));
            //thread::sleep(std::time::Duration::from_millis(100));
        }
        thread::sleep(std::time::Duration::from_millis(2000));
        let prev=std::io::set_output_capture(None);
        assert!(prev.is_some());
        let captured=prev.unwrap();
        _print(format_args!("Captured the following:\n"));
        // Convert captured data to a String
        let captured_string = {
            let captured_mutex = captured.lock().unwrap();
            String::from_utf8_lossy(&captured_mutex[..]).into_owned()
        };

        // Print the captured output as a string
        //print!("{}", captured_string);//works too
        _print(format_args!("{}", captured_string));
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread0.join().unwrap();
}
