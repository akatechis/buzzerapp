use std::cell::RefCell;
use std::rc::Rc;

use data::{Game, Player};
use serde_json::{Error as SerdeError, from_str, to_string};
use std::time::{SystemTime, UNIX_EPOCH};
use ws::{
  listen, Handler, Sender, Result, Message, Handshake, CloseCode, 
  Error as WSError
};

struct Server {
  out: Sender,
  state: Rc<RefCell<Game>>
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
  /// This message is sent by a client that wishes to participate as a player
  /// The attached String contains the player's name
  Join {
    name: String
  },

  /// This message is sent by a client that wishes to host a game
  Host,

  /// This message is sent by a player that wishes to answer a challenge
  /// The attached String contains the player's Identifier
  Buzz {
    id: String
  },

  /// This message is sent by the host to clear the buzzer
  /// The attached String contains the host's Identifier
  Clear {
    id: String
  },

  /// This message is sent by the host to remove all players
  /// The attached String contains the host's Identifier
  Reset {
    id: String
  }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum ServerMessage {

  /// This message is sent by the server when a player has joined.
  PlayerJoined {
    id: String,
    name: String
  },

  /// This message is sent by the server when the host has joined.
  HostJoined {
    id: String
  }
}

impl Into<Message> for ServerMessage {
  fn into(self) -> Message {
    Message::Text(to_string(&self).unwrap())
  }
}

impl Server {
  fn log_game_state(&self) {
    println!("====== Message handled ======");
    println!("====== Game state ======");
    println!("{:?}", &self.state.borrow());
  }
}

impl Handler for Server {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    let msg_json = msg.as_text().unwrap();
    let client_msg: ClientMessage = from_str(&msg_json).unwrap();

    let server_msg: Option<ServerMessage> = match client_msg {

      ClientMessage::Join { name } => {
        let mut state = &self.state;
        let mut game = state.borrow_mut();

        let new_player = Player::new(name.clone());
        let new_player_id = new_player.id.clone();
        if game.join(new_player) {
          Some(ServerMessage::PlayerJoined {
            name, id: new_player_id
          })
        }
        else {
          None
        }
      },

      ClientMessage::Host => {
        None
      },

      ClientMessage::Buzz { .. } => {
        None
      },

      ClientMessage::Clear { .. } => {
        None
      },

      ClientMessage::Reset { .. } => {
        None
      }

    };

    // Message handled, log the current state of the game
    self.log_game_state();

    // Send the response out
    if server_msg.is_some() {
      self.out.broadcast(server_msg.unwrap())
    }
    else {
      Ok(())
    }
  }

  // fn on_close(&mut self, code: CloseCode, reason: &str) {
  //   match code {
  //     CloseCode::Normal => println!("The client is done with the connection."),
  //     CloseCode::Away   => println!("The client is leaving the site."),
  //     CloseCode::Abnormal => println!(
  //       "Closing handshake failed! Unable to obtain closing status from client."),
  //     _ => println!("The client encountered an error: {}", reason),
  //   }
  // }

  fn on_error(&mut self, err: WSError) {
    println!("The server encountered an error: {:?}", err);
  }
}

pub fn start(addr: &str) -> Result<()> {
  let state = Rc::new(RefCell::new(Game::new()));
  listen(addr, |out|
    Server { out: out, state: state.clone() })
}
