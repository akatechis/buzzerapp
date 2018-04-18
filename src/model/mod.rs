use md5;
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

pub fn create_id(prefix: &str) -> String {
  let timestamp = timestamp();
  let template = format!("{}-{:?}", prefix, timestamp);
  let digest = md5::compute(template.as_bytes());
  format!("{:x}", digest)
}





// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_create_game() {
//     let game = Game::new();

//     assert_eq!(game.active_buzzer, None);
//     assert_eq!(game.players.len(), 0);
//   }

//   #[test]
//   fn test_player_can_join_game() {
//     let mut game = Game::new();
//     let player = Player::new("Alex".to_owned());
//     let player_id = player.id.to_owned();

//     let joined = game.join(player);
//     assert_eq!(joined, true);
//     assert_eq!(game.players.len(), 1);

//     let joined_player = game.player(&player_id).unwrap();
//     assert_eq!(joined_player.id, player_id);
//     assert_eq!(joined_player.name, "Alex");
//   }

//   #[test]
//   fn test_player_must_join_before_buzzing_in() {
//     let mut game = Game::new();
//     let player = Player::new("Alex".to_owned());

//     let buzzed = game.buzz(&player.id);
//     assert_eq!(buzzed, false);
//   }

//   #[test]
//   fn test_player_can_buzz_in_if_noone_is_buzzed_in() {
//     let mut game = Game::new();
//     let player = Player::new("Alex".to_owned());
//     let player_id = player.id.clone();
//     game.join(player);

//     let buzzed = game.buzz(&player_id);
//     assert_eq!(buzzed, true);
//     assert_eq!(game.active_buzzer, Some(player_id));
//   }

//   #[test]
//   fn test_player_cannot_buzz_in_if_someone_is_buzzed_in() {
//     let mut game = Game::new();
//     let p1 = Player::new("Alex".to_owned());
//     let p1_id = p1.id.clone();
//     game.join(p1);

//     let p2 = Player::new("Alex".to_owned());
//     let p2_id = p2.id.clone();
//     game.join(p2);

//     let p1_buzz = game.buzz(&p1_id);
//     let p2_buzz = game.buzz(&p2_id);

//     assert_eq!(p1_buzz, true);
//     assert_eq!(p2_buzz, false);
//     assert_eq!(game.active_buzzer, Some(p1_id));
//   }

//   #[test]
//   fn test_clearing_the_buzzer_when_noone_is_buzzed_in() {
//     let mut game = Game::new();
//     let host_id = game.host.clone();

//     // clearing when noone is buzzed in does nothing
//     let clear1 = game.clear_buzzer(&host_id);
//     assert_eq!(clear1, false);
//     assert_eq!(game.active_buzzer, None);
//   }

//   #[test]
//   fn test_clearing_the_buzzer_when_a_player_is_buzzed_in() {
//     let mut game = Game::new();
//     let host_id = game.host.clone();

//     let p1 = Player::new("Alex".to_owned());
//     let p1_id = p1.id.clone();
//     game.join(p1);

//     let p2 = Player::new("Alex".to_owned());
//     let p2_id = p2.id.clone();
//     game.join(p2);

//     // clearing when noone is buzzed in does nothing
//     let clear1 = game.clear_buzzer(&host_id);
//     assert_eq!(clear1, false);
//     assert_eq!(game.active_buzzer, None);

//     // player 1 buzzes in
//     game.buzz(&p1_id);
//     assert_eq!(game.active_buzzer, Some(p1_id));

//     // the host clears it
//     let clear2 = game.clear_buzzer(&host_id);
//     assert_eq!(clear2, true);

//     // player 2 buzzes in
//     game.buzz(&p2_id);
//     assert_eq!(game.active_buzzer, Some(p2_id));

//     // the host clears it again
//     let clear3 = game.clear_buzzer(&host_id);
//     assert_eq!(clear3, true);
//   }
// }
