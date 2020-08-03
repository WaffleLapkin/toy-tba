use crate::{
    methods::{GetMe, SendDocument, SendMessage, SendPhoto},
    requests::Request,
    types::{ChatId, InputFile},
};

/// The trait implemented by all bots & bot wrappers.
/// Essentially a request builder factory (?).
///
/// _This trait is included in the crate's [`prelude`](crate::prelude)_
#[doc(spotlight)]
pub trait Requester {
    type GetMe: Request<Payload = GetMe>;

    fn get_me(&self) -> Self::GetMe;

    type SendMessage: Request<Payload = SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>;

    type SendPhoto: Request<Payload = SendPhoto>;

    fn send_photo<C, T>(&self, chat_id: C, photo: T) -> Self::SendPhoto
    where
        C: Into<ChatId>,
        T: Into<InputFile>;

    type SendDocument: Request<Payload = SendDocument>;

    fn send_document<C, T>(&self, chat_id: C, document: T) -> Self::SendDocument
    where
        C: Into<ChatId>,
        T: Into<InputFile>;

    // TODO: remaining 67 methods
}
