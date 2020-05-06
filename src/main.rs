extern crate crossbeam_queue;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use irc_rust::message::Message;
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
use tungstenite::{connect, WebSocket};
use tungstenite::client::AutoStream;
use tungstenite::Message as WsMessage;
use url::Url;

use crate::queue::Queue;
use std::ops::Index;
use std::str::FromStr;
use chrono::{NaiveDateTime, Duration};

mod queue;

fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .default_format()
        .write_style(WriteStyle::Always)
        .init();

    let token = std::env::var("BDET_TOKEN").ok().expect("missing environment variable BDET_TOKEN");
    let nick = std::env::var("BDET_NICK").ok().expect("missing environment variable BDET_NICK");
    let channel = std::env::var("BDET_CHANNEL").ok().expect("missing encironment variable BDET_CHANNEL");

    let (mut socket, response) = connect(Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap()).expect("Can't connect to Twitch");

    if !response.status().is_success() && !response.status().is_informational() {
        panic!("returned error: {}", response.status());
    }
    info!("Connected to Twitch Server!");

    socket
        .write_message(WsMessage::Text(format!("PASS {}", token)))
        .expect("failed to send pass message");
    socket
        .write_message(WsMessage::Text(format!("NICK {}", nick)))
        .expect("failed to send nick message");
    socket
        .write_message(WsMessage::Text(format!("JOIN #{}", channel)))
        .expect("failed to send join message");
    socket
        .write_message(WsMessage::Text("CAP REQ :twitch.tv/membership".into()))
        .expect("failed to get membership cap");
    socket
        .write_message(WsMessage::Text("CAP REQ :twitch.tv/tags".into()))
        .expect("failed to get tags cap");
    socket
        .write_message(WsMessage::Text("CAP REQ :twitch.tv/commands".into()))
        .expect("failed to get commands cap");

    let queue = Queue::new(20, Duration::seconds(1));

    loop {
        let msg = socket.read_message()
            .expect("Error reading message");
        let split = msg.to_string();
        for line in split.split("\r\n") {
            if line.is_empty() {
                continue;
            }
            let msg = Message::from(line);

            match msg.command() {
                "PING" => {
                    socket
                        .write_message(WsMessage::Text("PONG :tmi.twitch.tv".into()))
                        .expect("failed to send pong message");
                    debug!("Sent Pong!");
                }
                "PRIVMSG" => {
                    let tags = msg.tags().expect("no tags present in PRIVMSG");
                    let time = &tags["tmi-sent-ts"];
                    let name = &tags["display-name"];

                    let time = usize::from_str(time)
                        .expect("failed to parse timestamp to usize");
                    let secs = (time / 1000) as i64;
                    let nanos = ((time % 1000) * 1000 * 1000) as u32;
                    let timestamp = NaiveDateTime::from_timestamp(secs, nanos);
                    debug!("Received message sent @ '{}' = '{}'", time, timestamp);
                    if !queue.add((timestamp, name.to_string())) {
                        warn!("Activating follower only Mode!");
                        if let Err(why) = socket
                            .write_message(WsMessage::Text(format!("PRIVMSG #{} :/followers 10m", channel))) {
                            warn!("failed to set followers only mode: {}", why);
                        }
                    }
                }
                "JOIN" => info!("Successfully joined {}!", msg.params().unwrap().iter().next().unwrap()),
                "CAP" => {
                    let params = msg.params().unwrap();
                    let mut iter = params.iter();
                    match (iter.next(), iter.next()) {
                        (Some("*"), Some("ACK")) => info!("Successfully obtained capability for '{}'!", params.trailing.unwrap()),
                        _ => warn!("failed to obtain some capability: {}", msg)
                    }
                },
                "ROOMSTATE" => {
                    let tags = msg.tags().unwrap();
                    let followers_only_duration = i64::from_str(&tags["followers-only"])
                        .expect("invalid duration");
                    if followers_only_duration < 0 {
                        info!("Room is no longer in followers-only mode!");
                    } else if followers_only_duration == 0 {
                        info!("Room is in followers-only mode!");
                    } else {
                        info!("Room is now in {}m followers-only mode!", followers_only_duration);
                    }
                }
                "001" | "002" | "003" | "004" | "375" | "372" | "376" | "353" | "366" => (),
                cmd => debug!("unsupported command: {}", cmd)
            }
        }
    }
}
