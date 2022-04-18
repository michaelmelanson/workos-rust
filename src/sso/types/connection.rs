use serde::{Deserialize, Serialize};

use crate::sso::ConnectionType;
use crate::KnownOrUnknown;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    Active,
    Inactive,
}

/// [WorkOS Docs: Connection](https://workos.com/docs/reference/sso/connection)
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    pub object: String,
    pub id: String,
    pub organization_id: Option<String>,

    #[serde(rename = "connection_type")]
    pub r#type: KnownOrUnknown<ConnectionType, String>,
    pub name: String,
    pub state: ConnectionState,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{sso::ConnectionType, KnownOrUnknown};

    use super::{Connection, ConnectionState};

    #[test]
    fn it_deserializes_a_connection() {
        let connection: Connection = serde_json::from_str(
            &json!({
              "object": "connection",
              "id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
              "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
              "connection_type": "GoogleOAuth",
              "name": "Foo Corp",
              "state": "active",
              "created_at": "2021-06-25T19:07:33.155Z",
              "updated_at": "2021-06-25T19:07:33.155Z",
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            connection,
            Connection {
                object: "connection".to_string(),
                id: "conn_01E4ZCR3C56J083X43JQXF3JK5".to_string(),
                organization_id: Some("org_01EHWNCE74X7JSDV0X3SZ3KJNY".to_string()),
                r#type: KnownOrUnknown::Known(ConnectionType::GoogleOauth),
                name: "Foo Corp".to_string(),
                state: ConnectionState::Active,
            }
        )
    }

    #[test]
    fn it_deserializes_unknown_connection_types() {
        let connection: Connection = serde_json::from_str(
            &json!({
              "object": "connection",
              "id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
              "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
              "connection_type": "UnknownType",
              "name": "Foo Corp",
              "state": "active",
              "created_at": "2021-06-25T19:07:33.155Z",
              "updated_at": "2021-06-25T19:07:33.155Z",
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            connection.r#type,
            KnownOrUnknown::Unknown("UnknownType".to_string())
        )
    }
}
