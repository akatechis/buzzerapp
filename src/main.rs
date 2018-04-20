extern crate ws;
extern crate env_logger;
extern crate md5;
extern crate dotenv;
extern crate serde_json;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate futures;

use hyper::server::Http;
use dotenv::dotenv;
use std::env;
use server::api::*;

mod server;
mod model;
mod db;

fn create_config() -> Configuration {
  Configuration {
    database_url: env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set"),

    buzzer_url: env::var("BUZZER_URL")
      .expect("BUZZER_URL must be set")
  }
}

fn main() {
  dotenv().ok();
  env_logger::init();

  let config = create_config();
  let addr = config.database_url.parse().unwrap();

  let server = Http::new()
  .bind(&addr, || Ok(server::api::APIService))
  .unwrap();
  server.run().unwrap();

  // if let Err(e) = server::start(config) {
  //   println!("Error starting server: {}", e);
  // }
}
