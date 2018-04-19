use std::path::Path;
use model::Player;
use rusqlite::{Connection, Error};

pub struct DB {
  connection: Connection
}

impl DB {
  pub fn in_memory() -> Result<DB, Error> {
    Connection::open_in_memory().map(|connection| 
      DB { connection })
  }

  pub fn from_file<P>(path: P) -> Result<DB, Error> 
    where P: AsRef<Path> {
    Connection::open(path).map(|connection|
      DB { connection })
  }

  pub fn insert_player(&self, player: &Player) -> Result<(), Error> {
    let stmt_txt = "insert into `players` values(null, :name)";
    let mut stmt = try!(self.connection.prepare(stmt_txt));
    stmt.execute_named(&[ (":id", &player.id), (":name", &player.name) ])
    .map(|_|())
  }

  pub fn update_player(&self, player: &Player) -> Result<(), Error> {
    let stmt_txt = "update `players` set name=:name where id=:id";
    let mut stmt = try!(self.connection.prepare(stmt_txt));
    stmt.execute_named(&[ (":id", &player.id),(":name", &player.name) ])
    .map(|_|())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_player_registration() {
    // CREATE TABLE `players` ( `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE, `name` TEXT NOT NULL )
    let db = DB::in_memory().unwrap();
    let player = Player::new("Alex".to_owned());
    let result = db.insert_player(&player);

    println!("{:?}", result);
    assert_eq!(result.is_ok(), true);
  }
}
