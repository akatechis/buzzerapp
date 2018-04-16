use md5;
use std::time::{SystemTime, UNIX_EPOCH};

type Identifier = String;

fn create_identifier(prefix: &str) -> String {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Clock fubar");
  let template = format!("{}-{:?}", prefix, timestamp);
  let digest = md5::compute(template.as_bytes());
  format!("{:x}", digest)
}

/// Contains all state related to a game
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  /// The host's client token
  pub host: Identifier,

  /// A list of all clients participating
  pub players: Vec<Player>,

  /// The currently buzzed player
  pub active_buzzer: Option<Identifier>
}

impl Game {
  pub fn new() -> Self {
    let players = vec![];
    let active_buzzer = None;
    let host = create_identifier("host");

    Game { host, players, active_buzzer }
  }

  pub fn player(&self, id: &Identifier) -> Option<&Player> {
    self.players.iter().find(|p| &p.id == id)
  }

  /// Buzz in the player with the given identifier
  /// Returns true if the game's state changed as a result
  pub fn buzz(&mut self, id: &Identifier) -> bool {
    let player_joined = self.players.iter().find(|p|
      &p.id == id).is_some();
    if player_joined && self.active_buzzer.is_none() {
      self.active_buzzer = Some(id.to_owned());
      true
    }
    else {
      false
    }
  }

  /// Clears the game buzzer
  /// Returns true if the game's state changed as a result
  pub fn clear_buzzer(&mut self, host: &Identifier) -> bool {
    let host_verified = &self.host == host;
    if host_verified && self.active_buzzer.is_some() {
      self.active_buzzer = None;
      true
    }
    else {
      false
    }
  }

  /// Adds the given player to the game
  /// Returns true if the game's state changed as a result
  pub fn join(&mut self, player: Player) -> bool {
    let exists = self.players.contains(&player);
    if !exists {
      self.players.push(player);
    }
    !exists
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
  pub id: Identifier,
  pub name: String
}

impl Player {
  pub fn new(name: String) -> Self {
    let id = create_identifier(&name);
    Player { id, name }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_game() {
    let game = Game::new();

    assert_eq!(game.active_buzzer, None);
    assert_eq!(game.players.len(), 0);
  }

  #[test]
  fn test_player_can_join_game() {
    let mut game = Game::new();
    let player = Player::new("Alex".to_owned());
    let player_id = player.id.to_owned();

    let joined = game.join(player);
    assert_eq!(joined, true);
    assert_eq!(game.players.len(), 1);

    let joined_player = game.player(&player_id).unwrap();
    assert_eq!(joined_player.id, player_id);
    assert_eq!(joined_player.name, "Alex");
  }

  #[test]
  fn test_player_must_join_before_buzzing_in() {
    let mut game = Game::new();
    let player = Player::new("Alex".to_owned());

    let buzzed = game.buzz(&player.id);
    assert_eq!(buzzed, false);
  }

  #[test]
  fn test_player_can_buzz_in_if_noone_is_buzzed_in() {
    let mut game = Game::new();
    let player = Player::new("Alex".to_owned());
    let player_id = player.id.clone();
    game.join(player);

    let buzzed = game.buzz(&player_id);
    assert_eq!(buzzed, true);
    assert_eq!(game.active_buzzer, Some(player_id));
  }

  #[test]
  fn test_player_cannot_buzz_in_if_someone_is_buzzed_in() {
    let mut game = Game::new();
    let p1 = Player::new("Alex".to_owned());
    let p1_id = p1.id.clone();
    game.join(p1);

    let p2 = Player::new("Alex".to_owned());
    let p2_id = p2.id.clone();
    game.join(p2);

    let p1_buzz = game.buzz(&p1_id);
    let p2_buzz = game.buzz(&p2_id);

    assert_eq!(p1_buzz, true);
    assert_eq!(p2_buzz, false);
    assert_eq!(game.active_buzzer, Some(p1_id));
  }

  #[test]
  fn test_clearing_the_buzzer_when_noone_is_buzzed_in() {
    let mut game = Game::new();
    let host_id = game.host.clone();

    // clearing when noone is buzzed in does nothing
    let clear1 = game.clear_buzzer(&host_id);
    assert_eq!(clear1, false);
    assert_eq!(game.active_buzzer, None);
  }

  #[test]
  fn test_clearing_the_buzzer_when_a_player_is_buzzed_in() {
    let mut game = Game::new();
    let host_id = game.host.clone();

    let p1 = Player::new("Alex".to_owned());
    let p1_id = p1.id.clone();
    game.join(p1);

    let p2 = Player::new("Alex".to_owned());
    let p2_id = p2.id.clone();
    game.join(p2);

    // clearing when noone is buzzed in does nothing
    let clear1 = game.clear_buzzer(&host_id);
    assert_eq!(clear1, false);
    assert_eq!(game.active_buzzer, None);

    // player 1 buzzes in
    game.buzz(&p1_id);
    assert_eq!(game.active_buzzer, Some(p1_id));

    // the host clears it
    let clear2 = game.clear_buzzer(&host_id);
    assert_eq!(clear2, true);

    // player 2 buzzes in
    game.buzz(&p2_id);
    assert_eq!(game.active_buzzer, Some(p2_id));

    // the host clears it again
    let clear3 = game.clear_buzzer(&host_id);
    assert_eq!(clear3, true);
  }
}
