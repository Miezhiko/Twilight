use crate::types::{
  common::State,
  gentoo::Bugs
};

use std::{
  str::FromStr,
  error::Error,
  sync::Arc,
  future::Future
};

use twilight_gateway::Event;

use twilight_model::{
  channel::Message,
  util::Timestamp
};

use twilight_util::builder::embed::{
  EmbedBuilder,
  EmbedFieldBuilder,
  EmbedFooterBuilder
};

fn spawn(fut: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
  tokio::spawn(async move {
    if let Err(why) = fut.await {
      tracing::debug!("handler error: {why:?}");
    }
  });
}

async fn help(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "wiki command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );
  state
    .http
    .create_message(msg.channel_id)
    .reply(msg.id)
    .content("I can handle -bug <num> command, maybe LOL")
    .await?;
  Ok(())
}

async fn bug(msg: Message, state: State) -> anyhow::Result<()> {
  tracing::debug!(
    "bug command in channel {} by {}",
    msg.channel_id,
    msg.author.name
  );
  let mut bug_number = -1;
  for if_num in msg.content.split_whitespace() {
    let try_num = if_num.parse::<i32>();
    match try_num {
      Ok(num) => {
        bug_number = num;
        break;
      }, Err(_) => {
        continue;
      }
    }
  }
  if bug_number < 0 {
    state
      .http
      .create_message(msg.channel_id)
      .reply(msg.id)
      .content("You need to provide bug number to search for")
      .await?;
    return Ok(())
  }
  let res = state
            .request_client
            .get(&format!("https://bugs.gentoo.org/rest/bug?id={bug_number}")).send().await?;

  let bugs: Bugs = res.json().await?;
  if let Some(bug) = bugs.bugs.first() {
    let mut e = EmbedBuilder::new()
      .title(&bug.summary)
      .url(format!("https://bugs.gentoo.org/{bug_number}"))
      .color(0xfd_69_b3)
      .field(EmbedFieldBuilder::new("1", "3").inline())
      .field(EmbedFieldBuilder::new("2", "4").inline())
      .footer(EmbedFooterBuilder::new(&format!("Requested by {}", msg.author.name)));
    if !bug.creation_time.is_empty() {
      if let Ok(dt) = Timestamp::from_str(&bug.creation_time) {
        e = e.timestamp(dt);
      }
    }
    if !bug.creator.is_empty() {
      e = e.field(EmbedFieldBuilder::new("creator", &bug.creator).inline());
    }
    if !bug.priority.is_empty() {
      e = e.field(EmbedFieldBuilder::new("priority", &bug.priority).inline());
    }
    if !bug.severity.is_empty() {
      e = e.field(EmbedFieldBuilder::new("severity", &bug.severity).inline());
    }
    if !bug.product.is_empty() {
      e = e.field(EmbedFieldBuilder::new("product", &bug.product).inline());
    }
    if !bug.resolution.is_empty() {
      e = e.field(EmbedFieldBuilder::new("resolution", &bug.resolution).inline());
    }
    if !bug.status.is_empty() {
      e = e.field(EmbedFieldBuilder::new("status", &bug.status).inline());
    }
    let embed = e.validate()?.build();
    state
      .http
      .create_message(msg.channel_id)
      .reply(msg.id)
      .embeds(&[embed])
      .await?;
  } else {
    state
      .http
      .create_message(msg.channel_id)
      .reply(msg.id)
      .content(&format!("Can't find bug {bug_number}"))
      .await?;
  }
  Ok(())
}

async fn wiki(msg: Message, state: State) -> anyhow::Result<()> {
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

pub async fn handle_event(
  event: Event,
  state: State,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  if let Event::MessageCreate(msg) = event {
    if msg.guild_id.is_none() || !msg.content.starts_with('!') {
      {};
    }
    match msg.content.split_whitespace().next() {
      Some("-help") => spawn(help(msg.0, Arc::clone(&state))),
      Some("-bug")  => spawn(bug(msg.0, Arc::clone(&state))),
      Some("-wiki") => spawn(wiki(msg.0, Arc::clone(&state))),
      Some(_)       => {}
      None          => {}
    }
  }

  Ok(())
}
