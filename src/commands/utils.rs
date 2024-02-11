use crate::types::voyager_api::GetErrors;
use crate::utils::http_client::deserializable::Deserializable;

trait VecJoin {
  fn join(&self, separator: &str) -> String;
}
impl VecJoin for Vec<String> {
  fn join(&self, separator: &str) -> String {
    self.iter()
      .map(|s| "- ".to_string() + s)
      .collect::<Vec<String>>()
      .join(separator)
  }
}

pub trait GetErrsStr {
  fn get_errs_str(&self) -> String;
}
impl<T: for<'de> serde::Deserialize<'de> + GetErrors> GetErrsStr for Option<Deserializable<T>> {
  fn get_errs_str(&self) -> String {
    let errors = {
      self
        .and_then(|r| r.data().map(|d| d.get_errors()))
        .unwrap_or(&Vec::<String>::new())
    };

    let error_join = errors.iter()
      .map(|s| format!("- {s}"))
      .collect::<Vec::<String>>()
      .join("\n");

    if !errors.is_empty() {
      format!("\nErrors:\n{error_join}",)
    } else { " Internal error.".to_string() }
  }
}
