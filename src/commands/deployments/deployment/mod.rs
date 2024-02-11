use crate::{Context, Error};
use crate::utils::handle_reply::HandleReply;
use crate::commands::deployments::deployment::{logs::logs};

pub(crate) mod get;
pub(crate) mod logs;
pub(crate) mod delete;

#[poise::command(prefix_command, slash_command, subcommands("logs"))]
pub async fn deployment(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("To list all available subcommands, please use /help deployments.").await.handle();
  Ok(())
}
