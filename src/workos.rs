use url::{ParseError, Url};

use crate::sso::Sso;

pub struct WorkOs {
    base_url: Url,
    api_key: String,
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
        WorkOs {
            base_url: self.base_url,
            api_key: self.api_key.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
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
}
