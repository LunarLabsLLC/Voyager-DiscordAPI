use poise::{ReplyHandle, serenity_prelude};
use tracing::{event, Level};

pub trait HandleReply {
  fn handle(&self);
}
impl<'a> HandleReply for Result<ReplyHandle<'a>, serenity_prelude::Error> {
  fn handle(&self) {
    if let Some(e) = self.as_ref().err() {
      event!(Level::ERROR, "Error sending reply: {:?}", e);
    }
  }
}
