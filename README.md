# Bot-Detector

Currently detects Twitch Chat-Spambots and sets follower-only mode if to many mesages are sent.

Setup: 

- download binary and move it to an empty folder
- in the same folder: create a file `.env` containing the following:

```dotenv
BDET_TOKEN=oauth token received at https://twitchapps.com/tmi/
BDET_NICK=The lowercase nickname of the user matching the token
BDET_CHANNEL=channel to listen to
```
