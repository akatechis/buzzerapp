extern crate websocket;
extern crate hyper;
extern crate futures;

mod server;

fn main() {
  let server = server::start("127.0.0.1:3000");
}
