use serde::{Deserialize};
use crate::types::voyager_api::{GetErrors};

#[derive(Debug, Deserialize)]
pub struct DeploymentResult {
  errors: Vec<String>,
  id: Option<String>,
}

impl DeploymentResult {
  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }
}

impl GetErrors for DeploymentResult {
  fn get_errors(&self) -> &Vec<String> {
    self.errors.as_ref()
  }
}
