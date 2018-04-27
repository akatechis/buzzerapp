use futures;
use futures::future::Future;
use hyper;
use hyper::Method;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

pub struct APIService;

type APIResult = Result<Response, hyper::Error>;
type Route<'a> = (&'a Method, &'a str);

impl APIService {
  fn handle_api_request(&self, &(method, path): &Route, req: &Request) -> APIResult {
    Ok(Response::new()
      .with_header(ContentLength(15))
      .with_body("Hello, from API"))
  }
}

impl Service for APIService {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let route = (req.method(), req.path());

    let resp = self.handle_api_request(&route, &req);

    Box::new(futures::future::result(resp))
  }
}
