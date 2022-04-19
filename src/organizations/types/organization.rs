use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrganizationId(String);

impl Display for OrganizationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OrganizationId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for OrganizationId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
