use futures;
use futures::future::Future;
use hyper;
use hyper::mime;
use hyper::Method;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Request, Response, Service};

const INDEX_FILE: &'static str = include_str!("./assets/index.html");
const NOT_FOUND: &'static str = include_str!("./assets/404.html");

pub struct AssetService;

impl Service for AssetService {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let method = req.method();
    let path = req.path();

    let resp = match method {
      &Method::Get => {
        match path {
          "/" => respond_with_html(INDEX_FILE),
          "/index.html" => respond_with_html(INDEX_FILE),
          _ => respond_with_html(NOT_FOUND)
        }
      },
      _ => respond_with_html(NOT_FOUND)
    };

    Box::new(futures::future::ok(resp))
  }
}

fn respond_with_html(contents: &'static str) -> Response {
  respond_with_file_contents(contents, mime::TEXT_HTML)
}

fn respond_with_file_contents(contents: &'static str, mimetype: mime::Mime) -> Response {
  Response::new()
    .with_header(ContentType(mimetype))
    .with_header(ContentLength(contents.len() as u64))
    .with_body(contents)
}
