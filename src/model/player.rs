use model;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
  pub id: String,
  pub name: String
}

impl Player {
  pub fn new(name: String) -> Self {
    let id = model::create_id(&name);
    Player { id, name }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_user() {
    let p = Player::new("Alex".to_owned());

    assert_eq!(p.name, "Alex".to_owned());
  }
}
