use serde::Deserialize;
use crate::types::voyager_api::{GetErrors};

#[derive(Debug, Deserialize)]
pub struct DeploymentDeleteResult {
  errors: Vec<String>,
}

impl GetErrors for DeploymentDeleteResult {
  fn get_errors(&self) -> &Vec<String> {
    self.errors.as_ref()
  }
}
