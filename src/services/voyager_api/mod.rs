use std::sync::Arc;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use tokio::sync::RwLock;
use crate::utils::http_client::http_client::HTTPClient;

pub mod get;
pub mod post;
pub mod delete;

lazy_static! {
  pub static ref VOYAGER_API_CLIENT: Arc<RwLock<HTTPClient>> = {
    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", std::env::var("VOYAGER_API_KEY").unwrap().parse().unwrap());
    HTTPClient::new(std::env::var("VOYAGER_API_ADDR").unwrap(), Some(headers))
      .map(|k| Arc::new(RwLock::new(k)))
      .unwrap_or_else(|e| panic!("Failed to create API client: {}", e))
  };
}
