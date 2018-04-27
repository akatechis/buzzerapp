#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate ws;
extern crate env_logger;
extern crate md5;
extern crate dotenv;
extern crate serde_json;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

use std::env;
use dotenv::dotenv;

mod model;
mod db;

pub struct Configuration {
  pub database_url: String,
  pub service_url: String,
}

fn create_config() -> Configuration {
  Configuration {
    database_url: env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set"),

    service_url: env::var("SERVICE_URL")
      .expect("SERVICE_URL must be set"),
  }
}

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/favicon.ico")]
fn favicon() -> &'static str {
  "hahahaha"
}

fn main() {
  dotenv().ok();
  env_logger::init();
  rocket::ignite().mount("/", routes![index]).launch();
}
