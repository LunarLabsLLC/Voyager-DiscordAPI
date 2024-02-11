#!forbid(unsafe_code)
mod commands;
mod services;
mod types;
mod utils;

use std::env::var;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GatewayIntents;
use tracing::{event, Level};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data { }

#[tokio::main]
async fn main() {
  // Logging
  std::env::set_var("RUST_LOG", "info");
  std::env::set_var("RUST_BACKTRACE", "1");
  std::env::set_var("RUST_LIB_BACKTRACE", "full");
  tracing_subscriber::fmt::init();
  color_eyre::install().unwrap();

    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
      commands: vec![help::help(), r#mod::invite(), deployments::deploy::deploy()],

      prefix_options: poise::PrefixFrameworkOptions {
          prefix: Some("~".into()),
          ..Default::default()
      },

      // The global error handler for all error cases that may occur
      on_error: |error| Box::pin(on_error(error)),

      // // This code is run before every command
      // pre_command: |ctx| {
      //     Box::pin(async move {
      //         println!("Executing command {}... ", ctx.command().qualified_name);
      //     })
      // },
      //
      // // This code is run after a command if it was successful (returned Ok)
      // post_command: |_ctx| {
      //     Box::pin(async move {
      //         println!("...success.");
      //     })
      // },
      //
      // event_handler: |_ctx, event, _framework, _data| {
      //     Box::pin(async move {
      //         println!(
      //             "Got an event in event handler: {:?}",
      //             event.snake_case_name()
      //         );
      //         Ok(())
      //     })
      // },

      ..Default::default()
    };

    let framework = poise::Framework::builder()
      .setup(move |ctx, _ready, framework| {
        Box::pin(async move {
          event!(Level::INFO, "Logged in as {}", _ready.user.name);
          poise::builtins::register_globally(ctx, &framework.options().commands).await?;
          Ok(Data {})
        })
      })
      .options(options)
      .build();

    dotenv().expect("Failed to load .env file");
    let token = var("DISCORD_TOKEN")
      .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents = GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
      .framework(framework)
      .await.expect("");

    client.start().await.expect("");
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
  match error {
    poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
    poise::FrameworkError::Command { error, ctx, .. } => {
      event!(Level::ERROR, "Error in command `{}`: {:?}", ctx.command().name, error,);
    }
    error => {
        if let Err(e) = poise::builtins::on_error(error).await {
          event!(Level::ERROR, "Error while handling error: {}", e)
        }
    }
  }
}
