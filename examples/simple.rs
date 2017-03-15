extern crate awesome_bot;

use awesome_bot::*;

fn echohandler(bot: &AwesomeBot, msg: &Message, _: String, args: Vec<String>) {
    // We can access safely because the pattern match have that argument mandatory
    let toecho = &args[1];
    let phrase = format!("Echoed: {}", toecho);
    // Send the text in a beauty way :)
    let sent = bot.answer(msg).text(&phrase).end();
    println!("{:?}", sent);
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
