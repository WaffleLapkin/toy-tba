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
    type Send = impl Future<Output = ResponseResult<P::Output>>;
    type SendRef = impl Future<Output = ResponseResult<P::Output>>;

    fn send(self) -> Self::Send {
        self.bot.execute_json(&self.payload)
    }

    fn send_ref(&self) -> Self::SendRef {
        self.bot.execute_json(&self.payload)
    }
}

impl<P: Payload + Serialize> core::ops::Deref for RequestJson<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<P: Payload + Serialize> core::ops::DerefMut for RequestJson<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payload
    }
}
