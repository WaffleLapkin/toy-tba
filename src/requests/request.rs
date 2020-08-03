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

    type Future: Future<
        Output = Result<<<Self as HasPayload>::Payload as Payload>::Output, Self::Err>,
    >;

    /// Send the request.
    ///
    /// ## Examples
    ///
    /// ```
    ///  async {
    /// use toy_tba::{methods::GetMe, requests::{Request, RequestJson}, types::User, bot::Bot};
    ///
    /// let bot = Bot::new("TOKEN");
    /// let method = GetMe::new();
    /// let request = JsonRequest::new(bot, method);
    /// let _: User = request.send().await.unwrap();
    ///# }
    /// ```
    fn send(self) -> Self::Future;
}
