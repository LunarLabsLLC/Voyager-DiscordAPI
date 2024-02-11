use serde_json::Value;
use tracing::{event, Level};
use crate::services::voyager_api::VOYAGER_API_CLIENT;
use crate::types::voyager_api::deployment_delete_result::DeploymentDeleteResult;
use crate::utils::ensure_success::EnsureSuccess;
use crate::utils::http_client::deserializable::Deserializable;

pub async fn deployment(deployment_id: String) -> (bool, Option<Deserializable<DeploymentDeleteResult>>) {
  let str = format!("Deleting deployment {deployment_id}");
  event!(Level::INFO, str);

  let none: Option<&Value> = { None };
  VOYAGER_API_CLIENT.write().await
    .delete::<DeploymentDeleteResult>(&format!("deployment/{deployment_id}"), none)
    .await.ensure_success(false)
}
