use crate::{Context, Error};
use crate::commands::utils::GetErrsStr;
use crate::services::voyager_api;
use crate::utils::handle_reply::HandleReply;

/// Deletes the deployment with the given id
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn delete(
  ctx: Context<'_>,
  #[description = "The id of the deployment to delete"]
  id: String,
) -> Result<(), Error> {
  let (is_success, result) = voyager_api::delete::deployment(id).await;

  match is_success {
    true => ctx.reply("Deleted successfully!").await.handle(),
    _ => {
      let errors = result.get_errs_str();
      ctx.reply(format!("Failed to delete.{errors}")).await.handle()
    }
  };

  Ok(())
}
