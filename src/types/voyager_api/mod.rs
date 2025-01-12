use serde::Deserialize;

pub mod deployment;
pub mod deployment_logs_result;
pub mod deployment_result;
pub mod deployments_get_result;
pub mod deployment_delete_result;
pub mod deployment_get_result;

#[derive(Debug, Deserialize)]
pub struct Logs {
  errors: Vec<String>,
}
impl Logs {
  fn errors(&self) -> &Vec<String> {
    self.errors.as_ref()
  }
}

pub trait GetErrors {
  fn get_errors(&self) -> &Vec<String>;
}
