use url::{ParseError, Url};

pub struct WorkOs {
    base_url: Url,
    api_key: String,
}

impl WorkOs {
    pub fn new(url: &str, api_key: &str) -> Result<Self, ParseError> {
        let base_url = Url::parse(url)?;

        Ok(Self {
            base_url,
            api_key: api_key.into(),
        })
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn api_key(&self) -> &String {
        &self.api_key
    }
}
