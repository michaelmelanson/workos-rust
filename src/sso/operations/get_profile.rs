use std::error::Error;

use async_trait::async_trait;

use crate::sso::{Profile, Sso};

pub struct GetProfileOptions<'a> {
    pub access_token: &'a str,
}

#[async_trait]
pub trait GetProfile {
    async fn get_profile(&self, options: &GetProfileOptions<'_>)
        -> Result<Profile, Box<dyn Error>>;
}

#[async_trait]
impl<'a> GetProfile for Sso<'a> {
    async fn get_profile(
        &self,
        options: &GetProfileOptions<'_>,
    ) -> Result<Profile, Box<dyn Error>> {
        let &GetProfileOptions { access_token } = options;

        let client = reqwest::Client::new();
        let url = self.workos.base_url().join("/sso/profile")?;
        let response = client.get(url).bearer_auth(access_token).send().await?;
        let profile = response.json::<Profile>().await?;

        Ok(profile)
    }
}
