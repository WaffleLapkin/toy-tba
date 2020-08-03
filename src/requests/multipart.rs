use serde::de::DeserializeOwned;

use super::Payload;
use crate::bot::Bot;
use crate::requests::{HasPayload, Request};
use crate::{RequestError, ResponseResult};
use serde::Serialize;
use std::future::Future;

// pub trait Payload: Method {
//     fn payload(&self) -> multipart::Form;
// }

/// Ready-to-send telegram request.
///
/// Note: params will be sent to telegram using [`multipart/form-data`]
///
/// See [SendAnimation] for reference implementation.
///
/// [`multipart/form-data`]: https://core.telegram.org/bots/api#making-requests
/// [SendAnimation]: crate::requests::payloads::SendAnimation
#[must_use = "requests do nothing until sent"]
pub struct RequestMultipart<P> {
    bot: Bot,
    payload: P,
}

impl<P> RequestMultipart<P> {
    pub fn new(bot: Bot, payload: P) -> Self {
        Self { bot, payload }
    }

    // /// Send request to telegram
    // pub async fn send(&self) -> ResponseResult<P::Output> {
    //     network::request_multipart(
    //         self.bot.client(),
    //         self.bot.token(),
    //         P::NAME,
    //         self.payload.payload(),
    //     )
    //         .await
    // }
}

impl<P> HasPayload for RequestMultipart<P>
where
    P: Payload,
{
    type Payload = P;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        &mut self.payload
    }
}

impl<P> Request for RequestMultipart<P>
where
    P: Payload + Serialize,
    P::Output: DeserializeOwned,
{
    type Err = RequestError;
    type Future = impl Future<Output = ResponseResult<P::Output>>;

    fn send(self) -> Self::Future {
        self.bot.execute_multipart(self.payload)
    }
}

impl<P: Payload + Serialize> core::ops::Deref for RequestMultipart<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<P: Payload + Serialize> core::ops::DerefMut for RequestMultipart<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payload
    }
}
