use serde_json::{json, Map, Value};
use crate::services::shared_api::SHARED_API_CLIENT;
use crate::utils::ensure_success::EnsureSuccess;
use crate::utils::http_client::deserializable::Deserializable;

pub async fn send_invite(email: String) -> (bool, Option<Deserializable<Value>>) {
  let mut map = Map::new();
  map.insert("email".to_string(), json!(email));

  SHARED_API_CLIENT.write().await
    .post("proxy/auth/token", Some(&map)).await
    .ensure_success(true)
}
