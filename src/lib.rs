//! This crate helps writing bots for Telegram. This is a framework to build the bots,
//! the main wrapper crate are `telegram-bot`
//!
//! How to use it
//! -------------
//!
//! The first step is always create the AwesomeBot instance,
//! this will represent your bot, so, if you have more than one bot,
//! you will have to create more instances.
//!
//! You have two ways to create the bot, with `new`,
//! that you pass the bot token diretly, or with `from_env`, more recommended.
//!
//! This framework uses a routing methodology to apply behaviour.
//! There are plenty of routing ways available, the main ones are:
//! `command`, `simple_command`, `any_fn`.
//! But there are many more, check the docs of [AwesomeBot](struct.AwesomeBot.html)
//!
//! Then, to send things you need to use the SendBuilder struct, to create an instance use the `send` or `answer` methods in `AwesomeBot`.
//!
//! This uses the "builder" methodology, for example, to answer a message with a text, while disabling web pages, you do (Assuming that bot is `&AwesomeBot` and msg is `&Message`):
//!
//! ``` ignore
//! bot.answer(msg).text("Answering this text").disable_preview(true).end();
//! ```
//!
//! Check [`SendBuilder`](struct.SendBuilder.html) struct implementation to see the methods available (text, photo, audio, ...)
//!
//! Once you have all your routings, you need to start the bot, right now it support only
//! getUpdates method, just call the `simple_start` method in AwesomeBot.
//!
//! You don't have to worry about blocking the bot in a function handler,
//! because it uses a thread pool (of 4 threads right now,
//! it will be configurable in the future), so the handling for
//! a message is done in his own thread.
//!
//! # Examples
//!
//! ## Minimalistic example (Echo text)
//! ```no_run
//! extern crate awesome_bot;
//!
//! use awesome_bot::*;
//!
//! fn echohandler(bot: &AwesomeBot, msg: &Message, _: String, args: Vec<String>) {
//!     let toecho = &args[1]; // We can access because the pattern match have that argument
//!     let phrase = format!("Echoed: {}", toecho);
//!     let sended = bot.answer(msg).text(&phrase).end();
//!     println!("{:?}", sended);
//! }
//!
//! fn main() {
//!     let mut bot = AwesomeBot::from_env("TELEGRAM_BOT_TOKEN");
//!     bot.command("echo (.+)", echohandler);
//!     let res = bot.simple_start();
//!     if let Err(e) = res {
//!         println!("An error occurred: {}", e);
//!     }
//! }
//! ```
//!
//! ## More examples
//! You have more examples in `examples/` directory in the project's repository.
//!

extern crate telegram_bot;
extern crate regex;
extern crate rustc_serialize;
extern crate scoped_threadpool;

mod send;
mod test;

pub use send::*;

pub use telegram_bot::*;

use scoped_threadpool::Pool;

use regex::Regex;
use std::env;
use std::sync::Arc;

/// Represents audio and voice, this is used in `all_music_fn` handler.
pub enum GeneralSound {
    Audio(Audio),
    Voice(Voice),
}

// This enumerable is used to determine what type of routing handler to use
#[derive(Clone)]
enum Muxer {
    PatternMux(Regex, Arc<Fn(&AwesomeBot, &Message, String, Vec<String>) + Send + Sync + 'static>),
    TextMux(Regex, Arc<Fn(&AwesomeBot, &Message, String) + Send + Sync + 'static>),
    PhotoMux(Arc<Fn(&AwesomeBot, &Message, Vec<PhotoSize>) + Send + Sync + 'static>),
    VideoMux(Arc<Fn(&AwesomeBot, &Message, Video) + Send + Sync + 'static>),
    DocumentMux(Arc<Fn(&AwesomeBot, &Message, Document) + Send + Sync + 'static>),
    StickerMux(Arc<Fn(&AwesomeBot, &Message, Sticker) + Send + Sync + 'static>),
    AudioMux(Arc<Fn(&AwesomeBot, &Message, Audio) + Send + Sync + 'static>),
    VoiceMux(Arc<Fn(&AwesomeBot, &Message, Voice) + Send + Sync + 'static>),
    GeneralAudioMux(Arc<Fn(&AwesomeBot, &Message, GeneralSound) + Send + Sync + 'static>),
    ContactMux(Arc<Fn(&AwesomeBot, &Message, Contact) + Send + Sync + 'static>),
    LocationMux(Arc<Fn(&AwesomeBot, &Message, Float, Float) + Send + Sync + 'static>),
    NewParticipantMux(Arc<Fn(&AwesomeBot, &Message, User) + Send + Sync + 'static>),
    LeftParticipantMux(Arc<Fn(&AwesomeBot, &Message, User) + Send + Sync + 'static>),
    NewTitleMux(Arc<Fn(&AwesomeBot, &Message, String) + Send + Sync + 'static>),
    NewChatPhotoMux(Arc<Fn(&AwesomeBot, &Message, Vec<PhotoSize>) + Send + Sync + 'static>),
    DeleteChatPhotoMux(Arc<Fn(&AwesomeBot, &Message, GroupChat) + Send + Sync + 'static>),
    GroupChatCreatedMux(Arc<Fn(&AwesomeBot, &Message, GroupChat) + Send + Sync + 'static>),
    AnyMux(Arc<Fn(&AwesomeBot, &Message) + Send + Sync + 'static>),
}

// This macros match one muxer and execute a block while sending to the "Any" message :)
// First: self
// Second: msg to pass
// Third: List of Pattern to match => Code block to execute for that Pattern
macro_rules! muxer_match {
    ($_self: expr, $msg: expr, [$($pat:pat => $result: expr),*]) => {
        for m in &$_self.muxers {
            match m {
                &Muxer::AnyMux(ref f) => {
                    f($_self, &$msg);
                },
                $($pat => $result,)*
                _ => {},
            }
        }
    }
}

// This macro add a muxer to the muxers vec
// First: self
// Second: handler (function to add)
// Third: The Muxer enum type
// Fourth: List of extra parameters, in order, first passed to the muxer
macro_rules! add_muxer {
    ($_self: expr,
     $handler: expr,
     $mux: expr,
     [$($extra: expr),*]) => {
        {
            let fa = Arc::new($handler);
            $_self.muxers.push($mux($($extra,)* fa.clone()));
            $_self
        }
    }
}

/// Main type for building the Telegram Bot.
///
/// You can create a new instance with `new` or `from_env`, add routing handlers and start the bot.
pub struct AwesomeBot {
    bot: Api,
    /// The ID of the bot.
    pub id: Integer,
    /// The username of the bot.
    pub username: String,
    muxers: Vec<Muxer>,
}

impl Clone for AwesomeBot {
    fn clone(&self) -> AwesomeBot {
        let b = self.bot.clone();
        let mut v: Vec<Muxer> = Vec::new();
        for m in &self.muxers {
            v.push(m.clone());
        }
        AwesomeBot {
            bot: b,
            id: self.id,
            username: self.username.clone(),
            muxers: v,
        }
    }
}

// unsafe impl Send for AwesomeBot { }
// unsafe impl Sync for AwesomeBot { }

impl AwesomeBot {
    // ===========================================================
    // Constructors
    // ===========================================================
    /// Creates a new bot with the given token. This checks that the token is a
    /// valid Telegram Bot Token by calling `get_me`.
    /// It panics if the token is invalid.
    pub fn new(token: &str) -> AwesomeBot {
        let bot = Api::from_token(token).unwrap();
        match bot.get_me() {
            Ok(user) => AwesomeBot {
                bot: bot,
                id: user.id,
                username: user.username.unwrap_or("".to_string()),
                muxers: Vec::new(),
            },
            Err(e) => panic!("Invalid token! ({})", e),
        }
    }

    /// Will receive the Bot Token from the environment variable `var` and call `new`.
    /// It panics if the environment variable can't be readed or the token are invalid.
    pub fn from_env(var: &str) -> AwesomeBot {
        let token = env::var(var)
            .ok()
            .expect(&format!("Environment variable {} error.", var));

        Self::new(&token)
    }

    // Listener functions

    /// Start the bot using `getUpdates` method, calling the routings defined before calling this method.
    pub fn simple_start(&self) -> Result<()> {
        let mut listener = self.bot.listener(ListeningMethod::LongPoll(Some(20)));
        let mut pool = Pool::new(4);
        // let botcloned = Arc::new(self.clone());

        pool.scoped(|scoped| {
            // Handle updates
            let result = listener.listen(|u| {
                if let Some(m) = u.message {
                    // let bot_instance = botcloned.clone();
                    scoped.execute(move || {
                        // bot_instance.handle_message(m);
                        self.handle_message(m);
                    });
                }
                Ok(ListeningAction::Continue)
            });
            scoped.join_all(); // Wait all scoped threads to finish
            result
        })
    }

    // Send builders
    /// Start a SendBuilder builder directly with the id, this is useful when you have the id saved and want to send a message.
    pub fn send(&self, id: Integer) -> SendBuilder {
        SendBuilder::new(id, self.bot.clone())
    }

    /// Start a SendBuilder builder answering a message directly, this is used to answer in a handler to the sender of the message.
    pub fn answer(&self, m: &Message) -> SendBuilder {
        self.send(m.chat.id())
    }

    // AUXILIAR FUNCTIONS

    // This function modify the command adding the username and the some regex cleanup
    fn modify_command(orig: &str, username: &str) -> String {
        let s = String::from(orig);
        let mut words: Vec<String> = s.split_whitespace().map(|x| String::from(x)).collect();

        if words.len() >= 1 {
            let mut lastchar = "";
            let mut comm = words.remove(0);
            if comm.ends_with("$") {
                lastchar = "$";
                comm.pop();
            }
            let ns: String = format!("{}(?:@{})?{}", comm, username, lastchar);
            words.insert(0, ns);
        }

        // let mut ns: String = words.join(" ");
        let mut ns: String = words.connect(" ");
        if !ns.starts_with("^/") {
            if !ns.starts_with("/") {
                ns.insert(0, '/');
            }
            if !s.starts_with("^") {
                ns.insert(0, '^');
            }
        }
        if !ns.ends_with("$") {
            ns.push_str("$");
        }
        ns
    }
}

// Handle functions, this implementations are separated of the other
impl AwesomeBot {
    fn handle_text_msg(&self, msg: &Message, text: String) {
        use Muxer::*;
        muxer_match!(self, msg,
                     [&TextMux(ref r, ref f) =>
                      {
                         if r.is_match(&text) {
                             f(self, msg, text.clone());
                         }
                      },
                      &PatternMux(ref r, ref f) =>
                      {
                          if r.is_match(&text) { // If there are match
                              r.captures(&text) // Get the captures
                                  .map(|c| { // Map over them because are Option<_>
                                      // Change the capture groups to Vec<String>
                                      c.iter().map(|x| String::from(x.unwrap_or("")))
                                          .collect::<Vec<_>>()
                                  })
                                  .map(|captures_vec|{
                                      // If everything goes well, call the function
                                      f(self, msg, text.clone(), captures_vec)
                                  });
                          }
                      }]
                     );
    }

    fn handle_image_msg(&self, msg: &Message, photos: Vec<PhotoSize>) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&PhotoMux(ref f) => f(self, msg, photos.clone())]
                          );
    }

    fn handle_video_msg(&self, msg: &Message, video: Video) {
        use Muxer::*;
        muxer_match!(self, msg,
                     [&VideoMux(ref f) => f(self, msg, video.clone())]
                     );
    }

    fn handle_document_msg(&self, msg: &Message, document: Document) {
        use Muxer::*;
        muxer_match!(self, msg,
                     [&DocumentMux(ref f) => f(self, msg, document.clone())]
                     );
    }

    fn handle_sticker_msg(&self, msg: &Message, sticker: Sticker) {
        use Muxer::*;
        muxer_match!(self, msg,
                     [&StickerMux(ref f) => f(self, msg, sticker.clone())]
                     );
    }

    fn handle_audio_msg(&self, msg: &Message, audio: Audio) {
        use Muxer::*;
        muxer_match!(self, msg,
                     [&AudioMux(ref f) => f(self, msg, audio.clone()),
                      &GeneralAudioMux(ref f) => f(self, msg, GeneralSound::Audio(audio.clone()))]
                          );
    }

    fn handle_voice_msg(&self, msg: &Message, voice: Voice) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&VoiceMux(ref f) => f(self, msg, voice.clone()),
                           &GeneralAudioMux(ref f) => f(self, msg, GeneralSound::Voice(voice.clone()))]
                          );
    }

    fn handle_contact_msg(&self, msg: &Message, cont: Contact) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&ContactMux(ref f) => f(self, msg, cont.clone())]
                          );
    }

    fn handle_location_msg(&self, msg: &Message, f1: Float, f2: Float) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&LocationMux(ref f) => f(self, msg, f1, f2)]
                          );
    }

    fn handle_new_chat_msg(&self, msg: &Message, newp: User) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&NewParticipantMux(ref f) => f(self, msg, newp.clone())]
                          );
    }

    fn handle_left_part_msg(&self, msg: &Message, user: User) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&LeftParticipantMux(ref f) => f(self, msg, user.clone())]
                          );
    }

    fn handle_new_title_msg(&self, msg: &Message, title: String) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&NewTitleMux(ref f) => f(self, msg, title.clone())]
                          );
    }

    fn handle_chat_photo_msg(&self, msg: &Message, photos: Vec<PhotoSize>) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&NewChatPhotoMux(ref f) => f(self, msg, photos.clone())]
                          );
    }

    fn handle_delete_photo_msg(&self, msg: &Message, group: GroupChat) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&DeleteChatPhotoMux(ref f) => f(self, msg, group.clone())]
                          );
    }

    fn handle_group_created_msg(&self, msg: &Message, group: GroupChat) {
        use Muxer::*;
        muxer_match!(self, msg,
                          [&GroupChatCreatedMux(ref f) => f(self, msg, group.clone())]
                          );
    }

    fn handle_message(&self, message: Message) {
        use telegram_bot::MessageType::*;
        // // Any message
        // let anybot = bot.clone();
        // let anym = m.clone();
        // thread::spawn(move || {
        //     anybot.handle_any_msg(anym);
        // });

        // Rest of messages :)
        match message.msg.clone() {
            Text(text) => self.handle_text_msg(&message, text),
            Audio(audio) => self.handle_audio_msg(&message, audio),
            Voice(voice) => self.handle_voice_msg(&message, voice),
            Photo(photos) => self.handle_image_msg(&message, photos),
            File(document) => self.handle_document_msg(&message, document),
            Sticker(sticker) => self.handle_sticker_msg(&message, sticker),
            Video(video) => self.handle_video_msg(&message, video),
            Contact(contact) => self.handle_contact_msg(&message, contact),
            Location(loc) => self.handle_location_msg(&message, loc.latitude, loc.longitude),
            NewChatParticipant(user) => self.handle_new_chat_msg(&message, user),
            LeftChatParticipant(user) => self.handle_left_part_msg(&message, user),
            NewChatTitle(title) => self.handle_new_title_msg(&message, title),
            NewChatPhoto(photos) => self.handle_chat_photo_msg(&message, photos),
            DeleteChatPhoto => {
                if let Chat::Group(group) = message.chat.clone() {
                    self.handle_delete_photo_msg(&message, group);
                }
            },
            GroupChatCreated => {
                if let Chat::Group(group) = message.chat.clone() {
                    self.handle_group_created_msg(&message, group);
                }
            },
        }
    }
}


// Add functions implementation
/// Methods to add function handlers on different routings.
///
/// The different parameters of the handlers are:
///
/// - Common:
///    - `&AwesomeBot`: First argument of all the handlers, it's the bot itself, so you can send/answer.
///    - `&Message`: Second argument of all the handlers, it's the message that has triggered the handler, you can grab all the information you want. This struct comes from `telegram-bot` crate.
/// - Specific to some handlers:
///    - `String`: Refers to the full text if it's a command or regular expression, or the new title of a group.
///    - `Vec<String>`: This are only in `command` and `regex` method, and are a vector of the capture groups.
///    - `Vec<PhotoSize>`: Represents an image (it's received in different sizes) and you get it when a photo arrives or when someone change a group photo.
///    - `Video`, `Document`, `Sticker`, `Audio`, `Voice`, `GeneralSound`, `Contact`, `Float`: All this parameters are the media that made the handler trigger, for example, in `video_fn` you will receive a `Video`.
///    - `User`: An User is received when a participant left or enter a group.
///    - `GroupChat`: Whenever someone delete a chat photo, or create a group (add the bot to the group) you receive this.
impl AwesomeBot {
    /// Add a complex command routing (With capture groups).
    ///
    /// This method will transform the pattern to be exhaustive and include the mention to the bot, for example, the pattern `echo (.+)` will be used inside an the regular expression `^/start(?:@usernamebot)? (.+)$`
    pub fn command<H>(&mut self, pattern: &str, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, String, Vec<String>) + Send + Sync + 'static
    {
        let nr = Self::modify_command(pattern, &self.username);
        match Regex::new(&*nr) {
            Ok(r) => {
                add_muxer!(self, handler, Muxer::PatternMux, [r])
            }
            Err(_) => self
        }
    }

    /// Add a simple command routing (Without capture groups).
    ///
    /// This method will transform the pattern the same as `command` method, but the handler will not get the capture groups.
    pub fn simple_command<H>(&mut self, pattern: &str, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, String) + Send + Sync + 'static
    {
        let nr = Self::modify_command(pattern, &self.username);
        match Regex::new(&*nr) {
            Ok(r) => {
                add_muxer!(self, handler, Muxer::TextMux, [r])
            }
            Err(_) => self
        }
    }

    /// Add a complex regular expression routing (With capture groups)
    ///
    /// This method won't tranform anything about the regular expression, you are free to write the expression you want and receive the capture groups matched.
    pub fn regex<H>(&mut self, pattern: &str, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, String, Vec<String>) + Send + Sync + 'static
    {
        match Regex::new(pattern) {
            Ok(r) => {
                add_muxer!(self, handler, Muxer::PatternMux, [r])
            }
            Err(_) => self
        }
    }

    /// Add a complex simple expression routing (Without capture groups)
    ///
    /// This method won't tranform anything about the regular expression, you are free to write the expression. The difference with `regex` is that you won't receive the capture groups.
    pub fn simple_regex<H>(&mut self, pattern: &str, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, String) + Send + Sync + 'static
    {
        match Regex::new(pattern) {
            Ok(r) => {
                add_muxer!(self, handler, Muxer::TextMux, [r])
            }
            Err(_) => self
        }
    }

    // pub fn multi_regex<H>(&mut self, patterns: Vec<&str>, handler: H) -> &mut AwesomeBot
    //     where H: Fn(&AwesomeBot, &Message, String, Vec<String>) + Send + Sync + 'static
    // {
    // }

    /// Add a routing handler that will be triggerer in every message, useful to log.
    pub fn any_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::AnyMux, [])
    }

    /// Add a photo media routing handler.
    pub fn photo_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Vec<PhotoSize>) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::PhotoMux, [])
    }

    /// Add a video media routing handler.
    pub fn video_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Video) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::VideoMux, [])
    }

    /// Add a document media routing handler.
    pub fn document_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Document) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::DocumentMux, [])
    }

    /// Add a sticker media routing handler.
    pub fn sticker_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Sticker) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::StickerMux, [])
    }

    /// Add a audio media routing handler.
    pub fn audio_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Audio) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::AudioMux, [])
    }

    /// Add a voice media routing handler.
    pub fn voice_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Voice) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::VoiceMux, [])
    }

    /// Add a routing handler that is triggered when it is received an `Audio` or a `Voice`
    pub fn all_music_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, GeneralSound) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::GeneralAudioMux, [])
    }

    /// Add a contact routing handler.
    pub fn contact_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Contact) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::ContactMux, [])
    }

    /// Add a location routing handler.
    pub fn location_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Float, Float) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::LocationMux, [])
    }

    /// Add a routing handler that is triggered when a new participant enters in a group.
    pub fn new_participant_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, User) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::NewParticipantMux, [])
    }

    /// Add a routing handler that is triggered when a participant left a group.
    pub fn left_participant_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, User) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::LeftParticipantMux, [])
    }

    /// Add a routing handler that is triggered when the title of a group chat is changed.
    pub fn new_title_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, String) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::NewTitleMux, [])
    }

    /// Add a routing handler that is triggered when the photo of a group chat is changed.
    pub fn new_chat_photo_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, Vec<PhotoSize>) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::NewChatPhotoMux, [])
    }

    /// Add a routing handler that is triggered when the photo of a group chat is deleted.
    pub fn delete_chat_photo_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, GroupChat) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::DeleteChatPhotoMux, [])
    }

    /// Add a routing handler that is triggered when a group chat is created.
    pub fn group_chat_created_fn<H>(&mut self, handler: H) -> &mut AwesomeBot
        where H: Fn(&AwesomeBot, &Message, GroupChat) + Send + Sync + 'static
    {
        add_muxer!(self, handler, Muxer::GroupChatCreatedMux, [])
    }
}



// fn detect_file_or_id(name: &str, path: String) -> SendPath {
//     // When PathExt becomes stable, use Path::new(&path).exists() instead of this!
//     let check = fs::metadata(&path);
//     if path.contains(".") && check.is_ok() && check.unwrap().is_file() {
//         SendPath::File(name.to_owned(), Path::new(&path).to_path_buf())
//     } else {
//         SendPath::Id(name.to_owned(), path)
//     }
// }
