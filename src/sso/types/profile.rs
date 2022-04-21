use serde::{Deserialize, Serialize};

use crate::KnownOrUnknown;

use super::ConnectionType;

/// [WorkOS Docs: Profile](https://workos.com/docs/reference/sso/profile)
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub connection_type: KnownOrUnknown<ConnectionType, String>,
    pub idp_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
