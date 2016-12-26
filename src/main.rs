extern crate irc;
extern crate telegram_bot;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::thread::spawn;


use telegram_bot::*;
use irc::client::prelude::*;
use rustc_serialize::json::decode;


#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
struct TgConfig {
    api_token: String,
}

#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
struct Config {
    irc_config: irc::client::data::Config,
    tg_config: TgConfig,
}

fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    decode(&data[..]).map_err(|_| {
        Error::new(ErrorKind::InvalidInput,
                   "Failed to decode configuration file.")
    })
}


fn main() {
    let config = load("config.json").unwrap();
    let irc_config = config.irc_config;
    let tg_config = config.tg_config;
    let irc_server = IrcServer::from_config(irc_config).unwrap();
    irc_server.identify().unwrap();

    let tg_api = Api::from_token(&tg_config.api_token).unwrap();
    println!("getMe: {:?}", tg_api.get_me());
    let mut listener = tg_api.listener(ListeningMethod::LongPoll(None));

    let iserver2 = irc_server.clone();
    let _ = spawn(move || {
        let _ = listener.listen(|u| {
            if let Some(m) = u.message {
                let name = if let Some(uname) = m.from.username {
                    uname
                } else {
                    m.from.first_name
                };

                match m.chat {
                    Chat::Group { id, .. } => {
                        if id != -139231621 {
                            return Ok(ListeningAction::Continue);
                        }
                    }
                    _ => return Ok(ListeningAction::Continue),
                }

                // Match message type
                // cant send voice/video/images/sticker to irc
                if let MessageType::Text(t) = m.msg {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);
                    iserver2.send_privmsg("#kbot-dev", &format!("<{}> {}", name, t))?;
                }
            }

            // If none of the "try!" statements returned an error: It's Ok!
            Ok(ListeningAction::Continue)
        });
    });

    let _ = spawn(move || {
            for msg in irc_server.iter() {
                let msg = msg.unwrap();
                println!("{}", msg);
                if let Command::PRIVMSG(_, ref content) = msg.command {
                    msg.source_nickname().map(|nick| {
                        let _ = tg_api.send_message(-139231621,
                                                    format!("<{}> {}", nick, content),
                                                    None,
                                                    None,
                                                    None,
                                                    None);
                    });
                };

            }
        })
        .join();
}
