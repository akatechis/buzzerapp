use md5;
use std::borrow::Borrow;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod player;
mod game;

pub use self::player::Player;
pub use self::game::Game;

fn timestamp() -> Duration {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Clock fubar")
}

pub fn create_id<S>(prefix: S) -> String
  where S: Borrow<str>{
  let timestamp = timestamp();
  let template = format!("{}-{:?}", prefix.borrow(), timestamp);
  let digest = md5::compute(template.as_bytes());
  format!("{:x}", digest)
}
