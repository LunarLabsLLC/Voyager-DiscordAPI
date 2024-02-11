use serde_json::{json, Map, Value};
use tracing::{event, Level};
use crate::services::voyager_api::VOYAGER_API_CLIENT;
use crate::types::voyager_api::deployment_get_result::DeploymentGetResult;
use crate::types::voyager_api::deployment_logs_result::DeploymentLogsResult;
use crate::types::voyager_api::deployments_get_result::DeploymentsGetResult;
use crate::utils::ensure_success::EnsureSuccess;
use crate::utils::http_client::deserializable::Deserializable;

pub async fn deployment_logs(deployment_id: String) -> (bool, Option<Deserializable<DeploymentLogsResult>>) {
  let str = format!("Getting logs for deployment {deployment_id}");
  event!(Level::INFO, str);

  let none: Option<&Value> = { None };
  VOYAGER_API_CLIENT.write().await
    .get::<DeploymentLogsResult>(&format!("deployment/{deployment_id}/logs"), none)
    .await.ensure_success(false)
}

pub async fn deployment(deployment_id: String) -> (bool, Option<Deserializable<DeploymentGetResult>>) {
  let str = format!("Getting deployment {deployment_id}");
  event!(Level::INFO, str);

  let none: Option<&Value> = { None };
  VOYAGER_API_CLIENT.write().await
    .get::<DeploymentGetResult>(&format!("deployment/{deployment_id}"), none)
    .await.ensure_success(false)
}

pub async fn deployments(repo_url: String) -> (bool, Option<Deserializable<DeploymentsGetResult>>) {
  let mut map = Map::new();
  map.insert("repoUrl".to_string(), json!(repo_url));

  let p = serde_url_params::to_string(&map).unwrap();

  let str = format!("Getting deployments for {repo_url}");
  event!(Level::INFO, str);

  let none: Option<&Value> = { None };
  VOYAGER_API_CLIENT.write().await
    .get::<DeploymentsGetResult>(&format!("deployment?={p}"), none)
    .await.ensure_success(false)
}
