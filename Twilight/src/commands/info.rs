use crate::{
  types::common::State,
  db::sled_info
};

use twilight_model::channel::Message;

pub async fn register(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "register command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );

  let mut key = String::new();
  for if_str in msg.content.split_whitespace() {
    if if_str.len() > 1 {
      if if_str != "-register" {
        key = if_str.to_string();
        break;
      }
    }
  }

  if key.is_empty() {
    state
      .http
      .create_message(msg.channel_id)
      .reply(msg.id)
      .content("You need to provide something to register")
      .await?;
    return Ok(())
  }

  let skip_len = "-register".len() + key.len() + 2;
  let rest = &msg.content[skip_len..msg.content.len()];

  if let Err(why) = sled_info::store(&key, rest).await {
    tracing::error!("Failed to register {key}, {why}");
  } else {
    state
      .http
      .create_message(msg.channel_id)
      .reply(msg.id)
      .content(&format!("Registered {key}"))
      .await?;
  }

  Ok(())
}

pub async fn show(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "show command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );

  let mut key = String::new();
  for if_str in msg.content.split_whitespace() {
    if if_str.len() > 1 {
      if if_str != "-show" {
        key = if_str.to_string();
        break;
      }
    }
  }

  if key.is_empty() {
    state
      .http
      .create_message(msg.channel_id)
      .reply(msg.id)
      .content("You need to provide something to show")
      .await?;
    return Ok(())
  }

  match sled_info::read(&key).await {
    Ok(val) => {
      state
        .http
        .create_message(msg.channel_id)
        .reply(msg.id)
        .content(&val)
        .await?;
    }, Err(why) => {
      tracing::error!("Failed to get {key}, {why}");
    }
  }

  Ok(())
}

pub async fn list(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "list command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );

  match sled_info::list().await {
    Ok(val) => {
      state
        .http
        .create_message(msg.channel_id)
        .reply(msg.id)
        .content(&val)
        .await?;
    }, Err(why) => {
      tracing::error!("Failed to show info list, {why}");
    }
  }

  Ok(())
}

pub async fn delete(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "delete command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );

  let mut key = String::new();
  for if_str in msg.content.split_whitespace() {
    if if_str.len() > 1 {
      if if_str != "-delete" {
        key = if_str.to_string();
        break;
      }
    }
  }

  match sled_info::delete(&key).await {
    Ok(_) => {
      state
        .http
        .create_message(msg.channel_id)
        .reply(msg.id)
        .content(&format!("Deleted {key}"))
        .await?;
    }, Err(why) => {
      tracing::error!("Failed to delete {key}, {why}");
    }
  }

  Ok(())
}
