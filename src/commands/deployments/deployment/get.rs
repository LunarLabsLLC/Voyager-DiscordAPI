use crate::{Context, Error};
use crate::commands::utils::GetErrsStr;
use crate::services::voyager_api;
use crate::types::voyager_api::deployment::Deployment;
use crate::utils::handle_reply::HandleReply;

/// Get deployment for a deployment id
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn get(
  ctx: Context<'_>,
  #[description = "The deployment id of the deployment to get"]
  id: String,
) -> Result<(), Error> {
  let (is_success, result) = voyager_api::get::deployment(id).await;

  match is_success {
    true => {
      let deployment = result.unwrap().data().unwrap();
      let deployment = deployment.deployment().unwrap();
      let Deployment {
        container_id,
        repo_url,
        branch,
        host,
        mode,
        state,
        created_at,
        ..
      } = deployment;

      let str = format!("\
          - Container Id: {container_id}\n\
          - Repo Url: {repo_url}\
          - Branch: {branch}\n\
          - Host: {host}\n\
          - Mode: {mode}\n\
          - State: {state}\n\
          - Created At: {created_at}");

      ctx.reply(format!("Deployment Info:\n{str}")).await.handle();
    },

    _ => {
      let errors = result.get_errs_str();
      ctx.reply(format!("Failed to get deployment.{errors}")).await.handle()
    }
  };

  Ok(())
}
