use model;

/// Contains all state related to a game
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  /// The host's client token
  pub host: String,

  /// A list of all clients participating
  pub players: Vec<model::Player>,

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
}
