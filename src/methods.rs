//#![deny(unused)]

mod get_me;
mod send_document;
mod send_message;
mod send_photo;

pub use self::{
    get_me::{GetMe, GetMeSetters},
    send_document::*,
    send_message::{SendMessage, SendMessageSetters},
    send_photo::{SendPhoto, SendPhotoSetters},
};
