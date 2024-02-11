use poise::{ReplyHandle, serenity_prelude};
use tracing::{event, Level};

pub trait HandleReply {
  fn handle(&self);
}
impl<'a> HandleReply for Result<ReplyHandle<'a>, serenity_prelude::Error> {
  fn handle(&self) {
    self.err().map(|e| event!(Level::WARN, "Warning: Failed to send reply. Error: {:?}", e));
  }
}
