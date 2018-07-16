use telegram_bot::*;
use rustc_serialize::{Decodable};

/// Help trait indicating that at least the `end` method is implemented for the SendBuilder structs
pub trait Ender<T: Decodable> {
    fn end(&mut self) -> Result<T>;
}

/// SendBuilder it's a builder struct that allows you to construct answers in
/// a composable way, you will use it transparently with the `send` and `answer`
/// methods of `AwesomeBot`
#[derive(Clone)]
pub struct SendBuilder {
    chat_id: Integer,
    bot: Api,
}

impl SendBuilder {
    /// Create a new SendBuilder, don't use it,
    /// use the `send` and `answer` methods of `AwesomeBot` :)
    pub fn new(id: Integer, bot: Api) -> SendBuilder {
        SendBuilder { chat_id: id, bot: bot }
    }

    /// Start a text constructor to send.
    pub fn text(self, t: &str) -> SendText {
        SendText { send: self, text: t.to_string(), parse_mode: None, disable_webpage_preview: None, reply_to_message_id: None, reply_markup: None }
    }

    /// Start a photo constructor to send.
    pub fn photo(self, t: &str) -> SendPhoto {
        SendPhoto { send: self, photo: t.to_string(), caption: None, reply_to_message_id: None, reply_markup: None }
    }

    /// Start an audio constructor to send.
    pub fn audio(self, t: &str) -> SendAudio {
        SendAudio { send: self, audio: t.to_string(), duration: None, performer: None, title: None, reply_to_message_id: None, reply_markup: None }
    }

    /// Start a voice constructor to send.
    pub fn voice(self, t: &str) -> SendVoice {
        SendVoice { send: self, voice: t.to_string(), duration: None, reply_to_message_id: None, reply_markup: None }
    }

    /// Start a document constructor to send.
    pub fn document(self, t: &str) -> SendDocument {
        SendDocument { send: self, document: t.to_string(), reply_to_message_id: None, reply_markup: None }
    }

    /// Start a sticker constructor to send.
    pub fn sticker(self, t: &str) -> SendSticker {
        SendSticker { send: self, sticker: t.to_string(), reply_to_message_id: None, reply_markup: None }
    }

    /// Start a video constructor to send.
    pub fn video(self, t: &str) -> SendVideo {
        SendVideo { send: self, video: t.to_string(), caption: None, duration: None, reply_to_message_id: None, reply_markup: None }
    }

    /// Start a forward constructor to send.
    pub fn forward(self, to: Integer, msg: Integer) -> SendForward {
        SendForward { send: self, to: to, msg: msg }
    }

    /// Start an action constructor to send.
    pub fn action(self, action: ChatAction) -> SendAction {
        SendAction { send: self, action: action }
    }

    /// Start a location constructor to send.
    pub fn location(self, latitude: Float, longitude: Float) -> SendLocation {
        SendLocation { send: self, latitude: latitude, longitude: longitude, reply_to_message_id: None, reply_markup: None }
    }
}

macro_rules! basesendtype {
    (
        $name: ident,
        $structdoc: expr,
        [$($id: ident => $field: ty),*],
        [$($o_id: ident => ($o_name: ident, $o_field: ty, $docf: expr)),*]) => {

        #[doc="Transparent struct built by `SendBuilder` to send"]
        #[doc=$structdoc]
        #[doc="messages."]
        pub struct $name  {
            send: SendBuilder,
            $($id: $field),*
                ,
            $($o_id: Option<$o_field>),*
        }

        impl $name {
            $(
                #[doc=$docf]
                pub fn $o_name(&mut self, v: $o_field) -> &mut $name {
                    self.$o_id = Some(v);
                    self
                }
                )*
        }
    }
}

macro_rules! addkeyboardfuncs {
    ($name: ident,
     $markname: ident) => {
        /// Add keyboard methods to the struct, only one of these will be sent,
        /// and it will be the last one used.
        impl $name {
            /// Add a keyboard to the reply
            pub fn keyboard(&mut self, r: ReplyKeyboardMarkup) -> &mut $name {
                self.$markname = Some(ReplyMarkup::from(r));
                self
            }

            /// Hide the keyboard
            pub fn hide(&mut self, h: bool) -> &mut $name {
                self.$markname = Some(ReplyMarkup::KeyboardHide(h));
                self
            }

            /// Force the reply to this message
            pub fn force(&mut self, f: bool) -> &mut $name {
                self.$markname = Some(ReplyMarkup::ForceReply(f));
                self
            }
        }
    }
}

basesendtype!(SendText,
              "`Text`",
              [text => String],
              [parse_mode => (parse_mode, ParseMode, "Set `ParseMode` for the message"),
               disable_webpage_preview => (disable_preview, bool, "Set `true` to disable the link preview in the message."),
               reply_to_message_id => (reply_id, Integer, "Set a message ID to reply to with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of directly using this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendText, reply_markup);

impl Ender<Message> for SendText {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_message(
            self.send.chat_id,
            self.text.clone(),
            self.parse_mode,
            self.disable_webpage_preview,
            self.reply_to_message_id,
            self.reply_markup.clone())
    }
}

basesendtype!(SendPhoto,
              "`Photo`",
              [photo => String],
              [caption => (caption, String, "Set a caption to be included with the message."),
               reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendPhoto, reply_markup);

impl Ender<Message> for SendPhoto {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_photo(
            self.send.chat_id,
            self.photo.clone(),
            self.caption.clone(),
            self.reply_to_message_id,
            self.reply_markup.clone(),
            )
    }
}

basesendtype!(SendAudio,
              "`Audio`",
              [audio => String],
              [duration => (duration, Integer, "Set the duration of the track"),
               performer => (performer, String, "Set the performer of the track"),
               title => (title, String, "Set the title of the track"),
               reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendAudio, reply_markup);

impl Ender<Message> for SendAudio {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_audio(
            self.send.chat_id,
            self.audio.clone(),
            self.duration,
            self.performer.clone(),
            self.title.clone(),
            self.reply_to_message_id,
            self.reply_markup.clone(),
            )
    }
}

basesendtype!(SendVoice,
              "`Voice`",
              [voice => String],
              [duration => (duration, Integer, "Set the duration of the voice audio."),
               reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendVoice, reply_markup);

impl Ender<Message> for SendVoice {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_voice(
            self.send.chat_id,
            self.voice.clone(),
            self.duration,
            self.reply_to_message_id,
            self.reply_markup.clone(),
            )
    }
}

basesendtype!(SendDocument,
              "`Document`",
              [document => String],
              [reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendDocument, reply_markup);

impl Ender<Message> for SendDocument {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_document(
            self.send.chat_id,
            self.document.clone(),
            self.reply_to_message_id,
            self.reply_markup.clone(),
            )
    }
}


basesendtype!(SendSticker,
              "`Sticker`",
              [sticker => String],
              [reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendSticker, reply_markup);

impl Ender<Message> for SendSticker {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_sticker(
            self.send.chat_id,
            self.sticker.clone(),
            self.reply_to_message_id,
            self.reply_markup.clone(),
            )
    }
}

basesendtype!(SendVideo,
              "`Video`",
              [video => String],
              [caption => (caption, String, "Set a caption to be included with the message."),
               duration => (duration, Integer, "Set the duration of the video"),
               reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendVideo, reply_markup);

impl Ender<Message> for SendVideo {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_video(
            self.send.chat_id,
            self.video.clone(),
            self.caption.clone(),
            self.duration,
            self.reply_to_message_id,
            self.reply_markup.clone(),
            )
    }
}

basesendtype!(SendForward,
              "`Forward`",
              [to => Integer, msg => Integer],
              []);

impl Ender<Message> for SendForward {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.forward_message(
            self.send.chat_id,
            self.to,
            self.msg,
            )
    }
}

basesendtype!(SendAction,
              "`Action`",
              [action => ChatAction],
              []);

impl Ender<bool> for SendAction {
    fn end(&mut self) -> Result<bool> {
        self.send.bot.send_chat_action(
            self.send.chat_id,
            self.action,
            )
    }
}

basesendtype!(SendLocation,
              "`Location`",
              [latitude => Float,
               longitude => Float],
              [reply_to_message_id => (reply_id, Integer, "Set a message ID to reply with this message."),
               reply_markup => (markup, ReplyMarkup, "Set a `ReplyMarkup` to send, but instead of this, use the `keyboard`, `hide` or `force` methods")]);

addkeyboardfuncs!(SendLocation, reply_markup);

impl Ender<Message> for SendLocation {
    fn end(&mut self) -> Result<Message> {
        self.send.bot.send_location(
            self.send.chat_id,
            self.latitude,
            self.longitude,
            self.reply_to_message_id,
            self.reply_markup.clone())
    }
}
