use model;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
  pub id: String,
  pub name: String,
  pub username: String,
  pub password: String,
}

impl Player {
  pub fn new(name: String, user: String, pass: String) -> Self {
    let id = model::create_id("player");
    Player { id, name, username: user, password: pass }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_user() {
    let p = Player::new(
      "Alex".to_owned(),
      "alex@pefin.com".to_owned(),
      "asdf1234".to_owned()
    );

    assert_eq!(p.name, "Alex".to_owned());
    assert_eq!(p.username, "alex@pefin.com".to_owned());
    assert_eq!(p.password, "asdf1234".to_owned());
    assert_eq!(p.id.len(), 32);
  }
}
