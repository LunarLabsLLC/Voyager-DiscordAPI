use serde::{Deserialize};
use crate::types::voyager_api::{Logs, GetErrors};

#[derive(Debug, Deserialize)]
pub struct DeploymentResult {
  pub logs: Logs,
  id: Option<String>,
}

impl DeploymentResult {
  pub fn new(message: String, errors: Vec<String>, id: Option<String>) -> Self {
    DeploymentResult { logs: Logs { message, errors }, id }
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }
}

impl GetErrors for DeploymentResult {
  fn get_errors(&self) -> &Vec<String> {
    self.logs.errors()
  }
}
