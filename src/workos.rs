use url::{ParseError, Url};

use crate::{organizations::Organizations, sso::Sso};

pub struct WorkOs {
    base_url: Url,
    api_key: String,
    client: reqwest::Client,
}

impl WorkOs {
    pub fn new(api_key: &str) -> Self {
        WorkOsBuilder::new(api_key).build()
    }

    pub fn builder(api_key: &str) -> WorkOsBuilder {
        WorkOsBuilder::new(api_key)
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn api_key(&self) -> &String {
        &self.api_key
    }

    pub(crate) fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn organizations(&self) -> Organizations {
        Organizations::new(self)
    }

    pub fn sso(&self) -> Sso {
        Sso::new(self)
    }
}

pub struct WorkOsBuilder<'a> {
    base_url: Url,
    api_key: &'a str,
}

impl<'a> WorkOsBuilder<'a> {
    pub fn new(api_key: &'a str) -> Self {
        Self {
            base_url: Url::parse("https://api.workos.com").unwrap(),
            api_key,
        }
    }

    pub fn base_url(mut self, base_url: &'a str) -> Result<WorkOsBuilder, ParseError> {
        self.base_url = Url::parse(base_url)?;
        Ok(self)
    }

    pub fn api_key(mut self, api_key: &'a str) -> Self {
        self.api_key = api_key;
        self
    }

    pub fn build(self) -> WorkOs {
        let client = reqwest::Client::builder()
            .user_agent(concat!("workos-rust/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap();

        WorkOs {
            base_url: self.base_url,
            api_key: self.api_key.to_owned(),
            client,
        }
    }
}

#[cfg(test)]
mod test {
    use mockito::mock;

    use super::*;

    #[test]
    fn it_supports_setting_the_base_url_through_the_builder() {
        let workos = WorkOs::builder("sk_example_123456789")
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
        let workos = WorkOs::builder("sk_example_123456789")
            .api_key("sk_another_api_key")
            .build();

        assert_eq!(workos.api_key(), "sk_another_api_key")
    }

    #[tokio::test]
    async fn it_sets_the_user_agent_header_on_the_client() {
        let workos = WorkOs::builder(&"sk_example_123456789")
            .base_url(&mockito::server_url())
            .unwrap()
            .build();

        let _mock = mock("GET", "/health")
            .match_header(
                "User-Agent",
                concat!("workos-rust/", env!("CARGO_PKG_VERSION")),
            )
            .with_status(200)
            .with_body("User-Agent correctly set")
            .create();

        let url = workos.base_url().join("/health").unwrap();
        let response = workos.client().get(url).send().await.unwrap();
        let response_body = response.text().await.unwrap();

        assert_eq!(response_body, "User-Agent correctly set")
    }
}
