Awesome Bot
===========

[![Travis](https://travis-ci.org/rockneurotiko/awesome-bot.svg?branch=master)](https://travis-ci.org/rockneurotiko/awesome-bot)
[![Crates.io](https://img.shields.io/crates/v/awesome-bot.svg)](https://crates.io/crates/telegram-bot)

[**Documentation**](http://web.neurotiko.com/awesome-bot/awesome_bot/)

A framework to build in an easy way your own [Telegram](https://telegram.org/) bots. More information [here](https://core.telegram.org/bots). Official API [here](https://core.telegram.org/bots/api).

This framework is based in [`telegram-bot`](https://github.com/LukasKalbertodt/telegram-bot) library to communicate with the API.

## Usage

This framework/library is available in `crates.io`. Just add this to your `Cargo.toml` dependencies:

```
[dependencies]
awesome-bot = "0.1.0"
```

## Example

Here is a simple example (download in [`examples/simple.rs`](https://github.com/rockneurotiko/awesome-bot/blob/master/examples/simple.rs)).

``` rust
extern crate awesome_bot;

use awesome_bot::*;

fn echohandler(bot: &AwesomeBot, msg: &Message, _: String, args: Vec<String>) {
    // We can access safely because the pattern match have that argument mandatory
    let toecho = &args[1];
    let phrase = format!("Echoed: {}", toecho);
    // Send the text in a beauty way :)
    let sended = bot.answer(msg).text(&phrase).end();
    println!("{:?}", sended);
}

fn main() {
    // Create the Awesome Bot (You need TELEGRAM_BOT_TOKEN environment with the token)
    let mut bot = AwesomeBot::from_env("TELEGRAM_BOT_TOKEN");
    // Add a command, this will add the routing to that function.
    bot.command("echo (.+)", echohandler);

    // Start the bot with getUpdates
    let res = bot.simple_start();
    if let Err(e) = res {
        println!("An error occurred: {}", e);
    }
}
```

In the `examples` folder are more code:

- [`examples/simple.rs`](https://github.com/rockneurotiko/awesome-bot/blob/master/examples/simple.rs): An echo example, really simple.
- [`examples/complete.rs`](https://github.com/rockneurotiko/awesome-bot/blob/master/examples/complete.rs): An example that have all (I think, probably not all, but almost all) features of `awesome-bot`, it's a big example ;-)

*Note: To execute `examples/complete.rs` with all the features, you will need to add some test files that the bot will send, this files are: `files/test.{jpg, mp3, mp4, pdf, webp}` for image, audio/voice, video, document and sticker*

## Collaboration

All help are welcome! Open issues, open PR of code or documentation, make suggestions, tell me that my rust sucks (and why), what you want :)

You can contact me in [telegram](https://telegram.me/rock_neurotiko).
