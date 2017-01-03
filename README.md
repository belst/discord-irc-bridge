# IRC <-> Discord Bridge 

This programm sends messages between IRC Channels and Discord Chats

## Configuration

Configuration should be pretty straight forward. Just check the [Sample configuration](config.sample.json).

Rename it to `config.json` after you are done configuring.

You can read all about the `irc_config` part [here](https://aatxe.github.io/irc/irc/client/data/config/struct.Config.html) (The Field parts is what u are probably most interested in.)

You will need a Discord Bot Token. You can generate one [here](https://discordapp.com/developers/applications/me)

The `mapping` section contains mappings in both directions.
Use `discord2irc` to send messages from Telegram to IRC and `irc2discord` to send from IRC to Telegram.

You can get the chat ID in discord by going to `settings->appearance` and selecting the `Developer Mode`  
You can now just rightclick a channel and copy the id.`

### filterchars
Messages which start with any of the characters in the string will be ignored (in both directions). This is nice to prevent sending bot commands around.
