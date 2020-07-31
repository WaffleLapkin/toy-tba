use reqwest::Client;
use std::sync::Arc;
use crate::requests::{Request, GetMe, SendMessage, RequestJson, ResponseResult, Payload};
use crate::types::ChatId;
use crate::net::request_json;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct Bot {
    client: Client,
    // `Bot` is frequently cloned so it's reasonable to use `Arc` instead of `String`.
    token: Arc<str>,
}

impl Bot {
    /// Execute json-request
    ///
    /// ## Example
    /// ```no_run
    /// # use sahke::{Bot, requests::payloads::SendMessage};
    /// # #[tokio::main] async fn main_() {
    /// let bot = Bot::new("TOKEN");
    /// let payload = SendMessage::new(123456, "text");
    /// bot.execute_json(&payload).await;
    /// # }
    /// ```
    ///
    /// **NOTE**: we recommend to use
    ///   `bot.send_message(id, "text").send().await` instead
    pub(crate) fn execute_json<P>(&self, payload: P) -> impl Future<Output = ResponseResult<P::Output>>
    where
        P: Payload + Serialize,
        P::Output: DeserializeOwned,
    {
        let client = self.client.clone();
        let token = Arc::clone(&self.token);

        async move {
            request_json(&client, token.as_ref(), P::NAME, payload).await
        }
    }
}

impl Bot {
    pub fn new(token: impl Into<Arc<str>>) -> Self {
        Self::with_client(token, Client::new())
    }

    pub fn with_client(token: impl Into<Arc<str>>, client: Client) -> Self {
        Self {
            client,
            token: token.into(),
        }
    }
}

pub trait Requester {
    type GetMe: Request<Payload = GetMe>;

    fn get_me(&self) -> Self::GetMe;

    type SendMessage: Request<Payload = SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>;

    // TODO: remaining 69 methods
}

impl Requester for Bot {
    type GetMe = RequestJson<GetMe>;

    fn get_me(&self) -> Self::GetMe {
        Self::GetMe::new(self.clone(), GetMe::new())
    }

    type SendMessage = RequestJson<SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::SendMessage::new(self.clone(), SendMessage::new(chat_id, text))
    }
}