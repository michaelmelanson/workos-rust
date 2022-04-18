use serde::{Deserialize, Serialize};

/// [WorkOS Docs: Profile](https://workos.com/docs/reference/sso/profile)
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub object: String,
    pub connection_type: String,
    pub idp_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
