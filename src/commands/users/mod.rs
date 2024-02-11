mod invite;

use crate::{Context, Error};
use crate::utils::handle_reply::HandleReply;
use crate::commands::users::{invite::invite};

#[poise::command(prefix_command, slash_command, subcommands("invite"))]
pub async fn users(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("To list all available subcommands, please use /help users.").await.handle();
  Ok(())
}
