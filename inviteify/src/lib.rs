use time::OffsetDateTime;

#[cfg(feature = "qrcode")]
use qrcode::QrCode;

mod error;
mod http;

use error::{APIError, InviteifyError};
use http::Client;
pub use http::{ChannelInvite, ChannelInviteRequest};

static DISCORD_INVITE_BASE: &str = "https://discord.gg/";

async fn response_json<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    res: reqwest::Response,
) -> Result<T, InviteifyError> {
    if res.status().is_success() {
        Ok(res.json::<T>().await?)
    } else {
        Err(APIError::from_reqwest(res).await.into())
    }
}

pub struct Inviteify {
    http: Client,
    user_id: Option<String>,
}

impl Inviteify {
    pub fn new(client_id: &str, auth_token: &str) -> Result<Self, InviteifyError> {
        let http = Client::new(client_id, auth_token)?;

        Ok(Self {
            http,
            user_id: None,
        })
    }

    pub async fn whoami(&mut self) -> Result<String, InviteifyError> {
        // If we have already checked, we're likely fine
        if let Some(user_id) = &self.user_id {
            return Ok(user_id.clone());
        }

        let response = self.http.get("/users/@me").send().await?;
        let data = response_json::<serde_json::Value>(response).await?;

        let data = if let serde_json::Value::Object(map) = data {
            map
        } else {
            return Err(InviteifyError::AuthenticationError);
        };

        if let Some(user_id) = data.get("id") {
            let user_id = if let serde_json::Value::String(id) = user_id {
                id
            } else {
                return Err(InviteifyError::AuthenticationError);
            };

            self.user_id = Some(user_id.clone());
            Ok(user_id.clone())
        } else {
            Err(InviteifyError::AuthenticationError)
        }
    }

    pub fn authorization_link(&self, server_id: &str) -> String {
        self.http.authorize_link(server_id)
    }

    pub fn invite_link(&self, invite: &ChannelInvite) -> String {
        format!("{DISCORD_INVITE_BASE}{}", invite.code)
    }

    #[cfg(feature = "qrcode")]
    pub fn invite_qrcode(&self, invite: &ChannelInvite) -> Result<QrCode, InviteifyError> {
        QrCode::new(self.invite_link(invite)).map_err(|e| e.into())
    }

    // Generate a new invite for a given discord channel.
    //
    // Will return an invite that matches expires greater than the min_age if it exists, or will
    // create a new invite (based on max age) and return that.
    pub async fn check_and_generate_invite(
        &mut self,
        channel: &str,
        min_age: u64,
        max_age: u64,
    ) -> Result<ChannelInvite, InviteifyError> {
        // Generate QR Code
        let req = ChannelInviteRequest {
            max_age,
            ..ChannelInviteRequest::default()
        };

        // Ensure we are authenticated
        let user_id = self.whoami().await?;

        let min_age_duration = core::time::Duration::from_secs(min_age);
        let now = OffsetDateTime::now_utc();
        let min_request_expiry = now + min_age_duration;

        // Check if there are other invites created by the same user that are
        // set to expire after what we require.
        self.channel_invite_list(channel)
            .await?
            .into_iter()
            .filter(|invite| invite.inviter.id == user_id)
            .filter(|invite| invite.expires_at.unwrap_or(now) > min_request_expiry)
            .find(|_| true) //get the first element if there is one, or
            .map_or(self.channel_invite_create(channel, &req).await, |invite| {
                Ok(invite)
            })
    }

    async fn channel_invite_list(
        &self,
        channel: &str,
    ) -> Result<Vec<ChannelInvite>, InviteifyError> {
        let invites = self
            .http
            .get(&format!("/channels/{channel}/invites"))
            .send()
            .await?;

        response_json::<Vec<ChannelInvite>>(invites).await
    }

    async fn channel_invite_create(
        &self,
        channel: &str,
        req: &ChannelInviteRequest,
    ) -> Result<ChannelInvite, InviteifyError> {
        let creation_response = self
            .http
            .post(&format!("/channels/{channel}/invites"))
            .json(&req)
            .send()
            .await?;

        response_json::<ChannelInvite>(creation_response).await
    }
}
