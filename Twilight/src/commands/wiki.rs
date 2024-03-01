use crate::types::common::State;

use twilight_model::channel::Message;

pub async fn wiki(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "wiki command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );
  state
    .http
    .create_message(msg.channel_id)
    .reply(msg.id)
    .content("Not yet implemented here! Will do later, use ~wiki and Amadeus will respond")
    .await?;
  Ok(())
}
