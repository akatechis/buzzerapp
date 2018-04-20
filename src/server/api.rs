use futures;
use futures::future::Future;
use hyper;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

pub struct Configuration {
  pub database_url: String,
  pub buzzer_url: String,
}

pub struct APIService;

impl Service for APIService {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  // The future representing the eventual Response your call will
  // resolve to. This can change to whatever Future you need.
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    println!("Got a request!");
    println!("{:?}", req);
    Box::new(futures::future::ok(
      Response::new()
        .with_header(ContentLength(12))
        .with_body("Hello, world")
    ))
  }
}
