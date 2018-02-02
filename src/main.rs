extern crate hyper;
extern crate futures;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use futures::future::Future;
use futures::future;

use hyper::{StatusCode, Method};
use hyper::header::{ContentLength};
use hyper::server::{Http, Request, Response, Service};

#[derive(Debug, Serialize, Deserialize)]
struct HelloResponse {
    hello: String,
}

impl HelloResponse {
    fn new(name: String) -> HelloResponse {
        HelloResponse { hello: name }
    }
}

const JSON_NOT_FOUND: &'static str = r#"{ "error": "not_found" }"#;
const JSON_STATUS_OK: &'static str = r#"{ "status": "ok" }"#;

fn wrap_response(response: Response) -> Box<Future<Item=Response, Error=hyper::Error>> {
    Box::new(
        future::ok(response)
    )
}

fn create_text_response(answer: &str, status: Option<StatusCode>) -> Response {
    let mut response = Response::new()
        .with_header(ContentLength(answer.len() as u64))
        .with_body(String::from(answer));

    if let Some(status) = status {
        response = response.with_status(status);
    }

    response
}

struct Application;

impl Service for Application {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("{} {}", req.method(), req.path());
        match (req.method(), req.path()) {
            (&Method::Get, "/") => wrap_response(
                create_text_response(JSON_STATUS_OK, None)
            ),
            (&Method::Get, "/hello") => {
                let body = serde_json::to_string(&HelloResponse::new("World".into())).unwrap();

                wrap_response(
                    create_text_response(
                        &body.to_owned()[..],
                        Some(StatusCode::Created),
                    )
                )
            },
            _ => wrap_response(
                create_text_response(JSON_NOT_FOUND, Some(StatusCode::NotFound))
            ),
        }
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Application)).unwrap();

    println!("Listening {address}...", address=addr);
    server.run().unwrap();
}
