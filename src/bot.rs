use std::{future::Future, sync::Arc};

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::net::{download_file, download_file_stream};
use crate::{
    methods::{GetMe, SendDocument, SendMessage, SendPhoto},
    net,
    requester::Requester,
    requests::{Payload, RequestJson, RequestMultipart},
    types::{ChatId, InputFile},
    DownloadError, ResponseResult,
};
use bytes::Bytes;
use tokio::io::AsyncWrite;
use tokio::stream::Stream;

/// The api client that sends requests over the net.
///
/// The main api of the `Bot` is located in [`impl Requester`].
///
/// Note: the bot mostly consists of [`Arc`]s so it's `clone` is cheap and there is no need to pass
/// it by ref (you will have a lot of problems with `'lifetimes` doing that)
/// or wrap in `Arc` (useless overhead).
///
/// [`Arc`]: std::sync::Arc
#[derive(Debug, Clone)]
pub struct Bot {
    client: Client,
    // `Bot` is frequently cloned (at least 1 time per every request)
    // so it's reasonable to use `Arc` instead of `String`.
    token: Arc<str>,
}

impl Bot {
    /// Create new instance of a bot from the bot token.
    ///
    /// For a way how to obtain a token see [telegram guide][tgg] (or just go to [@BotFather] right away)
    ///
    /// [tgg]: https://core.telegram.org/bots#3-how-do-i-create-a-bot
    /// [@BotFather]: https://t.me/BotFather
    ///
    /// ## Examples
    ///
    /// ```
    /// use toy_tba::bot::Bot;
    /// let bot = Bot::new("000000000:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    /// // Use bot somehow, e.g.: `bot.send_message(...).await`
    /// # drop(bot);
    /// ```
    pub fn new(token: impl Into<Arc<str>>) -> Self {
        Self::with_client(token, Client::new())
    }

    /// Create new instance of a bot from the bot token and the http client.
    ///
    /// See also: [`Bot::new`](Bot::new)
    pub fn with_client(token: impl Into<Arc<str>>, client: Client) -> Self {
        Self {
            client,
            token: token.into(),
        }
    }
}

impl Bot {
    /// Download a file from Telegram into `destination`.
    /// `path` can be obtained from [`get_file`] method.
    ///
    /// For downloading as Stream of Chunks see [`download_file_stream`].
    ///
    /// ## Examples
    ///
    /// ```ignore // TODO: without `get_file` this won't work :|
    /// # use toy_tba::bot::Bot;
    ///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = Bot::new("TOKEN");
    /// let mut file = File::create("/some/path/test.png").await?;
    ///
    /// let TgFile { file_path, .. } = bot.get_file("*file_id*").send().await?;
    /// bot.download_file(&file_path, &mut file).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// TODO: add read-into-mem example
    ///
    /// [`get_file`]: crate::bot::Bot::get_file
    /// [`download_file_stream`]: crate::bot::Bot::download_file_stream
    pub async fn download_file<D>(
        &self,
        path: &str,
        destination: &mut D,
    ) -> Result<(), DownloadError>
    where
        D: AsyncWrite + Unpin,
    {
        download_file(&self.client, &self.token, path, destination).await
    }

    /// Download a file from Telegram.
    ///
    /// `path` can be obtained from the [`get_file`] method.
    ///
    /// For downloading into [`AsyncWrite`] (e.g. [`tokio::fs::File`])
    /// see  [`download_file`].
    ///
    /// [`get_file`]: crate::bot::Bot::get_file
    /// [`AsyncWrite`]: tokio::io::AsyncWrite
    /// [`tokio::fs::File`]: tokio::fs::File
    /// [`download_file`]: crate::bot::Bot::download_file
    pub async fn download_file_stream(
        &self,
        path: &str,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error> {
        download_file_stream(&self.client, &self.token, path).await
    }
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

    type SendPhoto = RequestMultipart<SendPhoto>;

    fn send_photo<C, T>(&self, chat_id: C, photo: T) -> Self::SendPhoto
    where
        C: Into<ChatId>,
        T: Into<InputFile>,
    {
        Self::SendPhoto::new(self.clone(), SendPhoto::new(chat_id, photo))
    }

    type SendDocument = RequestMultipart<SendDocument>;

    fn send_document<C, T>(&self, chat_id: C, document: T) -> Self::SendDocument
    where
        C: Into<ChatId>,
        T: Into<InputFile>,
    {
        Self::SendDocument::new(self.clone(), SendDocument::new(chat_id, document))
    }
}

impl Bot {
    // TODO: should be make those "low-level" methods public?
    //  cons:
    //    - creates second way to send requests
    //    - this api isn't composable
    //  pros:
    //    - ???

    // TODO: it may be better in case of `.send` (not `_ref`) to serialize the data in the future,
    //       not before it

    pub(crate) fn execute_json<P>(
        &self,
        payload: &P,
    ) -> impl Future<Output = ResponseResult<P::Output>>
    where
        P: Payload + Serialize,
        P::Output: DeserializeOwned,
    {
        let client = self.client.clone();
        let token = Arc::clone(&self.token);

        let params = serde_json::to_vec(payload)
            // this `expect` should be ok since we don't write request those may trigger error here
            .expect("serialization of request to be infallible");

        // async move to capture client&token
        async move { net::request_json(&client, token.as_ref(), P::NAME, params).await }
    }

    pub(crate) fn execute_multipart<P>(
        &self,
        payload: &P,
    ) -> impl Future<Output = ResponseResult<P::Output>>
    where
        P: Payload + Serialize,
        P::Output: DeserializeOwned,
    {
        let client = self.client.clone();
        let token = Arc::clone(&self.token);

        let params = serde_multipart::to_form(payload);

        // async move to capture client&token
        async move { net::request_multipart(&client, token.as_ref(), P::NAME, params).await }
    }
}
