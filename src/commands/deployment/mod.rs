use crate::{Context, Error};
use crate::utils::handle_reply::HandleReply;
use crate::commands::deployment::{create::create, delete::delete, get::get, logs::logs};

pub mod get;
pub mod logs;
pub mod delete;
mod create;

// Commands for creating and managing deployments
#[poise::command(slash_command, subcommands("create", "delete", "get", "logs"))]
pub async fn deployment(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("To list all available subcommands, please use /help deployment.").await.handle();
  Ok(())
}
