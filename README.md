# Bot-Detector

Currently detects Twitch Chat-Spambots and sets follower-only mode if to many mesages are sent.

Setup: 

0. Install rust through [rustup](https://rustup.rs/)
1. clone the repo
2. in the root of the project create a file `.env` containing the following:

```dotenv
BDET_TOKEN=oauth token received at https://twitchapps.com/tmi/
BDET_NICK=The lowercase nickname of the user matching the token
BDET_CHANNEL=channel to listen to
```
3. execute `cargo run` in a cli in the root of the project
