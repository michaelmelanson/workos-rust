use url::{ParseError, Url};

use crate::WorkOs;

#[derive(Debug)]
pub enum Provider {
    GoogleOauth,
    MicrosoftOauth,
}

#[derive(Debug)]
pub enum ConnectionSelector<'a> {
    Connection(&'a str),
    Organization(&'a str),
    Provider(&'a Provider),
}

#[derive(Debug)]
pub struct GetAuthorizationUrlOptions<'a> {
    pub client_id: &'a str,
    pub redirect_uri: &'a str,
    pub connection_selector: ConnectionSelector<'a>,
    pub state: Option<&'a str>,
}

pub struct Sso<'a> {
    workos: &'a WorkOs,
}

impl<'a> Sso<'a> {
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }

    /// Returns an authorization URL to use to initiate SSO.
    ///
    /// [WorkOS Docs: Get Authorization URL](https://workos.com/docs/reference/sso/authorize/get)
    pub fn get_authorization_url(
        &self,
        options: &GetAuthorizationUrlOptions,
    ) -> Result<Url, ParseError> {
        let GetAuthorizationUrlOptions {
            connection_selector,
            client_id,
            redirect_uri,
            state,
        } = options;

        let query = {
            let mut query_params: querystring::QueryParams = vec![
                ("response_type", "code"),
                ("client_id", client_id),
                ("redirect_uri", redirect_uri),
                match connection_selector {
                    ConnectionSelector::Connection(connection_id) => ("connection", connection_id),
                    ConnectionSelector::Organization(organization_id) => {
                        ("organization", organization_id)
                    }
                    ConnectionSelector::Provider(provider) => (
                        "provider",
                        match provider {
                            Provider::GoogleOauth => "GoogleOAuth",
                            Provider::MicrosoftOauth => "MicrosoftOauth",
                        },
                    ),
                },
            ];

            if let Some(state) = state {
                query_params.push(("state", state));
            }
            String::from(querystring::stringify(query_params).trim_end_matches('&'))
        };

        self.workos
            .base_url()
            .join(&format!("/sso/authorize?{}", query))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_builds_an_authorization_url_when_given_a_connection_id() {
        let workos = WorkOs::new("https://api.workos.com", "sk_example_123456789").unwrap();
        let workos_sso = Sso::new(&workos);

        let authorization_url = workos_sso
            .get_authorization_url(&GetAuthorizationUrlOptions {
                client_id: "client_123456789",
                redirect_uri: "https://your-app.com/callback",
                connection_selector: ConnectionSelector::Connection("conn_1234"),
                state: None,
            })
            .unwrap();

        assert_eq!(
            authorization_url,
            Url::parse(
                "https://api.workos.com/sso/authorize?response_type=code&client_id=client_123456789&redirect_uri=https://your-app.com/callback&connection=conn_1234"
            )
            .unwrap()
        )
    }

    #[test]
    fn it_builds_an_authorization_url_when_given_an_organization_id() {
        let workos = WorkOs::new("https://api.workos.com", "sk_example_123456789").unwrap();
        let workos_sso = Sso::new(&workos);

        let authorization_url = workos_sso
            .get_authorization_url(&GetAuthorizationUrlOptions {
                client_id: "client_123456789",
                redirect_uri: "https://your-app.com/callback",
                connection_selector: ConnectionSelector::Organization("org_1234"),
                state: None,
            })
            .unwrap();

        assert_eq!(
            authorization_url,
            Url::parse(
                "https://api.workos.com/sso/authorize?response_type=code&client_id=client_123456789&redirect_uri=https://your-app.com/callback&organization=org_1234"
            )
            .unwrap()
        )
    }

    #[test]
    fn it_builds_an_authorization_url_when_given_a_provider() {
        let workos = WorkOs::new("https://api.workos.com", "sk_example_123456789").unwrap();
        let workos_sso = Sso::new(&workos);

        let authorization_url = workos_sso
            .get_authorization_url(&GetAuthorizationUrlOptions {
                client_id: "client_123456789",
                redirect_uri: "https://your-app.com/callback",
                connection_selector: ConnectionSelector::Provider(&Provider::GoogleOauth),
                state: None,
            })
            .unwrap();

        assert_eq!(
            authorization_url,
            Url::parse(
                "https://api.workos.com/sso/authorize?response_type=code&client_id=client_123456789&redirect_uri=https://your-app.com/callback&provider=GoogleOAuth"
            )
            .unwrap()
        )
    }
}
