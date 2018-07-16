extern crate awesome_bot;

use std::{thread, time};
use std::iter::FromIterator;
use std::collections::HashMap;

use awesome_bot::{ AwesomeBot, Message, ReplyKeyboardMarkup, ChatAction, PhotoSize, Finisher, MessageType, Audio, Voice, Document, Sticker, Video, Float};

macro_rules! debug {
    ($e: expr) => {
        println!("{:?}", $e);
    }
}

macro_rules! debug_msg {
    ($m: expr, $t: expr) => {
        println!("<{}> {}", $m.from.first_name, $t);
    };
    ($m: expr) => {
        println!("<{}>", $m.from.first_name);
    }
}

fn transform(vecs: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    vecs.iter().map(|x| x.iter().map(|x| x.to_string()).collect()).collect()
}

fn cmd_keyboard(bot: &AwesomeBot, msg: &Message, _: String) {
    let kbl = ReplyKeyboardMarkup {
        keyboard: transform(vec![vec!["I", "<3"], vec!["You"]]),
        resize_keyboard: None,
        one_time_keyboard: None,
        selective: None,
    };
    debug!(bot.answer(msg).text("There you go!").keyboard(kbl).end());
}

fn test_async_hand(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).text("Starting, send me another command in the next 5 seconds...").end());
    thread::sleep(time::Duration::from_millis(5000));
    debug!(bot.answer(msg).text("End async test").end());
}

fn divide_by_two<T: Iterator>(it: T) -> Vec<Vec<T::Item>> {
    let mut result: Vec<Vec<T::Item>> = vec![];
    let mut nit = it;
    loop {
        let e1 = nit.next();
        let e2 = nit.next();
        match (e1, e2) {
            (None, _) => break, // means that we are done!
            (Some(e11), Some(e21)) => result.push(vec![e11, e21]),
            (Some(e11), None) => result.push(vec![e11]),
        }
    }
    result
}

fn show_me_hand(bot: &AwesomeBot, msg: &Message, _: String) {
    let cmds = HashMap::<&str, &str>::from_iter(vec![
        ("/start", "Start the bot!"),
        ("/keyboard", "Send you a keyboard"),
        ("/hidekeyboard", "Hide the keyboard"),
        ("/hardecho", "Echo with force reply"),
        ("/forwardme", "Forward that message to you"),
        ("/sleep", "Sleep for 5 seconds, without blocking, awesome goroutines"),
        ("/showmecommands", "Returns you a keyboard with the simplest commands"),
        ("/sendimage", "Sends you an image"),
        ("/sendimagekey", "Sends you an image with a custom keyboard"),
        ("/senddocument", "Sends you a document"),
        ("/sendsticker", "Sends you a sticker"),
        ("/sendvideo", "Sends you a video"),
        ("/sendlocation", "Sends you a location"),
        ("/sendchataction", "Sends a random chat action")]);

    let commands_k = divide_by_two(cmds.into_iter().map(|(x,_)| x));
    let kbl = ReplyKeyboardMarkup {
        keyboard: transform(commands_k),
        resize_keyboard: None,
        one_time_keyboard: Some(true),
        selective: None,
    };
    debug!(bot.answer(msg).text("There you have the commands!").keyboard(kbl).end());
}

fn hide_keyboard(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).text("Hiden!!").hide(true).end());
}

fn handforw(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).forward(msg.chat.id(), msg.message_id).end());
}

fn hard_echo(bot: &AwesomeBot, msg: &Message, _: String, args: Vec<String>) {
    debug!(bot.answer(msg).text(&args[1]).force(true).end());
}

fn hello_hand(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).text(&format!("Hi {}!", msg.from.first_name)).end());
}

fn tell_me_hand(bot: &AwesomeBot, msg: &Message, _: String, args: Vec<String>) {
    debug!(bot.answer(msg).text(&args[1]).end());
}

// ======
// LOGGER
// ======

fn all_msg_hand(_: &AwesomeBot, msg: &Message) {
    if let MessageType::Text(ref t) = msg.msg {
        debug_msg!(msg, t);
    } else {
        debug_msg!(msg);
    }
}

// =============
// Send media handlers
// =============

fn handimage(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).photo("files/test.jpg").end());
}

fn handaudio(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).audio("files/test.mp3").end());
}

fn handvoice(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).voice("files/test.mp3").end());
}

fn handdoc(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).document("files/test.pdf").end());
}

fn handstick(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).sticker("files/test.webp").end());
}

fn handvideo(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).video("files/test.mp4").end());
}

fn handlocation(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).location(40.324159, -4.21096).end());
}

fn handaction(bot: &AwesomeBot, msg: &Message, _: String) {
    debug!(bot.answer(msg).action(ChatAction::Typing).end());
}

// =================
// MEDIA HANDLERS
// =================

fn transform_info_photos(v: Vec<PhotoSize>) -> String {
    v.iter().map(|p| {
        format!("Image of size ({} x {})\nID: {}\nSize: {}", p.width, p.height, p.file_id, p.file_size.unwrap_or(0))
    }).collect::<Vec<_>>().join("\n----------\n")
}

fn photo_handler(bot: &AwesomeBot, msg: &Message, photos: Vec<PhotoSize>) {
    let imageinfo = transform_info_photos(photos);
    debug!(bot.answer(msg).text(&imageinfo).end());
}

fn audio_handler(bot: &AwesomeBot, msg: &Message, audio: Audio) {
    let message = format!("Information about the audio:\nID: {}\nDuration: {} seconds\nPerformer: {}\nTitle: {}\nMimeType: {}\nFile size: {} Bytes", audio.file_id, audio.duration, audio.performer.unwrap_or("No performer".into()), audio.title.unwrap_or("No title".into()), audio.mime_type.unwrap_or("No mime type".into()), audio.file_size.unwrap_or(0));
    debug!(bot.answer(msg).text(&message).end());
}

fn voice_handler(bot: &AwesomeBot, msg: &Message, voice: Voice) {
    let message = format!("Information about the voice:\nID: {}\nDuration: {} seconds\nMimeType: {}\nFile size: {} Bytes", voice.file_id, voice.duration, voice.mime_type.unwrap_or("No mime type".into()), voice.file_size.unwrap_or(0));
    debug!(bot.answer(msg).text(&message).end());
}

fn document_handler(bot: &AwesomeBot, msg: &Message, document: Document) {
    let mut message = format!("Information about the document:\nID: {}\nFile name: {}\nMimeType: {}\nFile size: {} Bytes", document.file_id, document.file_name.unwrap_or("No name".into()), document.mime_type.unwrap_or("No mime type".into()), document.file_size.unwrap_or(0));
    if let Some(thumb) = document.thumb {
        message = format!("{}\nAnd the information about the thumb:\n{}", message, transform_info_photos(vec![thumb]));
    }
    // Add thumb
    debug!(bot.answer(msg).text(&message).end());
}

fn sticker_handler(bot: &AwesomeBot, msg: &Message, sticker: Sticker) {
    let mut message = format!("Information about the sticker:\nID: {}\nWidth: {}\nHeight: {}\nFile size: {} Bytes", sticker.file_id, sticker.width, sticker.height, sticker.file_size.unwrap_or(0));
    // Add thumb
    if let Some(thumb) = sticker.thumb {
        message = format!("{}\nAnd the information about the thumb:\n{}", message, transform_info_photos(vec![thumb]));
    }
    debug!(bot.answer(msg).text(&message).end());
}

fn video_handler(bot: &AwesomeBot, msg: &Message, video: Video) {
    let mut message = format!("Information about the video:\nID: {}\nWidth: {}\nHeight: {}\nDuration: {}\nMime type: {}\nFile size: {} Bytes", video.file_id, video.width, video.height, video.duration, video.mime_type.unwrap_or("No mime type".into()), video.file_size.unwrap_or(0));
    // Add thumb
    if let Some(thumb) = video.thumb {
        message = format!("{}\nThumb:\n{}", message, transform_info_photos(vec![thumb]));
    }
    debug!(bot.answer(msg).text(&message).end());
}

fn location_handler(bot: &AwesomeBot, msg: &Message, latitude: Float, longitude: Float) {
    let message = format!("Information of location:\nLatitude: {}\nLongitude: {}", latitude, longitude);
    debug!(bot.answer(msg).text(&message).end());
}

fn main() {
    let mut bot = AwesomeBot::from_env("TELEGRAM_BOT_TOKEN");

    // Logger :)
    bot.any_fn(all_msg_hand); // Just to print all the messages

    // Random handlers
    bot.simple_command("sleep", test_async_hand) // Test to prove asynchronous
        .simple_command("showmecommands", show_me_hand) // Send a keyboard with all the commands
        .simple_command("keyboard", cmd_keyboard) // Send a keyboard
        .simple_command("hidekeyboard", hide_keyboard) // Hide the keyboard
        .simple_command("forwardme", handforw) // Forward the message
        .command("hardecho (.+)", hard_echo) // Echo the text with a force reply
        .simple_regex("^Hello!?$", hello_hand) // Answer to Hello!
        .regex("^Tell me (.+)$", tell_me_hand); // An echo without command

    // Add commands that send media files (And action)
    // To make this commands work, you need files to send:
    // files/test.{jpg, mp3, mp4, pdf, webp}
    bot.simple_command("sendimage", handimage)
        .simple_command("sendaudio", handaudio)
        .simple_command("sendvoice", handvoice)
        .simple_command("senddocument", handdoc)
        .simple_command("sendsticker", handstick)
        .simple_command("sendvideo", handvideo)
        .simple_command("sendlocation", handlocation)
        .simple_command("sendaction", handaction);

    // Add handlers that react with media files received sending information
    bot.photo_fn(photo_handler)
        .audio_fn(audio_handler)
        .voice_fn(voice_handler)
        .document_fn(document_handler)
        .sticker_fn(sticker_handler)
        .video_fn(video_handler)
        .location_fn(location_handler);

    let res = bot.simple_start();
    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
