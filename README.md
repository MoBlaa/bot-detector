# Bot-Detector

![Rust](https://github.com/MoBlaa/bot-detector/workflows/Rust/badge.svg)

Currently detects Twitch Chat-Spambots and sets follower-only mode if to many mesages are sent.

Setup: 

1. download appropriate binary from [releases](https://github.com/MoBlaa/bot-detector/releases)
2. extract it to a new directory
3. make `bot-detector` executable if necessary (`chmod +x bot-detector` on linux)
4. modify the file `configs/.env` (enable showing hidden files in file explorer):
    ```dotenv
    RUST_LOG=info
    BDET_TOKEN=oauth token received at https://twitchapps.com/tmi/
    BDET_NICK=The lowercase nickname of the user matching the token
    BDET_CHANNEL=channel to listen to
    ```
5. run the application by executing it in a terminal

The `RUST_LOG` value configures the bot output to your terminal:

- Show only important messages from the bot: `RUST_LOG=info`
- Show every single message received from twitch + debug information (not that clear): `RUST_LOG=bot_detector=trace`
