use crate::{Context, Error};
use crate::utils::handle_reply::HandleReply;
use crate::commands::deployments::{new::new, list::list};

pub mod new;
pub mod list;
pub(crate) mod deployment;

#[poise::command(prefix_command, slash_command, subcommands("new", "list"))]
pub async fn deployments(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("To list all available subcommands, please use /help deployments.").await.handle();
  Ok(())
}
