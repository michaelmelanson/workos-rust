use async_trait::async_trait;
use serde::Serialize;

use crate::directory_sync::{DirectoryGroup, DirectoryId, DirectorySync, DirectoryUserId};
use crate::{PaginatedList, PaginationParams, ResponseExt, WorkOsResult};

/// A filter for [`ListDirectoryGroups`].
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DirectoryGroupsFilter<'a> {
    /// Retrieve directory groups within the specified directory.
    Directory {
        /// The ID of the directory to retrieve directory groups in.
        directory: &'a DirectoryId,
    },

    /// Retrieve directory groups a specified directory user is a member of.
    User {
        /// The ID of the directory user to retrieve directory groups for.
        user: &'a DirectoryUserId,
    },
}

/// The parameters for [`ListDirectoryGroups`].
#[derive(Debug, Serialize)]
pub struct ListDirectoryGroupsParams<'a> {
    /// The pagination parameters to use when listing directory groups.
    #[serde(flatten)]
    pub pagination: PaginationParams<'a>,

    /// The filter to use when listing directory groupss.
    #[serde(flatten)]
    pub filter: DirectoryGroupsFilter<'a>,
}

/// [WorkOS Docs: List Directory Groups](https://workos.com/docs/reference/directory-sync/group/list)
#[async_trait]
pub trait ListDirectoryGroups {
    /// Retrieves a list of [`DirectoryGroup`]s.
    ///
    /// [WorkOS Docs: List Directory Groups](https://workos.com/docs/reference/directory-sync/group/list)
    ///
    /// ```
    /// # use workos::WorkOsResult;
    /// # use workos::directory_sync::*;
    /// use workos::{ApiKey, WorkOs};
    ///
    /// # async fn run() -> WorkOsResult<(), ()> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let paginated_groups = workos
    ///     .directory_sync()
    ///     .list_directory_groups(&ListDirectoryGroupsParams {
    ///         filter: DirectoryGroupsFilter::Directory {
    ///             directory: &DirectoryId::from("directory_01ECAZ4NV9QMV47GW873HDCX74"),
    ///         },
    ///         pagination: Default::default(),
    ///     })
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn list_directory_groups(
        &self,
        params: &ListDirectoryGroupsParams<'_>,
    ) -> WorkOsResult<PaginatedList<DirectoryGroup>, ()>;
}

#[async_trait]
impl<'a> ListDirectoryGroups for DirectorySync<'a> {
    async fn list_directory_groups(
        &self,
        params: &ListDirectoryGroupsParams<'_>,
    ) -> WorkOsResult<PaginatedList<DirectoryGroup>, ()> {
        let url = self.workos.base_url().join("/directory_groups")?;
        let directory_groups = self
            .workos
            .client()
            .get(url)
            .query(&params)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<DirectoryGroup>>()
            .await?;

        Ok(directory_groups)
    }
}

#[cfg(test)]
mod test {
    use mockito::{self, Matcher};
    use serde_json::json;
    use tokio;

    use crate::directory_sync::DirectoryGroupId;
    use crate::{ApiKey, WorkOs};

    use super::*;

    #[tokio::test]
    async fn it_calls_the_list_directory_groups_endpoint_with_a_directory_id() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/directory_groups")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("order".to_string(), "desc".to_string()),
                Matcher::UrlEncoded(
                    "directory".to_string(),
                    "directory_01ECAZ4NV9QMV47GW873HDCX74".to_string(),
                ),
            ]))
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "data" : [{
                        "id" : "directory_group_01E1JJS84MFPPQ3G655FHTKX6Z",
                        "idp_id": "02grqrue4294w24",
                        "directory_id": "directory_01ECAZ4NV9QMV47GW873HDCX74",
                        "name" : "Developers",
                        "created_at": "2021-06-25T19:07:33.155Z",
                        "updated_at": "2021-06-25T19:07:33.155Z",
                        "raw_attributes": {"id":"02grqrue4294w24"}
                      }],
                      "list_metadata" : {
                        "after" : "directory_group_01E1JJS84MFPPQ3G655FHTKX6Z",
                        "before" : "directory_group_01E1JJS84MFPPQ3G655FHTKX6Z"
                      }
                    }
                )
                .to_string(),
            )
            .create();

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let paginated_list = workos
            .directory_sync()
            .list_directory_groups(&ListDirectoryGroupsParams {
                pagination: Default::default(),
                filter: DirectoryGroupsFilter::Directory {
                    directory: &DirectoryId::from("directory_01ECAZ4NV9QMV47GW873HDCX74"),
                },
            })
            .await
            .unwrap();

        assert_eq!(
            paginated_list
                .data
                .into_iter()
                .next()
                .map(|directory_group| directory_group.id),
            Some(DirectoryGroupId::from(
                "directory_group_01E1JJS84MFPPQ3G655FHTKX6Z"
            ))
        )
    }

    #[tokio::test]
    async fn it_calls_the_list_directory_groups_endpoint_with_a_directory_user_id() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/directory_groups")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("order".to_string(), "desc".to_string()),
                Matcher::UrlEncoded(
                    "user".to_string(),
                    "directory_user_01FYVX377G1S69ASY580WK6WVN".to_string(),
                ),
            ]))
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                "data": [
                    {
                        "object": "directory_group",
                        "id": "directory_group_01FYVX39X7A7YS95CEAJ9AJT18",
                        "idp_id": "Developers",
                        "directory_id": "directory_01FYVWZ2KGW7KPKGR58VHW1KT2",
                        "name": "Developers",
                        "created_at": "2022-03-23T17:27:24.838Z",
                        "updated_at": "2022-03-23T17:27:24.838Z",
                        "raw_attributes": {
                            "meta": {
                                "resourceType": "Group"
                            },
                            "members": [],
                            "schemas": [
                                "urn:ietf:params:scim:schemas:core:2.0:Group"
                            ],
                            "externalId": "0b797e61-352a-4e94-b21b-2be370ec5541",
                            "displayName": "Developers"
                        }
                    }
                ],
                "list_metadata": {
                    "before": "directory_group_01FYVX39X7A7YS95CEAJ9AJT18",
                    "after": null
                }})
                .to_string(),
            )
            .create();

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let paginated_list = workos
            .directory_sync()
            .list_directory_groups(&ListDirectoryGroupsParams {
                pagination: Default::default(),
                filter: DirectoryGroupsFilter::User {
                    user: &DirectoryUserId::from("directory_user_01FYVX377G1S69ASY580WK6WVN"),
                },
            })
            .await
            .unwrap();

        assert_eq!(
            paginated_list
                .data
                .into_iter()
                .next()
                .map(|directory_group| directory_group.id),
            Some(DirectoryGroupId::from(
                "directory_group_01FYVX39X7A7YS95CEAJ9AJT18"
            ))
        )
    }
}
