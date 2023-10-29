use reqwest::{header::HeaderMap, ClientBuilder, RequestBuilder, Result};
use serde::{Deserialize, Serialize};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
static DOMAIN: &str = "https://discord.com";
static API_BASE: &str = "https://discord.com/api/v10";

static PERM_CREATE_INVITE: usize = 0x1;
static PERM_MANAGE_CHANNELS: usize = 0x10;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ChannelInvite {
    pub code: String,
    #[serde(with = "time::serde::iso8601::option")]
    pub expires_at: Option<time::OffsetDateTime>,
    pub inviter: User,
}

#[derive(Serialize)]
pub struct ChannelInviteRequest {
    pub max_age: u64,
    pub max_uses: usize,
    pub temporary: bool,
    pub unique: bool,
}

impl Default for ChannelInviteRequest {
    fn default() -> Self {
        Self {
            max_age: 86400,
            max_uses: 0,
            temporary: false,
            unique: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Client {
    client_id: String,
    client: reqwest::Client,
}

impl Client {
    pub(crate) fn new(client_id: &str, auth_token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        let authorization_header = format!("Bot {auth_token}").parse().unwrap();
        headers.insert("Authorization", authorization_header);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(APP_USER_AGENT)
            .https_only(true)
            .build()?;

        Ok(Self {
            client_id: client_id.to_string(),
            client,
        })
    }

    pub(crate) fn authorize_link(&self, server_id: &str) -> String {
        format!(
            "{DOMAIN}/oauth2/authorize?client_id={}&scope=bot&permission={}&guild_id={server_id}",
            self.client_id,
            PERM_CREATE_INVITE & PERM_MANAGE_CHANNELS
        )
    }

    pub(crate) fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(format!("{API_BASE}{url}"))
    }

    pub(crate) fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(format!("{API_BASE}{url}"))
    }
}
