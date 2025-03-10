use crate::types::enums::Error;

use reqwest::{
    Client,
    header::{
        HeaderMap,
        HeaderValue
    }
};



pub fn init_client() -> Result<Client, Error> {
    let mut headers_map: HeaderMap = HeaderMap::with_capacity(1);
    headers_map.insert("Content-Type", HeaderValue::from_static("application/json"));

    Client::builder()
        .default_headers(headers_map)
        .timeout(tokio::time::Duration::from_secs(5))
        .pool_idle_timeout(tokio::time::Duration::from_secs(60))
        .build()
        .map_err(|_| Error::CreateHttpClient) 
}