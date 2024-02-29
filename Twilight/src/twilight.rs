use crate::types::common::{ IOptions, StateRef };
use crate::handler::handle_event;

use std::sync::Arc;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;

pub async fn run(opts: IOptions) -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  let mut shard = Shard::new(
    ShardId::ONE,
    opts.discord.clone(),
    Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
  );

  let http = HttpClient::new(opts.discord);
  let cache = DefaultInMemoryCache::builder()
                .resource_types(ResourceType::MESSAGE)
                .build();

   let request_client = reqwest::Client::builder()
                .pool_max_idle_per_host(0)
                .build()?;

  let state = Arc::new(StateRef { http, request_client });

  while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
    let Ok(event) = item else {
      tracing::warn!(source = ?item.unwrap_err(), "error receiving event");
      continue;
    };

    cache.update(&event);
    tokio::spawn(handle_event(event, Arc::clone(&state)));
  }

  Ok(())
}
