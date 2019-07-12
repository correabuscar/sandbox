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

use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};

fn main() {
    //    ctrlc::set_handler(move || {
    //        println!("received Ctrl+C!");
    //    })
    //    .expect("Error setting Ctrl-C handler");
    let signals = Signals::new(&[SIGINT, SIGTERM, SIGHUP, SIGQUIT]).unwrap();

    let addr = "127.0.0.1:7878".parse().unwrap();
    //let listener = TcpListener::bind(&addr).unwrap();
    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();
    println!("HTTP server is listening on {}", addr);

    #[cfg(feature = "https")]
    macro_rules! a_block_that_does_not_restrict_scope {
        //FIXME: this still does!
        () => {
            let addr_https = "127.0.0.1:8443".parse().unwrap();
            let server_https = TcpListener::bind(&addr_https).unwrap();
            println!("HTTPS server is listening on {}", addr_https);
        };
    }
    #[cfg(feature = "https")]
    a_block_that_does_not_restrict_scope! {}

    /*    #[cfg(feature = "https")]
    {
        let addr_https = "127.0.0.1:8443".parse().unwrap();
        let server_https = TcpListener::bind(&addr_https).unwrap();
        println!("HTTPS server is listening on {}", addr_https);
    } this block restricts scope so the ^ vars are undefined outside of it */

    let pool = ThreadPool::new(4);

    // Setup some tokens to allow us to identify which event is
    // for which socket.
    const SERVER: Token = Token(0);
    const SIGNALS: Token = Token(1);
    #[cfg(feature = "https")]
    const SERVER_HTTPS: Token = Token(2);

    // Create a poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();
    #[cfg(feature = "https")]
    poll.register(
        &server_https,
        SERVER_HTTPS,
        Ready::readable(),
        PollOpt::edge(),
    )
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
        println!(".");
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
                /*                random_inexistent_identifier => {
                    //okFIXME: what?! how come this works!
                    // crate-wide #![allow_unused] would hide any warnings here! very dangerous!
                    // but using a new(ie. random_inexistent_identifier) variable here is how 'match' works: matches anything and bind this new var. to it.
                    // I didn't expect it in this context.
                }*/
                #[cfg(feature = "https")]
                SERVER_HTTPS => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let stream = server.accept_std().unwrap();
                    pool.execute(|| {
                        handle_connection(stream.0);
                    });
                }
                SIGNALS => {
                    //#[allow(clippy::never_loop)]  //this is not needed anymore, lookslike
                    //for sig in &signals {
                    for sig in signals.forever() {
                        //same thing!
                        println!("Received signal {:?}", sig);
                        match sig {
                            SIGINT | SIGTERM => break 'loopy,
                            _ => {
                                //FIXME: pkill -1 l_20_1_hello will cause epic 100% cpu usage from then on!
                                // SIGHUP and SIGQUIT must be already in the list of &signals else they won't be caught here!
                                println!("Ignored signal '{:?}'", sig);
                                //signals.close(); //this prevents 100% cpu usage but also stops everything from being handled!
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
    #[cfg(feature = "sleepbeforeexitmain")]
    {
        println!("...but first: sleeping 5 sec before exiting main()");
        thread::sleep(Duration::from_secs(5));
        println!("done sleeping, exiting!");
    }
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
