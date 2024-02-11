use serde::Deserialize;
use crate::types::voyager_api::deployment::Deployment;
use crate::types::voyager_api::{GetErrors};

#[derive(Debug, Deserialize)]
pub struct DeploymentGetResult {
  errors: Vec<String>,
  deployment: Option<Deployment>,
}

impl DeploymentGetResult {
  pub fn deployment(&self) -> Option<&Deployment> {
    self.deployment.as_ref()
  }
}

impl GetErrors for DeploymentGetResult {
  fn get_errors(&self) -> &Vec<String> {
    self.errors.as_ref()
  }
}
