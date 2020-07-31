mod get_me;
mod send_message;

pub use self::{
    send_message::{SendMessage, SendMessageSetters},
    get_me::{GetMe, GetMeSetters},
};

use std::future::Future;
use crate::bot::Bot;
use serde::de::DeserializeOwned;
use crate::RequestError;
use serde::Serialize;

/// A type that is returned when making requests to telegram
pub type ResponseResult<T> = Result<T, crate::RequestError>;

pub trait Request: HasPayload {
    /*
     * Could be mostly `core::future::IntoFuture` though there is no reason before it's integrated
     * in async/await
     */

    type Err;

    type Future: Future<Output = Result<
        <<Self as HasPayload>::Payload as Payload>::Output,
        Self::Err,
    >>;

    fn send(self) -> Self::Future;
}

pub trait Payload {
    type Output;

    const NAME: &'static str;
}

pub struct RequestJson<P> {
    bot: Bot, // TODO: generic?
    payload: P,
}

pub trait HasPayload {
    type Payload: Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload;
}

impl<P> HasPayload for P
where
    P: Payload,
{
    type Payload = Self;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self
    }
}

impl<P> HasPayload for RequestJson<P>
where
    P: Payload,
{
    type Payload = P;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        &mut self.payload
    }
}


impl<P> Request for RequestJson<P>
where
    P: Payload + Serialize,
    P::Output: DeserializeOwned,
{
    type Err = RequestError; // TODO: Bot's err
    type Future = impl Future<Output = ResponseResult<P::Output>>;

    fn send(self) -> Self::Future {
        self.bot.execute_json(self.payload)
    }
}

impl<P> RequestJson<P> {
    pub fn as_ref(&self) -> RequestJson<&P> {
        RequestJson {
            bot: self.bot.clone(),
            payload: &self.payload,
        }
    }

    pub fn new(bot: Bot, payload: P) -> Self {
        Self { bot, payload }
    }
}
