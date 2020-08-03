use std::future::Future;

use crate::requests::{HasPayload, Payload};

/// ???
#[doc(spotlight)]
pub trait Request: HasPayload {
    /*
     * Could be mostly `core::future::IntoFuture` though there is no reason before it's integrated
     * in async/await
     */

    type Err;

    type Send: Future<
        Output = Result<<<Self as HasPayload>::Payload as Payload>::Output, Self::Err>,
    >;

    // NOTE: this intentionally forbids borrowing from self
    type SendRef: Future<
        Output = Result<<<Self as HasPayload>::Payload as Payload>::Output, Self::Err>,
    >;

    /// Send the request.
    ///
    /// ## Examples
    ///
    /// ```
    /// # async {
    /// use toy_tba::{methods::GetMe, requests::{Request, RequestJson}, types::User, bot::Bot};
    ///
    /// let bot = Bot::new("TOKEN");
    /// let method = GetMe::new();
    /// let request = JsonRequest::new(bot, method);
    /// let _: User = request.send().await.unwrap();
    ///# }
    /// ```
    fn send(self) -> Self::Send;

    /// Send the request.
    ///
    /// This method is analogous to [`send`](Request::send), but this method doesn't take the
    /// ownership of `self`. This allows to send the same (or slightly different) requests over and
    /// over.
    ///
    /// _Also_ it is expected that calling this method is better than just `clone`ing the requests.
    /// (because instead of copying all the data and then serializing it, this method should just
    /// serialize the data)
    ///
    /// ## Examples
    ///
    /// ```
    /// # async {
    /// use toy_tba::prelude::*;
    ///
    /// let bot = Bot::new("TOKEN");
    /// # let chat_ids = vec![1, 2, 3, 4].into_iter().map(Into::into);
    ///
    /// let mut req = bot.send_message(0, "Hi there!");
    /// for chat_id in chat_ids {
    ///     req.chat_id = chat_id;
    ///     req.send_ref().await.unwrap();
    /// }
    ///# }
    /// ```
    fn send_ref(&self) -> Self::SendRef;
}
