# Irc <-> Telegram Bridge 

This programm sends messages between IRC Channels and Telegram Chats

## Configuration

Configuration should be pretty straight forward. Just check the [Sample configuration](config.sample.json).

Rename it to `config.json` after you are done configuring.

You can read all about the `irc_config` part [here](https://aatxe.github.io/irc/irc/client/data/config/struct.Config.html) (The Field parts is what u are probably most interested in.)

You will need a Telegram API Token. You can read about how to get one [here](https://core.telegram.org/bots#botfather)

The `mapping` section contains mappings in both directions.
Use `tg2irc` to send messages from Telegram to IRC and `irc2tg` to send from IRC to Telegram.

### Getting the Chat ID for Telegram Chats

This is a bit tricky, since there is no option other than to use the api directly:

1. go to: https://api.telegram.org/bot&lt;token&gt;/getUpdates (replace `<token>` with the bot token you got before)
2. in the result you will get an array of messages (if there are any new messages, better write a few times in the chat where the bot is):

    ```js
    {
        "ok": true,
        "result": [
            {
                "message": {
                    "chat": {
                        "all_members_are_administrators": true,
                        "id": -13371337,
                        "title": "myAwesomeGroup",
                        "type": "group"
                    },
                    "date": 1483441022,
                    "from": {
                        "...": "..."
                    },
                    "message_id": 1337,
                    "text": "foo"
                },
                "update_id": 42133742
            }
        ]
    }
    ```
3. look for the correct `message->chat->id` value. This will be most likely negative for group chats and positive for normal chats.

### filterchars
Messages which start with any of the characters in the string will be ignored (in both directions). This is nice to prevent sending bot commands around.
