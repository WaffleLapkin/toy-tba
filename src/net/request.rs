use std::future::Future;

use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    multipart::Form,
    Client, Response,
};
use serde::de::DeserializeOwned;

use super::{TelegramResponse, TELEGRAM_API_URL};
use crate::{RequestError, ResponseResult};

pub(crate) async fn request_multipart<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: impl Future<Output = Result<Form, serde_multipart::Error>>,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    let params = params
        .await
        // this `expect` should be ok since we don't write request those may trigger error here
        .expect("serialization of request to be infallible");

    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .multipart(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub(crate) async fn request_json<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: Vec<u8>,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

async fn process_response<T>(response: Response) -> ResponseResult<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str::<TelegramResponse<T>>(
        &response.text().await.map_err(RequestError::NetworkError)?,
    )
    .map_err(RequestError::InvalidJson)?
    .into()
}
