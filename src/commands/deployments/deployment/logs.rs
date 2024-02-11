use poise::futures_util::future;
use crate::{Context, Error};
use crate::commands::utils::GetErrsStr;
use crate::services::voyager_api;
use crate::utils::handle_reply::HandleReply;

/// Returns the logs for the given deployment
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn logs(
  ctx: Context<'_>,
  #[description = "The repo url to get deployments for"]
  deployment_id: String,
) -> Result<(), Error> {
  let (is_success, result) = voyager_api::get::deployment_logs(deployment_id).await;

  match is_success {
    true => {
      let logs = result.unwrap().data().unwrap();
      let logs = logs.deployment_logs().unwrap();
      let logs = logs.iter()
        .map(|l| format!("- {l}"))
        .collect::<Vec<String>>()
        .join("\n");

      ctx.reply("Sending all logs...").await.handle();

      let n = logs.len() / 1500;
      let mut logs_vec = Vec::new();
      (1..=n).fold(logs, |acc, _| {
        let (first, second) = acc.split_at(1500);
        logs_vec.push(first.to_owned());
        second.to_string()
      });

      let futures = logs_vec.iter().map(|l| ctx.say(l));
      let results = future::join_all(futures).await;
      results.iter().for_each(|r| r.handle());
    },

    _ => {
      let errors = result.get_errs_str();
      ctx.reply(format!("Failed to get logs.{errors}")).await.handle();
    }
  };

  Ok(())
}
