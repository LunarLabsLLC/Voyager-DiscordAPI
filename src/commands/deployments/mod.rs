use crate::{Context, Error};
use crate::utils::handle_reply::HandleReply;
use crate::commands::deployments::{list::list};

pub mod list;

#[poise::command(prefix_command, slash_command, subcommands("list"))]
pub async fn deployments(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("To list all available subcommands, please use /help deployments.").await.handle();
  Ok(())
}
