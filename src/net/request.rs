use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{RequestError, ResponseResult};

use super::{TelegramResponse, TELEGRAM_API_URL};

pub(crate) async fn request_multipart<T, P>(
    client: &Client,
    token: &str,
    method_name: &str,
    payload: P,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    let params = serde_multipart::to_form(&payload)
        .await
        .expect("serialization of request to be infallible"); // this should be ok since we don't
                                                              // write request those may trigger
                                                              // error here

    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .multipart(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub(crate) async fn request_json<T, P>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: P,
) -> ResponseResult<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    let response = client
        .post(&super::method_url(TELEGRAM_API_URL, token, method_name))
        .json(&params)
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
