use std::future::Future;

use crate::{
    methods::SendMessage,
    requester::Requester,
    requests::{HasPayload, Payload, Request},
    types::{ChatId, InputFile},
};

pub struct RateLimits<B> {
    inner: B,
}

impl<B> RateLimits<B> {
    pub fn new(bot: B) -> Self {
        Self { inner: bot }
    }
}

pub struct RateLimitRequest<R> {
    inner: R,
}

impl<R: HasPayload> HasPayload for RateLimitRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.inner.payload_mut()
    }
}

impl<R: Request<Payload = SendMessage>> Request for RateLimitRequest<R> {
    type Err = R::Err;
    type Future =
        impl Future<Output = Result<<<R as HasPayload>::Payload as Payload>::Output, Self::Err>>;

    fn send(self) -> Self::Future {
        async {
            // TODO: rate limit
            unimplemented!()
        }
    }
}

impl<R: Request<Payload = SendMessage> + Deref<Target = SendMessage>> Deref for RateLimitRequest<R> {
    type Target = SendMessage;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<R: Request<Payload = SendMessage> + DerefMut<Target = SendMessage>> DerefMut for RateLimitRequest<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
        T: Into<String>,
    {
        RateLimitRequest {
            inner: self.inner.send_message(chat_id, text),
        }
    }

    type SendPhoto = B::SendPhoto;

    fn send_photo<C, T>(&self, chat_id: C, photo: T) -> Self::SendPhoto
    where
        C: Into<ChatId>,
        T: Into<InputFile>,
    {
        self.inner.send_photo(chat_id, photo)
    }

    type SendDocument = B::SendDocument;

    fn send_document<C, T>(&self, chat_id: C, document: T) -> Self::SendDocument
    where
        C: Into<ChatId>,
        T: Into<InputFile>,
    {
        self.inner.send_document(chat_id, document)
    }
}
