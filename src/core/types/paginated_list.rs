use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedList<T> {
    pub data: Vec<T>,

    #[serde(rename = "list_metadata")]
    pub metadata: ListMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMetadata {
    pub before: Option<String>,
    pub after: Option<String>,
}
