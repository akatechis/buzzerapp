extern crate ws;
extern crate env_logger;
extern crate md5;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod server;
mod data;

fn main() {
  env_logger::init();
  if let Err(e) = server::start("127.0.0.1:4000") {
    println!("Error starting socket server: {}", e);
  }
}
