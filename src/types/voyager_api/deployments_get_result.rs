use serde::Deserialize;
use crate::types::voyager_api::{GetErrors};
use crate::types::voyager_api::deployment::Deployment;

#[derive(Debug, Deserialize)]
pub struct DeploymentsGetResult {
  errors: Vec<String>,
  deployments: Vec<Deployment>
}

impl DeploymentsGetResult {
  pub fn deployments(&self) -> &Vec<Deployment> {
    self.deployments.as_ref()
  }
}

impl GetErrors for DeploymentsGetResult {
  fn get_errors(&self) -> &Vec<String> {
    self.errors.as_ref()
  }
}
