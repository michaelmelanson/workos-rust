use url::{ParseError, Url};

use crate::organizations::OrganizationId;
use crate::sso::{ClientId, ConnectionId, Sso};

#[derive(Debug)]
pub enum Provider {
    GoogleOauth,
    MicrosoftOauth,
}

#[derive(Debug)]
pub enum ConnectionSelector<'a> {
    Connection(&'a ConnectionId),
    Organization(&'a OrganizationId),
    Provider(&'a Provider),
}

#[derive(Debug)]
pub struct GetAuthorizationUrlOptions<'a> {
    pub client_id: &'a ClientId,
    pub redirect_uri: &'a str,
    pub connection_selector: ConnectionSelector<'a>,
    pub state: Option<&'a str>,
}

pub trait GetAuthorizationUrl {
    /// Returns an authorization URL to use to initiate SSO.
    ///
    /// [WorkOS Docs: Get Authorization URL](https://workos.com/docs/reference/sso/authorize/get)
    fn get_authorization_url(
        &self,
        options: &GetAuthorizationUrlOptions,
    ) -> Result<Url, ParseError>;
}

impl<'a> GetAuthorizationUrl for Sso<'a> {
    fn get_authorization_url(
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
            let client_id = client_id.to_string();

            let connection_selector_param = match connection_selector {
                ConnectionSelector::Connection(connection_id) => {
                    ("connection", connection_id.to_string())
                }
                ConnectionSelector::Organization(organization_id) => {
                    ("organization", organization_id.to_string())
                }
                ConnectionSelector::Provider(provider) => (
                    "provider",
                    match provider {
                        Provider::GoogleOauth => "GoogleOAuth".to_string(),
                        Provider::MicrosoftOauth => "MicrosoftOAuth".to_string(),
                    },
                ),
            };

            let mut query_params: querystring::QueryParams = vec![
                ("response_type", "code"),
                ("client_id", &client_id),
                ("redirect_uri", redirect_uri),
                (connection_selector_param.0, &connection_selector_param.1),
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
    use crate::WorkOs;

    use super::*;

    #[test]
    fn it_builds_an_authorization_url_when_given_a_connection_id() {
        let workos = WorkOs::new("sk_example_123456789");
        let workos_sso = Sso::new(&workos);

        let authorization_url = workos_sso
            .get_authorization_url(&GetAuthorizationUrlOptions {
                client_id: &ClientId::from("client_123456789"),
                redirect_uri: "https://your-app.com/callback",
                connection_selector: ConnectionSelector::Connection(&ConnectionId::from(
                    "conn_1234",
                )),
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
        let workos = WorkOs::new("sk_example_123456789");
        let workos_sso = Sso::new(&workos);

        let authorization_url = workos_sso
            .get_authorization_url(&GetAuthorizationUrlOptions {
                client_id: &ClientId::from("client_123456789"),
                redirect_uri: "https://your-app.com/callback",
                connection_selector: ConnectionSelector::Organization(&OrganizationId::from(
                    "org_1234",
                )),
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
        let workos = WorkOs::new("sk_example_123456789");
        let workos_sso = Sso::new(&workos);

        let authorization_url = workos_sso
            .get_authorization_url(&GetAuthorizationUrlOptions {
                client_id: &ClientId::from("client_123456789"),
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