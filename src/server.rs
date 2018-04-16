use std::rc::Rc;
use std::cell::Cell;
use data::{Game, Player};
use serde_json::{Error as SerdeError, from_str, to_string};
use std::time::{SystemTime, UNIX_EPOCH};
use ws::{
  listen, Handler, Sender, Result, Message, Handshake, CloseCode, 
  Error as WSError
};

struct Server {
  out: Sender,
  state: Rc<Cell<Game>>
}

#[derive(Serialize, Deserialize)]
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

impl Handler for Server {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    // We have a new connection, so we increment the connection counter
    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    let msg_json = msg.as_text().unwrap();
    let parsed: ClientMessage = from_str(&msg_json).unwrap();

    let updated = match parsed {

      ClientMessage::Join { name } => {
        let new_player = Player::new(name);
        let state = self.state.clone().get_mut();
        state.join(new_player)
      }

    };

    // Send the response out
    self.out.send(msg)
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
  let state = Rc::new(Cell::new(Game::new()));
  listen(addr, |out|
    Server { out: out, state: state.clone() })
}
