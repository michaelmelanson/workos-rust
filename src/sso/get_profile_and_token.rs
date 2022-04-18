use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::sso::Sso;

#[derive(Debug)]
pub struct GetProfileAndTokenOptions<'a> {
    pub client_id: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub object: String,
    pub connection_type: String,
    pub idp_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetProfileAndTokenResponse {
    pub access_token: String,
    pub profile: Profile,
}

#[async_trait]
pub trait GetProfileAndToken {
    /// [WorkOS Docs: Get a Profile and Token](https://workos.com/docs/reference/sso/profile/token)
    async fn get_profile_and_token(
        &self,
        options: &GetProfileAndTokenOptions<'_>,
    ) -> Result<GetProfileAndTokenResponse, Box<dyn Error>>;
}

#[async_trait]
impl<'a> GetProfileAndToken for Sso<'a> {
    async fn get_profile_and_token(
        &self,
        options: &GetProfileAndTokenOptions<'_>,
    ) -> Result<GetProfileAndTokenResponse, Box<dyn Error>> {
        let &GetProfileAndTokenOptions { client_id, code } = options;

        let client = reqwest::Client::new();
        let url = self.workos.base_url().join("/sso/token")?;
        let params = [
            ("client_id", client_id),
            ("client_secret", self.workos.api_key()),
            ("grant_type", "authorization_code"),
            ("code", code),
        ];
        let response = client.post(url).form(&params).send().await?;
        let get_profile_and_token_response = response.json::<GetProfileAndTokenResponse>().await?;

        Ok(get_profile_and_token_response)
    }
}
