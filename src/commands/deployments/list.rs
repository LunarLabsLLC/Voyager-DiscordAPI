use poise::futures_util::{future};
use crate::{Context, Error};
use crate::commands::utils::GetErrsStr;
use crate::services::voyager_api;
use crate::types::voyager_api::deployment::Deployment;
use crate::utils::handle_reply::HandleReply;

/// Gets the deployments for the given repo
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn list(
  ctx: Context<'_>,
  #[description = "The repo url to get deployments for"]
  repo_url: String,
) -> Result<(), Error> {
  let (is_success, result) = voyager_api::get::deployments(repo_url).await;

  match is_success {
    true => {
      let deployments = result.unwrap();
      let deployments = deployments.data().unwrap().deployments();
      let deployment_vec = deployments.iter()
        .map(|d| {
        let Deployment {
          container_id,
          repo_url,
          branch,
          host,
          mode,
          state,
          created_at,
          ..
        } = d;

        format!("\
          - Container Id: {container_id}\n\
          - Repo Url: {repo_url}\
          - Branch: {branch}\n\
          - Host: {host}\n\
          - Mode: {mode}\n\
          - State: {state}\n\
          - Created At: {created_at}")
        })
        .collect::<Vec<String>>();

      ctx.reply("Sending all deployments...").await.handle();

      let futures = deployment_vec.iter().map(|d| ctx.say(d));
      let results = future::join_all(futures).await;
      results.iter().for_each(|r| r.handle());
    },

    _ => {
      let errors = result.get_errs_str();
      ctx.reply(format!("Failed to get deployments.{errors}")).await.handle()
    }
  };

  Ok(())
}
