extern crate irc;
extern crate telegram_bot;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::thread::spawn;
use std::collections::HashMap;


use telegram_bot::*;
use irc::client::prelude::*;
use rustc_serialize::json::decode;


#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
struct TgConfig {
    api_token: String,
}

#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
struct Mapping {
    tg2irc: HashMap<i64, String>,
    irc2tg: HashMap<String, i64>,
}

#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
struct Config {
    irc_config: irc::client::data::Config,
    tg_config: TgConfig,
    mapping: Mapping,
    filterchars: String,
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
    println!("Starting bridge");
    let config = load("config.json").unwrap();
    let irc_config = config.irc_config;
    let tg_config = config.tg_config;
    let tg2irc = config.mapping.tg2irc;
    let irc2tg = config.mapping.irc2tg;
    let filterchars = config.filterchars;

    let irc_server = IrcServer::from_config(irc_config).unwrap();
    irc_server.identify().unwrap();

    let tg_api = Api::from_token(&tg_config.api_token).unwrap();
    let mut listener = tg_api.listener(ListeningMethod::LongPoll(None));

    println!("Bridge started.");

    let filterchars_ = filterchars.clone();
    let iserver2 = irc_server.clone();
    let _ = spawn(move || {
        let _ = listener.listen(|u| {
            if let Some(m) = u.message {
                let name = m.from.username.unwrap_or(m.from.first_name);

                let id = match m.chat {
                    Chat::Private { id, .. } => id,
                    Chat::Group { id, .. } => id,
                    Chat::Channel { id, .. } => id,
                };

                if let Some(target) = tg2irc.get(&id) {
                    // for now only MessageType::Text is supported
                    if let MessageType::Text(msg) = m.msg {
                        if filterchars_.chars().all(|c| !msg.starts_with(c)) {
                            iserver2.send_privmsg(target, &format!("<{}> {}", name, msg))?;
                        }
                    }
                }
            }

            // If none of the "try!" statements returned an error: It's Ok!
            Ok(ListeningAction::Continue)
        });
    });

    let _ = spawn(move || {
            for msg in irc_server.iter() {
                let msg = msg.unwrap();
                if let Command::PRIVMSG(ref target, ref content) = msg.command {
                    if (&filterchars).chars().any(|c| content.starts_with(c)) {
                        continue;
                    }
                    if let Some(target) = irc2tg.get(target) {
                        msg.source_nickname().map(|nick| {
                            let _ = tg_api.send_message(*target,
                                                        format!("<{}> {}", nick, content),
                                                        None,
                                                        None,
                                                        None,
                                                        None);
                        });
                    }
                };

            }
        })
        .join();
}
