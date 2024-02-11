use serde::Deserialize;
use crate::types::voyager_api::{Logs, GetErrors};

#[derive(Debug, Deserialize)]
pub struct DeploymentLogsResult {
  logs: Logs,
  #[serde(rename = "deploymentLogs")]
  deployment_logs: Option<Vec<String>>,
}

impl DeploymentLogsResult {
  pub fn new(message: String, errors: Vec<String>, deployment_logs: Option<Vec<String>>) -> Self {
    DeploymentLogsResult { logs: Logs { message, errors }, deployment_logs }
  }

  pub fn deployment_logs(&self) -> Option<&Vec<String>> {
    self.deployment_logs.as_ref()
  }
}

impl GetErrors for DeploymentLogsResult {
  fn get_errors(&self) -> &Vec<String> {
    self.logs.errors()
  }
}
