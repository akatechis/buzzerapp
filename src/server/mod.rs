use futures::future::Future;
use hyper;
use hyper::{Body, Method, StatusCode};
use futures;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

struct BuzzerServer;

const PHRASE: &'static str = "Hello, World!";

impl Service for BuzzerServer {
    type Request = Request<Body>;
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

         match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Try POSTing data to /echo");
            },
            (&Method::Post, "/echo") => {
                // we'll be back
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}

pub fn start(addr: &str) {
  let addr = addr.parse().unwrap();
  let server = Http::new().bind(&addr, || Ok(BuzzerServer)).unwrap();
  server.run().unwrap();
}
