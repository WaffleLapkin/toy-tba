use std::future::Future;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bot::Bot,
    requests::{HasPayload, Payload, Request},
    RequestError, ResponseResult,
};

pub struct RequestJson<P> {
    bot: Bot,
    payload: P,
}

impl<P> RequestJson<P> {
    pub const fn new(bot: Bot, payload: P) -> Self {
        Self { bot, payload }
    }

    pub fn as_ref(&self) -> RequestJson<&P> {
        RequestJson {
            bot: self.bot.clone(),
            payload: &self.payload,
        }
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
    type Err = RequestError;
    type Future = impl Future<Output = ResponseResult<P::Output>>;

    fn send(self) -> Self::Future {
        self.bot.execute_json(self.payload)
    }
}
