use crate::{Context, Error};
use crate::services::shared_api::send_invite::send_invite;
use crate::utils::handle_reply::HandleReply;

/// Sends an invitation token to the given email
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn invite(
  ctx: Context<'_>,
  #[description = "The email to get the token for"]
  email: String,
) -> Result<(), Error> {
  let (is_success, _) = send_invite(email).await;

  match is_success {
    true => ctx.reply("Token successfully sent to email.").await.handle(),
    _ => ctx.reply("Failed to send token.").await.handle()
  };

  Ok(())
}
