use serde::Deserialize;
use crate::types::voyager_api::{GetErrors, Logs};

#[derive(Debug, Deserialize)]
pub struct DeploymentDeleteResult {
  logs: Logs
}

impl DeploymentDeleteResult {
  pub fn new(message: String, errors: Vec<String>) -> Self {
    DeploymentDeleteResult { logs: Logs { message, errors } }
  }
}

impl GetErrors for DeploymentDeleteResult {
  fn get_errors(&self) -> &Vec<String> {
    self.logs.errors()
  }
}
