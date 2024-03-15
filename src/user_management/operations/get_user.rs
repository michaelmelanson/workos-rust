use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use thiserror::Error;

use crate::user_management::{User, UserManagement};
use crate::{WorkOsError, WorkOsResult};

/// The parameters for [`GetUser`].
#[derive(Debug)]
pub struct GetUserParams<'a> {
    /// The user's ID.
    pub user_id: &'a str,
}

/// The response for [`GetUser`].
#[derive(Debug, Deserialize)]
pub struct GetUserResponse {
    /// The user's profile.
    #[serde(flatten)]
    pub user: User,
}

/// An error returned from [`GetProfileAndToken`].
#[derive(Debug, Error, Deserialize)]
#[error("{error}: {error_description}")]
pub struct GetUserError {
    /// The error code of the error that occurred.
    pub error: String,

    /// The description of the error.
    pub error_description: String,
}

#[async_trait]
trait HandleGetUserError
where
    Self: Sized,
{
    async fn handle_get_user_error(self) -> WorkOsResult<Self, GetUserError>;
}

#[async_trait]
impl HandleGetUserError for Response {
    async fn handle_get_user_error(self) -> WorkOsResult<Self, GetUserError> {
        match self.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => match err.status() {
                Some(StatusCode::NOT_FOUND) => {
                    let body = self.text().await?;

                    Err(WorkOsError::Operation(GetUserError {
                        error: "not_found".to_string(),
                        error_description: body,
                    }))
                }

                _ => Err(WorkOsError::RequestError(err)),
            },
        }
    }
}

/// [WorkOS Docs: Get user](https://workos.com/docs/reference/user-management/user/get)
#[async_trait]
pub trait GetUser {
    /// [WorkOS Docs: Get ser](https://workos.com/docs/reference/user-management/user/get)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos::WorkOsResult;
    /// # use workos::user_management::*;
    /// use workos::{AuthorizationCode, ApiKey, ClientId, WorkOs};
    ///
    /// # async fn run() -> WorkOsResult<(), GetUserError> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let GetUserResponse { user, .. } = workos
    ///     .user_management()
    ///     .get_user(&GetUserParams {
    ///         user_id: "user_0c2f3b4d5e6f7g8h9i0j1k2l3",
    ///     })
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_user(
        &self,
        params: &GetUserParams<'_>,
    ) -> WorkOsResult<GetUserResponse, GetUserError>;
}

#[async_trait]
impl<'a> GetUser for UserManagement<'a> {
    async fn get_user(
        &self,
        params: &GetUserParams<'_>,
    ) -> WorkOsResult<GetUserResponse, GetUserError> {
        let GetUserParams { user_id } = params;

        let url = self
            .workos
            .base_url()
            .join(&format!("/user_management/users/{user_id}"))?;

        let request = self.workos.client().get(url).bearer_auth(self.workos.key());
        let get_user_response = request
            .send()
            .await?
            .handle_get_user_error()
            .await?
            .json::<GetUserResponse>()
            .await?;

        Ok(get_user_response)
    }
}

#[cfg(test)]
mod test {
    use matches::assert_matches;
    use mockito;
    use serde_json::json;
    use tokio;

    use crate::{user_management::UserId, ApiKey, WorkOs, WorkOsError};

    use super::*;

    #[tokio::test]
    async fn it_calls_the_endpoint() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock(
                "GET",
                "/user_management/users/user_0c2f3b4d5e6f7g8h9i0j1k2l3",
            )
            .with_status(200)
            .with_body(
                json!({
                    "object": "user",
                    "id": "user_0c2f3b4d5e6f7g8h9i0j1k2l3",
                    "email": "marcelina.davis@example.com",
                    "first_name": "Marcelina",
                    "last_name": "Davis",
                    "email_verified": true,
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-25T19:07:33.155Z"
                })
                .to_string(),
            )
            .create();

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let response = workos
            .user_management()
            .get_user(&GetUserParams {
                user_id: "user_0c2f3b4d5e6f7g8h9i0j1k2l3",
            })
            .await
            .unwrap();

        assert_eq!(
            response.user.id,
            UserId::from("user_0c2f3b4d5e6f7g8h9i0j1k2l3")
        );
        assert_eq!(response.user.email, "marcelina.davis@example.com");
        assert_eq!(response.user.first_name, "Marcelina");
        assert_eq!(response.user.last_name, "Davis");
        assert_eq!(response.user.email_verified, true);
        assert_eq!(response.user.created_at, "2021-06-25T19:07:33.155Z");
        assert_eq!(response.user.updated_at, "2021-06-25T19:07:33.155Z");
    }

    #[tokio::test]
    async fn it_returns_a_not_found_error() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock(
                "GET",
                "/user_management/users/user_0c2f3b4d5e6f7g8h9i0j1k2l3",
            )
            .with_status(404)
            .with_body(
                json!({
                    "error": "not_found",
                    "error_description": "No such user."
                })
                .to_string(),
            )
            .create();

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let result = workos
            .user_management()
            .get_user(&GetUserParams {
                user_id: "user_0c2f3b4d5e6f7g8h9i0j1k2l3",
            })
            .await;

        assert_matches!(result, Err(WorkOsError::Operation(_)))
    }
}
