use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The ID of a [`User`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserId(String);

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for UserId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// [WorkOS Docs: User](https://workos.com/docs/reference/user-management/user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The ID of the profile.
    pub id: UserId,

    /// The email address of the user.
    pub email: String,

    /// The first name of the user.
    pub first_name: String,

    /// The last name of the user.
    pub last_name: String,

    /// Whether the user's email address has been verified.
    pub email_verified: bool,

    /// The user's profile picture URL.
    pub profile_picture_url: Option<String>,

    /// The date and time the user was created.
    pub created_at: String,

    /// The date and time the user was last updated.
    pub updated_at: String,
}
