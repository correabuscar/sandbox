use {
    futures::{
        // Extension trait for futures 0.1 futures, adding the `.compat()` method
        // which allows us to use `.await` on 0.1 futures.
        compat::Future01CompatExt,
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
    },
    hyper::{
        // A function which runs a future to completion using the Hyper runtime.
        rt::run,
        // This function turns a closure which returns a future into an
        // implementation of the the Hyper `Service` trait, which is an
        // asynchronous function from a generic `Request` to a `Response`.
        service::service_fn,

        // Miscellaneous types from Hyper for working with HTTP.
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request {:#?}", _req);
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    //Ok(Response::new(Body::from("hello, world!")))
    //let url_str = "http://www.rust-lang.org/en-US/"; //must be http or else 'connection was reset'! but when it's http then it just redirects there! so it doesn't remain on http://127.0.0.1:3000
    let url_str = "http://www.rust-lang.org/en-US/";
    //let url_str = "http://test.com"; //302 https://www.test.com
    //let url_str = "http://raw.githubusercontent.com/tikv/client-rust/master/examples/raw.rs"; //same, has to be http for this to work!
    //let url_str = "https://raw.githubusercontent.com/tikv/client-rust/master/examples/raw.rs"; //Error not https hmm
    let url = url_str.parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).compat().await;
    // Return the result of the request directly to the user
    println!("request finished-- returning response");
    println!("Returning this {:#?}", res);
    //let (mut parts, body) = res.unwrap().into_parts();
    //parts.headers().insert("location","https://github.com".parse().unwrap());
    let mut messing = res.unwrap();
    //messing.headers.location = "https://github.com";
    //let headers: &mut HeaderMap<HeaderValue> = messing
    let headers = messing.headers_mut();
    headers
        //.insert("location", "https://github.com".parse().unwrap())
        .remove("location");
    //messing.status(200);
    //res //fail, connection was reset!
    Ok(messing)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // Create a server bound on the provided address
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async serve_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response. To use our `serve_req` function with
        // Hyper, we have to box it and put it in a compatability
        // wrapper to go from a futures 0.3 future (the kind returned by
        // `async fn`) to a futures 0.1 future (the kind used by Hyper).
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    // Set the address to run our socket on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Call our `run_server` function, which returns a future.
    // As with every `async fn`, for `run_server` to do anything,
    // the returned future needs to be run. Additionally,
    // we need to convert the returned future from a futures 0.3 future into a
    // futures 0.1 future.
    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    // Finally, we can run the future to completion using the `run` function
    // provided by Hyper.
    run(futures_01_future);
}
