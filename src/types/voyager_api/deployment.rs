use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Deployment {
  pub id: String,
  pub port: u16,
  pub mode: String,
  pub host: String,
  pub state: String,
  pub directory: String,
  pub branch: String,

  #[serde(rename = "containerId")]
  pub container_id: String,
  #[serde(rename = "dnsRecordId")]
  pub dns_record_id: String,
  #[serde(rename = "repoUrl")]
  pub repo_url: String,
  #[serde(rename = "createdAt")]
  pub created_at: i64,
}
