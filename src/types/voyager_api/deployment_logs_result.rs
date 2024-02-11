use serde::Deserialize;
use crate::types::voyager_api::{GetErrors};

#[derive(Debug, Deserialize)]
pub struct DeploymentLogsResult {
  errors: Vec<String>,
  #[serde(rename = "deploymentLogs")]
  deployment_logs: Option<Vec<String>>,
}

impl DeploymentLogsResult {
  pub fn deployment_logs(&self) -> Option<&Vec<String>> {
    self.deployment_logs.as_ref()
  }
}

impl GetErrors for DeploymentLogsResult {
  fn get_errors(&self) -> &Vec<String> {
    self.errors.as_ref()
  }
}
