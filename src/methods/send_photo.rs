use crate::requests::{HasPayload, Payload};
use crate::types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup};
use serde::{Deserialize, Serialize};

/// Use this method to send photos. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. More info on Sending Files »
    photo: InputFile,
    /// Photo caption (may also be used when resending photos by file_id), 0-1024 characters
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

impl Payload for SendPhoto {
    type Output = Message;

    const NAME: &'static str = "sendPhoto";
}

impl SendPhoto {
    pub fn new<C, P>(chat_id: C, photo: P) -> Self
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        let chat_id = chat_id.into();
        let photo = photo.into();
        Self {
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

pub trait SendPhotoSetters: HasPayload<Payload = SendPhoto> + Sized {
    fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.payload_mut().chat_id = val.into();
        self
    }

    fn photo<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.payload_mut().photo = val.into();
        self
    }

    fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.payload_mut().caption = Some(val.into());
        self
    }

    fn parse_mode(mut self, val: ParseMode) -> Self {
        self.payload_mut().parse_mode = Some(val);
        self
    }

    fn disable_notification(mut self, val: bool) -> Self {
        self.payload_mut().disable_notification = Some(val);
        self
    }

    fn reply_to_message_id(mut self, val: i32) -> Self {
        self.payload_mut().reply_to_message_id = Some(val);
        self
    }

    fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.payload_mut().reply_markup = Some(val);
        self
    }
}

impl<P> SendPhotoSetters for P where P: HasPayload<Payload = SendPhoto> {}
