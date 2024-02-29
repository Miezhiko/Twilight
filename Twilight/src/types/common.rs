use std::sync::Arc;

use serde::Deserialize;

use twilight_http::Client as HttpClient;
use reqwest::Client as Reqwest;

#[derive(Clone, Deserialize, Debug)]
pub struct IOptions {
  pub discord: String
}

#[derive(Debug)]
pub struct StateRef {
  pub http: HttpClient,
  pub request_client: Reqwest
}

pub type State = Arc<StateRef>;
