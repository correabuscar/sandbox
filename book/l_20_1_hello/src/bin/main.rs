#![allow(clippy::unused_io_amount)]

use std::fs;
use std::io::prelude::*;
//use std::net::TcpListener;
//use std::net::TcpStream;
use mio::net::TcpListener;
use mio::*;
use std::thread;
use std::time::Duration;

use l_20_1_hello::ThreadPool;

use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

fn main() {
    //    ctrlc::set_handler(move || {
    //        println!("received Ctrl+C!");
    //    })
    //    .expect("Error setting Ctrl-C handler");
    let signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();

    let addr = "127.0.0.1:7878".parse().unwrap();
    //let listener = TcpListener::bind(&addr).unwrap();
    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    let pool = ThreadPool::new(4);

    // Setup some tokens to allow us to identify which event is
    // for which socket.
    const SERVER: Token = Token(0);
    const SIGNALS: Token = Token(1);

    // Create a poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();
    poll.register(&signals, SIGNALS, Ready::readable(), PollOpt::edge())
        .unwrap();
    // Create storage for events
    let mut events = Events::with_capacity(1024);

    //    'streem: for stream in listener.incoming()
    //    /*.take(2) */
    //    {
    //        let stream = stream.unwrap();
    //
    //        pool.execute(|| {
    //            handle_connection(stream);
    //        });
    //        //this won't check until after a connection:
    //        for sig in &signals {
    //            println!("Received signal {:?}", sig);
    //            match sig {
    //                //SIGINT => break 'streem,
    //                _ => {
    //                    println!("not-Ignored signal '{:?}'", sig);
    //                    break 'streem;
    //                }
    //            }
    //        }
    //    }

    'loopy: loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let stream = server.accept_std().unwrap();
                    pool.execute(|| {
                        handle_connection(stream.0);
                    });
                }
                SIGNALS => {
                    #[allow(clippy::never_loop)]
                    for sig in &signals {
                        println!("Received signal {:?}", sig);
                        match sig {
                            SIGINT | SIGTERM => break 'loopy,
                            _ => {
                                println!("Ignored signal '{:?}'", sig);
                                //break 'loopy;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        //        #[allow(clippy::never_loop)]
        //        for sig in &signals {
        //            println!("Received signal {:?}", sig);
        //            match sig {
        //                //SIGINT => break 'streem,
        //                _ => {
        //                    println!("not-Ignored signal '{:?}'", sig);
        //                    break 'loopy;
        //                }
        //            }
        //        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request:\n{}\n-----", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
