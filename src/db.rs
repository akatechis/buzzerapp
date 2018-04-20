use model::Player;
use rusqlite::{Connection, Error};

pub struct DB {
  conn: Connection
}

impl DB {
  pub fn from_file(path: &str) -> Result<DB, Error> {
    Connection::open(path)
    .map(|conn| DB { conn })
  }

  pub fn insert_player(&self, player: &Player) -> Result<(), Error> {
    let qry = "insert into players values(:id, :name, :user, :pass)";
    let mut stmt = self.conn.prepare(qry)?;
    stmt.execute_named(&[
      (":id", &player.id),
      (":name", &player.name),
      (":user", &player.username),
      (":pass", &player.password),
    ])?;
    Ok(())
  }

  pub fn update_player(&self, player: &Player) -> Result<(), Error> {
    let qry = "update players set name=:name where id=:id";
    let mut stmt = self.conn.prepare(qry)?;
    stmt.execute_named(&[
      (":id", &player.id),
      (":name", &player.name),
    ])?;
    Ok(())
  }
}
