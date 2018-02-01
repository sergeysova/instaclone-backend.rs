extern crate hyper;
extern crate futures;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use futures::future::Future;

use hyper::header::{ContentLength};
use hyper::server::{Http, Request, Response, Service};

#[derive(Debug, Serialize, Deserialize)]
struct HelloResponse {
    name: String,
}

impl HelloResponse {
    fn new(name: String) -> HelloResponse {
        HelloResponse { name }
    }
}


struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let resp = HelloResponse::new("Sova".into());
        let resp_string = serde_json::to_string(&resp).unwrap();

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(resp_string.len() as u64))
                .with_body(resp_string)
        ))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();

    println!("Listening {address}...", address=addr);
    server.run().unwrap();
}
