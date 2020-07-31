//! TODO: documentation
// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs" cargo doc --open --all-features
// ```
#![feature(type_alias_impl_trait)]
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg))]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

#[macro_use]
mod local_macros;
mod net;
mod error;

pub mod bot;
pub mod tools;
pub mod requests;
pub mod types;

pub use self::{
    error::{RequestError, DownloadError},
};
use crate::tools::RateLimits;
use crate::requests::SendMessageSetters;

pub mod prelude {


    pub use super::requests::{
        GetMeSetters as _,
    };
    pub use super::{
        bot::{Bot, Requester as _},
        requests::Request as _,
    };
}



#[tokio::test]
async fn main() {
    use crate::prelude::*;

    let bot = RateLimits::new(Bot::new("( ͡° ͜ʖ ͡°)"));
    let x = bot
        .send_message(218485655, "ох блять оно работает")
        .disable_notification(true)
        .send()
        .await
        .unwrap();
}





// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=33e2237370a9fbae1598194a8bc7bb4f