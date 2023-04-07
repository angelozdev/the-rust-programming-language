use reqwest::header as reqwest_header;
use serde::Deserialize;
use std::fmt::Debug;

pub async fn handle_success<T: for<'de> Deserialize<'de> + Debug>(
    res: reqwest::Response,
) -> Option<T> {
    let result = res.json::<T>().await;
    match result {
        Ok(value) => Some(value),
        Err(err) => {
            println!("[HandleSuccess]: {}", err);
            None
        }
    }
}

pub fn build_headers() -> reqwest_header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("reqwest-rs/0.10.8"),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}
