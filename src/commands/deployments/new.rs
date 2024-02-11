use crate::{Context, Error};
use crate::commands::utils::GetErrsStr;
use crate::services::voyager_api;
use crate::utils::handle_reply::HandleReply;

/// Deploys the given repo & branch to the given subdomain
#[poise::command(prefix_command, slash_command)]
pub async fn new(
  ctx: Context<'_>,
  #[description = "The repo url to deploy, including the branch"]
  repo_url: String,
  #[description = "The subdomain to deploy to"]
  sub_domain: Option<String>,
  #[description = "The environment to deploy"]
  mode: String,
) -> Result<(), Error> {
  let (is_success, result) = voyager_api::post::deployment(repo_url, sub_domain, mode).await;

  match is_success {
    true => {
      let id = result.unwrap().data().unwrap();
      let id = id.id().unwrap();
      ctx.reply(format!("Deployed successfully! Id: {id}")).await.handle()
    },
    _ => {
      let errors = result.get_errs_str();
      ctx.reply(format!("Failed to deploy.{errors}")).await.handle()
    }
  };

  Ok(())
}
