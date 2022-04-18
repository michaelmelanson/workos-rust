use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    #[serde(rename = "ADFSSAML")]
    AdfsSaml,

    #[serde(rename = "ADPOIDC")]
    AdpOidc,

    #[serde(rename = "Auth0SAML")]
    Auth0Saml,

    #[serde(rename = "AzureSAML")]
    AzureSaml,

    #[serde(rename = "CASSAML")]
    CasSaml,

    #[serde(rename = "ClassLinkSAML")]
    ClassLinkSaml,

    #[serde(rename = "CloudflareSAML")]
    CloudflareSaml,

    #[serde(rename = "CyberArkSAML")]
    CyberArkSaml,

    #[serde(rename = "DuoSAML")]
    DuoSaml,

    #[serde(rename = "GenericOIDC")]
    GenericOidc,

    #[serde(rename = "GenericSAML")]
    GenericSaml,

    #[serde(rename = "GoogleOAuth")]
    GoogleOauth,

    #[serde(rename = "GoogleSAML")]
    GoogleSaml,

    #[serde(rename = "JumpCloudSAML")]
    JumpCloudSaml,

    #[serde(rename = "KeycloakSAML")]
    KeycloakSaml,

    #[serde(rename = "MicrosoftOAuth")]
    MicrosoftOauth,

    #[serde(rename = "MiniOrangeSAML")]
    MiniOrangeSaml,

    #[serde(rename = "NetIqSAML")]
    NetIqSaml,

    #[serde(rename = "OktaSAML")]
    OktaSaml,

    #[serde(rename = "OneLoginSAML")]
    OneLoginSaml,

    #[serde(rename = "OracleSAML")]
    OracleSaml,

    #[serde(rename = "PingFederateSAML")]
    PingFederateSaml,

    #[serde(rename = "PingOneSAML")]
    PingOneSaml,

    #[serde(rename = "SalesforceSAML")]
    SalesforceSaml,

    #[serde(rename = "ShibbolethSAML")]
    ShibbolethSaml,

    #[serde(rename = "SimpleSamlPhpSAML")]
    SimpleSamlPhpSaml,

    #[serde(rename = "VMwareSAML")]
    VMwareSaml,
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use serde_json::json;

    use super::ConnectionType;

    #[derive(Serialize)]
    struct Wrapper {
        r#type: ConnectionType,
    }

    #[test]
    fn it_properly_serializes_adp_oidc() {
        assert_eq!(
            serde_json::to_string(&ConnectionType::AdpOidc).unwrap(),
            json!("ADPOIDC").to_string()
        )
    }

    #[test]
    fn it_properly_deserializes_adp_oidc() {
        assert_eq!(
            serde_json::from_str::<ConnectionType>(&json!("ADPOIDC").to_string()).unwrap(),
            ConnectionType::AdpOidc
        )
    }
}
