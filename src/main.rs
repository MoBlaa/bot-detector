#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use irc_rust::message::Message;
use log::LevelFilter;
use tungstenite::{connect, WebSocket};
use tungstenite::Message as WsMessage;
use url::Url;
use tungstenite::client::AutoStream;
use pretty_env_logger::env_logger::WriteStyle;

fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .default_format()
        .write_style(WriteStyle::Always)
        .init();

    let token = std::env::var("BDET_TOKEN").ok().expect("missing environment variable BDET_TOKEN");
    let nick = std::env::var("BDET_NICK").ok().expect("missing environment variable BDET_NICK");

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
        .write_message(WsMessage::Text("JOIN #mo_blaa".into()))
        .expect("failed to send join message");

    loop {
        let msg = socket.read_message()
            .expect("Error reading message");
        let split = msg.to_string();
        for line in split.split("\r\n") {
            if line.is_empty() {
                continue;
            }
            let msg = Message::from(line);

            handle(&mut socket, msg)
        }
    }
}

fn handle(socket: &mut WebSocket<AutoStream>, msg: Message) {
    match msg.command() {
        "PING" => {
            socket
                .write_message(WsMessage::Text("PONG :tmi.twitch.tv".into()))
                .expect("failed to send pong message");
            debug!("Sent Pong!");
        },
        "PRIVMSG" => debug!("Here adding to some queue would be nice!"),
        "JOIN" => info!("Successfully joined {}!", msg.params().unwrap().iter().next().unwrap()),
        "001" | "002" | "003" | "004" | "375" | "372" | "376" | "353" | "366" => (),
        cmd => debug!("unsupported command: {}", cmd)
    }
}
