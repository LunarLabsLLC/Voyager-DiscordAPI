use serde::Deserialize;
use crate::types::voyager_api::deployment::Deployment;
use crate::types::voyager_api::{GetErrors, Logs};

#[derive(Debug, Deserialize)]
pub struct DeploymentGetResult {
  logs: Logs,
  deployment: Option<Deployment>,
}

impl DeploymentGetResult {
  pub fn deployment(&self) -> Option<&Deployment> {
    self.deployment.as_ref()
  }
}

impl GetErrors for DeploymentGetResult {
  fn get_errors(&self) -> &Vec<String> {
    self.logs.errors()
  }
}
