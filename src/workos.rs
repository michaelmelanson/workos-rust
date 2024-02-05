use url::{ParseError, Url};

use crate::admin_portal::AdminPortal;
use crate::directory_sync::DirectorySync;
use crate::mfa::Mfa;
use crate::organizations::Organizations;
use crate::passwordless::Passwordless;
use crate::sso::Sso;
use crate::user_management::UserManagement;
use crate::ApiKey;

/// The WorkOS client.
pub struct WorkOs {
    base_url: Url,
    key: ApiKey,
    client: reqwest::Client,
}

impl WorkOs {
    /// Returns a new instance of the WorkOS client using the provided API key.
    pub fn new(key: &ApiKey) -> Self {
        WorkOsBuilder::new(key).build()
    }

    /// Returns a [`WorkOsBuilder`] that may be used to construct a WorkOS client.
    pub fn builder(key: &ApiKey) -> WorkOsBuilder {
        WorkOsBuilder::new(key)
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn key(&self) -> &ApiKey {
        &self.key
    }

    pub(crate) fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Returns an [`AdminPortal`] instance.
    pub fn admin_portal(&self) -> AdminPortal {
        AdminPortal::new(self)
    }

    /// Returns a [`DirectorySync`] instance.
    pub fn directory_sync(&self) -> DirectorySync {
        DirectorySync::new(self)
    }

    /// Returns an [`Mfa`] instance.
    pub fn mfa(&self) -> Mfa {
        Mfa::new(self)
    }

    /// Returns an [`Organizations`] instance.
    pub fn organizations(&self) -> Organizations {
        Organizations::new(self)
    }

    /// Returns a [`Passwordless`] instance.
    pub fn passwordless(&self) -> Passwordless {
        Passwordless::new(self)
    }

    /// Returns an [`Sso`] instance.
    pub fn sso(&self) -> Sso {
        Sso::new(self)
    }

    /// Returns a [`UserManagement`] instance.
    pub fn user_management(&self) -> UserManagement {
        UserManagement::new(self)
    }
}

/// A builder for a WorkOS client.
pub struct WorkOsBuilder<'a> {
    base_url: Url,
    key: &'a ApiKey,
}

impl<'a> WorkOsBuilder<'a> {
    /// Returns a new [`WorkOsBuilder`] using the provided API key.
    pub fn new(key: &'a ApiKey) -> Self {
        Self {
            base_url: Url::parse("https://api.workos.com").unwrap(),
            key,
        }
    }

    /// Sets the base URL of the WorkOS API that the client should point to.
    pub fn base_url(mut self, base_url: &'a str) -> Result<WorkOsBuilder, ParseError> {
        self.base_url = Url::parse(base_url)?;
        Ok(self)
    }

    /// Sets the API key that the client will use.
    pub fn key(mut self, key: &'a ApiKey) -> Self {
        self.key = key;
        self
    }

    /// Consumes the builder and returns the constructed client.
    pub fn build(self) -> WorkOs {
        let client = reqwest::Client::builder()
            .user_agent(concat!("workos-rust/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap();

        WorkOs {
            base_url: self.base_url,
            key: self.key.to_owned(),
            client,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_supports_setting_the_base_url_through_the_builder() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url("https://auth.your-app.com")
            .unwrap()
            .build();

        assert_eq!(
            workos.base_url(),
            &Url::parse("https://auth.your-app.com").unwrap()
        )
    }

    #[test]
    fn it_supports_setting_the_api_key_through_the_builder() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .key(&ApiKey::from("sk_another_api_key"))
            .build();

        assert_eq!(workos.key(), &ApiKey::from("sk_another_api_key"))
    }

    #[tokio::test]
    async fn it_sets_the_user_agent_header_on_the_client() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/health")
            .match_header(
                "User-Agent",
                concat!("workos-rust/", env!("CARGO_PKG_VERSION")),
            )
            .with_status(200)
            .with_body("User-Agent correctly set")
            .create();

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let url = workos.base_url().join("/health").unwrap();
        let response = workos.client().get(url).send().await.unwrap();
        let response_body = response.text().await.unwrap();

        assert_eq!(response_body, "User-Agent correctly set")
    }
}
