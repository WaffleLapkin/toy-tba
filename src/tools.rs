use crate::bot::{Bot, Requester};
use crate::requests::{GetMe, SendMessage, HasPayload, Request, Payload};
use crate::types::ChatId;
use std::future::Future;

pub struct RateLimits<B> {
    inner: B,
}

impl<B> RateLimits<B> {
    pub fn new(bot: B) -> Self {
        Self { inner: bot }
    }
}

pub struct RateLimitRequest<R> {
    inner: R
}

impl<R: HasPayload> HasPayload for RateLimitRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.inner.payload_mut()
    }
}

impl<R: Request<Payload = SendMessage>> Request for RateLimitRequest<R> {
    type Err = R::Err;
    type Future = impl Future<Output = Result<
        <<R as HasPayload>::Payload as Payload>::Output,
        Self::Err,
    >>;

    fn send(self) -> Self::Future {
        async {
            // TODO: rate limit
            unimplemented!()
        }
    }
}

impl<B: Requester> Requester for RateLimits<B> {
    type GetMe = B::GetMe;

    fn get_me(&self) -> Self::GetMe {
        self.inner.get_me()
    }

    type SendMessage = RateLimitRequest<B::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>
    {
        RateLimitRequest { inner: self.inner.send_message(chat_id, text) }
    }
}