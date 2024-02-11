use serde_json::{json, Map, Value};
use tracing::{event, Level};
use crate::services::voyager_api::VOYAGER_API_CLIENT;
use crate::types::voyager_api::deployment_result::DeploymentResult;
use crate::utils::ensure_success::EnsureSuccess;
use crate::utils::http_client::deserializable::Deserializable;

pub async fn deployment(repo_url: String, sub_domain: Option<String>, mode: String) -> (bool, Option<Deserializable<DeploymentResult>>) {
  let mut map = Map::new();
  map.insert("mode".to_string(), json!(mode));
  map.insert("repoUrl".to_string(), json!(repo_url));

  if let Some(sub_domain) = &sub_domain {
    map.insert("subDomain".to_string(), json!(sub_domain));
  }

  let p = serde_url_params::to_string(&map).unwrap();

  let str = format!(
    "Deploying {repo_url}{} in {mode} mode",
    sub_domain.map_or(String::new(), |s| format!(" to subdomain {s}"))
  );
  event!(Level::INFO, str);

  let none: Option<&Value> = { None };
  VOYAGER_API_CLIENT.write().await
    .post::<DeploymentResult>(&format!("deployment?={p}"), none)
    .await.ensure_success(false)
}
