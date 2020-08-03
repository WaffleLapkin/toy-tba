//! TODO: documentation
// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs" cargo doc --open --all-features
// ```
#![feature(type_alias_impl_trait)]
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_spotlight))]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

// TODO: add a way to send multiple common requests without clonning
//       (we serialize the data anyway, soo...)

#[macro_use]
mod local_macros;
mod error;
mod net;
mod util;

pub mod bot;
pub mod methods;
pub mod requester;
pub mod requests;
pub mod tools;
pub mod types;

pub use self::error::{DownloadError, RequestError, ResponseResult};

pub mod prelude {
    // see https://discordapp.com/channels/442252698964721669/459149231702278154/739825135443378176
    // for the reason why no_inline here
    #[doc(no_inline)]
    pub use crate::{bot::Bot, requester::Requester as _, requests::Request as _};

    #[doc(no_inline)]
    pub use crate::methods::{
        GetMeSetters as _, SendDocumentSetters as _, SendMessageSetters as _, SendPhotoSetters as _,
    };
}

// Just some testing
#[tokio::test]
async fn main() {
    use crate::{prelude::*, tools::RateLimits, types::InputFile};

    use tokio::{fs::File, io::AsyncReadExt};

    let mut file = File::open("/home/waffle/Downloads/photo_2020-07-27_22-02-39.jpg")
        .await
        .unwrap();

    let mut buf = Vec::with_capacity(
        file.metadata()
            .await
            .map(|m| m.len() as usize)
            .unwrap_or(64),
    );

    file.read_to_end(&mut buf).await.unwrap();

    let bot = RateLimits::new(Bot::new("TOKEN"));
    let x = bot
        .send_photo(
            218485655,
            InputFile::Memory {
                file_name: "сырна.jpg".to_string(),
                data: buf.into(),
            },
        )
        .disable_notification(true)
        .send()
        .await
        .unwrap();

    let file_id = x.photo().unwrap().first().unwrap().file_id.clone();
    bot.send_photo(218485655, InputFile::FileId(file_id))
        .send()
        .await
        .unwrap();

    bot.send_photo(
        218485655,
        InputFile::Url(String::from(
            "https://pbs.twimg.com/media/EeT1yiuWoAA8wj7?format=jpg&name=small",
        )),
    )
    .send()
    .await
    .unwrap();
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=33e2237370a9fbae1598194a8bc7bb4f
