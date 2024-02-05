use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use thiserror::Error;

use crate::user_management::{User, UserManagement};
use crate::{AuthorizationCode, ClientId, WorkOsError, WorkOsResult};

/// The parameters for [`AuthenticateWithCode`].
#[derive(Debug)]
pub struct AuthenticateWithCodeParams<'a> {
    /// The client ID corresponding to the environment that SSO was initiated
    /// from.
    pub client_id: &'a ClientId,

    /// The client secret corresponding to the environment that SSO was initiated.
    pub client_secret: String,

    /// The grant type of the request.
    /// This should always be "authorization_code".
    pub grant_type: String,

    /// The authorization code that was returned from the SSO redirect.
    pub code: &'a AuthorizationCode,

    /// The IP address of the user that initiated the SSO request.
    pub ip_address: String,

    /// The user agent of the user that initiated the SSO request.
    pub user_agent: String,
}

/// The response for [`AuthenticateWithCode`].
#[derive(Debug, Deserialize)]
pub struct AuthenticateWithCodeResponse {
    /// The user's profile.
    pub user: User,

    /// The ID of the organization that the user is a member of.
    pub organization_id: Option<String>,
}

/// An error returned from [`GetProfileAndToken`].
#[derive(Debug, Error, Deserialize)]
#[error("{error}: {error_description}")]
pub struct AuthenticateWithCodeError {
    /// The error code of the error that occurred.
    pub error: String,

    /// The description of the error.
    pub error_description: String,
}

#[async_trait]
trait HandleAuthenticateWithCodeError
where
    Self: Sized,
{
    async fn handle_authenticate_with_code_error(
        self,
    ) -> WorkOsResult<Self, AuthenticateWithCodeError>;
}

#[async_trait]
impl HandleAuthenticateWithCodeError for Response {
    async fn handle_authenticate_with_code_error(
        self,
    ) -> WorkOsResult<Self, AuthenticateWithCodeError> {
        match self.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => match err.status() {
                Some(StatusCode::BAD_REQUEST) => {
                    let error = self.json::<AuthenticateWithCodeError>().await?;

                    Err(match error.error.as_str() {
                        "invalid_client" | "unauthorized_client" => WorkOsError::Unauthorized,
                        _ => WorkOsError::Operation(error),
                    })
                }
                _ => Err(WorkOsError::RequestError(err)),
            },
        }
    }
}

/// [WorkOS Docs: Authenticate with code](https://workos.com/docs/reference/user-management/authentication/code)
#[async_trait]
pub trait AuthenticateWithCode {
    /// [WorkOS Docs: Authenticate with code](https://workos.com/docs/reference/user-management/authentication/code)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos::WorkOsResult;
    /// # use workos::user_management::*;
    /// use workos::{AuthorizationCode, ApiKey, ClientId, WorkOs};
    ///
    /// # async fn run() -> WorkOsResult<(), AuthenticateWithCodeError> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let AuthenticateWithCodeResponse { user, .. } = workos
    ///     .user_management()
    ///     .authenticate_with_code(&AuthenticateWithCodeParams {
    ///         client_id: &ClientId::from("client_1234"),
    ///         client_secret: "client secret".to_string(),
    ///         grant_type: "authorization_code".to_string(),
    ///         code: &AuthorizationCode::from("code_1234"),
    ///         ip_address: "1.2.3.4".to_string(),
    ///         user_agent: "Mozilla/5.0".to_string(),
    ///     })
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn authenticate_with_code(
        &self,
        params: &AuthenticateWithCodeParams<'_>,
    ) -> WorkOsResult<AuthenticateWithCodeResponse, AuthenticateWithCodeError>;
}

#[async_trait]
impl<'a> AuthenticateWithCode for UserManagement<'a> {
    async fn authenticate_with_code(
        &self,
        params: &AuthenticateWithCodeParams<'_>,
    ) -> WorkOsResult<AuthenticateWithCodeResponse, AuthenticateWithCodeError> {
        let AuthenticateWithCodeParams {
            client_id,
            client_secret,
            grant_type,
            code,
            ip_address,
            user_agent,
        } = params;

        let url = self
            .workos
            .base_url()
            .join("/user_management/authenticate")?;
        let params = [
            ("client_id", &client_id.to_string()),
            ("client_secret", &client_secret),
            ("grant_type", &grant_type),
            ("code", &code.to_string()),
            ("ip_address", &ip_address),
            ("user_agent", &user_agent),
        ];

        let authenticate_with_code_response = self
            .workos
            .client()
            .post(url)
            .form(&params)
            .send()
            .await?
            .handle_authenticate_with_code_error()
            .await?
            .json::<AuthenticateWithCodeResponse>()
            .await?;

        Ok(authenticate_with_code_response)
    }
}

#[cfg(test)]
mod test {
    use matches::assert_matches;
    use mockito::{self, mock, Matcher};
    use serde_json::json;
    use tokio;

    use crate::{user_management::UserId, ApiKey, WorkOs, WorkOsError};

    use super::*;

    #[tokio::test]
    async fn it_calls_the_token_endpoint() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&mockito::server_url())
            .unwrap()
            .build();

        let _mock = mock("POST", "/user_management/authenticate")
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("client_id".into(), "client_1234".into()),
                Matcher::UrlEncoded("client_secret".into(), "client".into()),
                Matcher::UrlEncoded("grant_type".into(), "authorization_code".into()),
                Matcher::UrlEncoded("code".into(), "abc123".into()),
                Matcher::UrlEncoded("ip_address".into(), "1.2.3.4".into()),
                Matcher::UrlEncoded("user_agent".into(), "Mozilla/5.0".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                  "user": {
                    "object": "user",
                    "id": "user_01E4ZCR3C56J083X43JQXF3JK5",
                    "email": "marcelina.davis@example.com",
                    "first_name": "Marcelina",
                    "last_name": "Davis",
                    "email_verified": true,
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-25T19:07:33.155Z"
                  },
                  "organization_id": "org_01H945H0YD4F97JN9MATX7BYAG"
                })
                .to_string(),
            )
            .create();

        let response = workos
            .user_management()
            .authenticate_with_code(&AuthenticateWithCodeParams {
                client_id: &ClientId::from("client_1234"),
                client_secret: "client".into(),
                grant_type: "authorization_code".into(),
                code: &AuthorizationCode::from("abc123"),
                ip_address: "1.2.3.4".into(),
                user_agent: "Mozilla/5.0".into(),
            })
            .await
            .unwrap();

        assert_eq!(
            response.user.id,
            UserId::from("user_01E4ZCR3C56J083X43JQXF3JK5")
        );
        assert_eq!(response.user.email, "marcelina.davis@example.com");
        assert_eq!(response.user.first_name, "Marcelina");
        assert_eq!(response.user.last_name, "Davis");
        assert_eq!(response.user.email_verified, true);
        assert_eq!(response.user.created_at, "2021-06-25T19:07:33.155Z");
        assert_eq!(response.user.updated_at, "2021-06-25T19:07:33.155Z");
        assert_eq!(
            response.organization_id,
            Some("org_01H945H0YD4F97JN9MATX7BYAG".to_string())
        );
    }

    #[tokio::test]
    async fn it_returns_an_unauthorized_error_with_an_invalid_client() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&mockito::server_url())
            .unwrap()
            .build();

        let _mock = mock("POST", "/user_management/authenticate")
            .with_status(400)
            .with_body(
                json!({
                    "error": "invalid_client",
                    "error_description": "Invalid client ID."
                })
                .to_string(),
            )
            .create();

        let result = workos
            .user_management()
            .authenticate_with_code(&AuthenticateWithCodeParams {
                client_id: &ClientId::from("client_1234"),
                client_secret: "client".into(),
                grant_type: "authorization_code".into(),
                code: &AuthorizationCode::from("abc123"),
                ip_address: "1.2.3.4".into(),
                user_agent: "Mozilla/5.0".into(),
            })
            .await;

        assert_matches!(result, Err(WorkOsError::Unauthorized))
    }

    #[tokio::test]
    async fn it_returns_an_unauthorized_error_with_an_unauthorized_client() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&mockito::server_url())
            .unwrap()
            .build();

        let _mock = mock("POST", "/user_management/authenticate")
            .with_status(400)
            .with_body(
                json!({
                    "error": "unauthorized_client",
                    "error_description": "Unauthorized"
                })
                .to_string(),
            )
            .create();

        let result = workos
            .user_management()
            .authenticate_with_code(&AuthenticateWithCodeParams {
                client_id: &ClientId::from("client_1234"),
                client_secret: "client".into(),
                grant_type: "authorization_code".into(),
                code: &AuthorizationCode::from("abc123"),
                ip_address: "1.2.3.4".into(),
                user_agent: "Mozilla/5.0".into(),
            })
            .await;

        assert_matches!(result, Err(WorkOsError::Unauthorized))
    }

    #[tokio::test]
    async fn it_returns_an_error_when_the_authorization_code_is_invalid() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&mockito::server_url())
            .unwrap()
            .build();

        let _mock = mock("POST", "/user_management/authenticate")
            .with_status(400)
            .with_body(
                json!({
                    "error": "invalid_grant",
                    "error_description": "The code 'abc123' has expired or is invalid."
                })
                .to_string(),
            )
            .create();

        let result = workos
            .user_management()
            .authenticate_with_code(&AuthenticateWithCodeParams {
                client_id: &ClientId::from("client_1234"),
                client_secret: "client".into(),
                grant_type: "authorization_code".into(),
                code: &AuthorizationCode::from("abc123"),
                ip_address: "1.2.3.4".into(),
                user_agent: "Mozilla/5.0".into(),
            })
            .await;

        if let Err(WorkOsError::Operation(error)) = result {
            assert_eq!(error.error, "invalid_grant");
            assert_eq!(
                error.error_description,
                "The code 'abc123' has expired or is invalid."
            );
        } else {
            panic!("expected get_profile_and_token to return an error")
        }
    }
}
