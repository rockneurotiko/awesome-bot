extern crate awesome_bot;

use awesome_bot::*;

fn echohandler(bot: &AwesomeBot, msg: &Message, _: String, args: Vec<String>) {
    let toecho = &args[1]; // We can access because the pattern match have that argument
    let phrase = format!("Echoed: {}", toecho);
    let sended = bot.answer(msg).text(&phrase).end();
    println!("{:?}", sended);
}

fn main() {
    let mut bot = AwesomeBot::from_env("TELEGRAM_BOT_TOKEN");
    bot.command("echo (.+)", echohandler);
    let res = bot.simple_start();
    if let Err(e) = res {
        println!("An error occurred: {}", e);
    }
}
