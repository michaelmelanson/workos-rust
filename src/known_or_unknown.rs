use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KnownOrUnknown<K, U> {
    Known(K),
    Unknown(U),
}
