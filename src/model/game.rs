use model;
use model::Player;

/// Contains all state related to a game
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  /// The host's client token
  pub host: String,

  /// A list of all clients participating
  pub players: Vec<Player>,

  /// The currently buzzed player
  pub active_buzzer: Option<String>
}

impl Game {
  pub fn new() -> Self {
    let players = vec![];
    let active_buzzer = None;
    let host = model::create_id("host");

    Game { host, players, active_buzzer }
  }

  pub fn player(&self, id: &String) -> Option<&Player> {
    self.players.iter().find(|p| &p.id == id)
  }

  /// Buzz in the player with the given identifier
  /// Returns true if the game's state changed as a result
  pub fn buzz(&mut self, id: &String) -> bool {
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
  pub fn clear_buzzer(&mut self, host: &String) -> bool {
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
