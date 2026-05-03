// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::hmac_key::{
    HMAC_KEY_ACTIVE, HMAC_KEY_INACTIVE, HmacKey, HmacKeyUpdate, ListHmacKeysResponse,
};
use crate::{Error, Result};

/// The request builder for [Storage::create_hmac_key][crate::client::Storage::create_hmac_key] calls.
///
/// # Example
/// ```
/// # use google_cloud_storage::client::Storage;
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// let key = client
///     .create_hmac_key("my-project", "service-account@example.com")
///     .send()
///     .await?;
/// println!("access id={}", key.access_id);
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct CreateHmacKey<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    project: String,
    service_account_email: String,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for CreateHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            project: self.project.clone(),
            service_account_email: self.service_account_email.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> CreateHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<P, E>(
        stub: std::sync::Arc<S>,
        project: P,
        service_account_email: E,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        P: Into<String>,
        E: Into<String>,
    {
        Self {
            stub,
            project: project.into(),
            service_account_email: service_account_email.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<HmacKey> {
        if self.project.is_empty() {
            return Err(Error::binding("missing project id"));
        }
        if self.service_account_email.is_empty() {
            return Err(Error::binding("missing service account email"));
        }
        self.stub
            .create_hmac_key(self.project, self.service_account_email, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::get_hmac_key][crate::client::Storage::get_hmac_key] calls.
#[derive(Debug)]
pub struct GetHmacKey<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    project: String,
    access_id: String,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for GetHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            project: self.project.clone(),
            access_id: self.access_id.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> GetHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<P, A>(
        stub: std::sync::Arc<S>,
        project: P,
        access_id: A,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        P: Into<String>,
        A: Into<String>,
    {
        Self {
            stub,
            project: project.into(),
            access_id: access_id.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<HmacKey> {
        self.stub
            .get_hmac_key(self.project, self.access_id, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::update_hmac_key][crate::client::Storage::update_hmac_key] calls.
#[derive(Debug)]
pub struct UpdateHmacKey<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    project: String,
    access_id: String,
    update: HmacKeyUpdate,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for UpdateHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            project: self.project.clone(),
            access_id: self.access_id.clone(),
            update: self.update.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> UpdateHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<P, A>(
        stub: std::sync::Arc<S>,
        project: P,
        access_id: A,
        update: HmacKeyUpdate,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        P: Into<String>,
        A: Into<String>,
    {
        Self {
            stub,
            project: project.into(),
            access_id: access_id.into(),
            update,
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<HmacKey> {
        if self.update.state != HMAC_KEY_ACTIVE && self.update.state != HMAC_KEY_INACTIVE {
            return Err(Error::binding(format!(
                "invalid HMAC key state {}, must be {HMAC_KEY_ACTIVE} or {HMAC_KEY_INACTIVE}",
                self.update.state
            )));
        }
        self.stub
            .update_hmac_key(self.project, self.access_id, self.update, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::delete_hmac_key][crate::client::Storage::delete_hmac_key] calls.
#[derive(Debug)]
pub struct DeleteHmacKey<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    project: String,
    access_id: String,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for DeleteHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            project: self.project.clone(),
            access_id: self.access_id.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> DeleteHmacKey<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<P, A>(
        stub: std::sync::Arc<S>,
        project: P,
        access_id: A,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        P: Into<String>,
        A: Into<String>,
    {
        Self {
            stub,
            project: project.into(),
            access_id: access_id.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .delete_hmac_key(self.project, self.access_id, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::list_hmac_keys][crate::client::Storage::list_hmac_keys] calls.
#[derive(Debug)]
pub struct ListHmacKeys<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    project: String,
    service_account_email: Option<String>,
    show_deleted_keys: bool,
    page_size: Option<i64>,
    page_token: Option<String>,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for ListHmacKeys<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            project: self.project.clone(),
            service_account_email: self.service_account_email.clone(),
            show_deleted_keys: self.show_deleted_keys,
            page_size: self.page_size,
            page_token: self.page_token.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> ListHmacKeys<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<P>(
        stub: std::sync::Arc<S>,
        project: P,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        P: Into<String>,
    {
        Self {
            stub,
            project: project.into(),
            service_account_email: None,
            show_deleted_keys: false,
            page_size: None,
            page_token: None,
            options,
        }
    }

    /// Filters keys to those associated with this service account email.
    pub fn set_service_account_email(mut self, service_account_email: impl Into<String>) -> Self {
        self.service_account_email = Some(service_account_email.into());
        self
    }

    /// Includes deleted HMAC keys in the response.
    pub fn set_show_deleted_keys(mut self, show_deleted_keys: bool) -> Self {
        self.show_deleted_keys = show_deleted_keys;
        self
    }

    /// Sets the maximum number of results to return.
    pub fn set_page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets the page token for the request.
    pub fn set_page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<ListHmacKeysResponse> {
        self.stub
            .list_hmac_keys(
                self.project,
                self.service_account_email,
                self.show_deleted_keys,
                self.page_size,
                self.page_token,
                self.options,
            )
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawHmacKey {
    pub metadata: RawHmacKeyMetadata,
    pub secret: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawHmacKeyMetadata {
    pub access_id: String,
    pub etag: String,
    pub id: String,
    pub project_id: String,
    pub service_account_email: String,
    pub time_created: Option<wkt::Timestamp>,
    pub updated: Option<wkt::Timestamp>,
    pub state: String,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawListHmacKeysResponse {
    pub items: Vec<RawHmacKeyMetadata>,
    pub next_page_token: String,
}

impl From<RawHmacKey> for HmacKey {
    fn from(value: RawHmacKey) -> Self {
        let mut key = HmacKey::from(value.metadata);
        key.secret = value.secret;
        key
    }
}

impl From<RawHmacKeyMetadata> for HmacKey {
    fn from(value: RawHmacKeyMetadata) -> Self {
        Self {
            secret: String::new(),
            access_id: value.access_id,
            etag: value.etag,
            id: value.id,
            project_id: value.project_id,
            service_account_email: value.service_account_email,
            create_time: value.time_created,
            update_time: value.updated,
            state: value.state,
        }
    }
}

impl From<HmacKeyUpdate> for RawHmacKeyMetadata {
    fn from(value: HmacKeyUpdate) -> Self {
        Self {
            state: value.state,
            etag: value.etag,
            ..Default::default()
        }
    }
}

impl From<RawListHmacKeysResponse> for ListHmacKeysResponse {
    fn from(value: RawListHmacKeysResponse) -> Self {
        Self {
            hmac_keys: value.items.into_iter().map(HmacKey::from).collect(),
            next_page_token: value.next_page_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Storage;
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use httptest::{Expectation, Server, matchers::*, responders::status_code};

    #[test]
    fn raw_hmac_key_to_public() {
        let key = HmacKey::from(RawHmacKey {
            secret: "secret".into(),
            metadata: RawHmacKeyMetadata {
                access_id: "access-id".into(),
                project_id: "project-id".into(),
                service_account_email: "service@example.com".into(),
                state: HMAC_KEY_ACTIVE.into(),
                ..Default::default()
            },
        });
        assert_eq!(key.secret, "secret");
        assert_eq!(key.access_id, "access-id");
        assert_eq!(key.project_id, "project-id");
        assert_eq!(key.service_account_email, "service@example.com");
        assert_eq!(key.state, HMAC_KEY_ACTIVE);
    }

    #[tokio::test]
    async fn create_hmac_key() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("POST", "/storage/v1/projects/test-project/hmacKeys"),
                request::query(url_decoded(contains((
                    "serviceAccountEmail",
                    "service@example.com"
                )))),
                request::query(url_decoded(contains((
                    "userProject",
                    "billing-project"
                )))),
            ])
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"secret":"secret","metadata":{"accessId":"access-id","projectId":"test-project","serviceAccountEmail":"service@example.com","state":"ACTIVE"}}"#,
            )),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .create_hmac_key("test-project", "service@example.com")
            .with_user_project("billing-project")
            .send()
            .await?;

        assert_eq!(got.secret, "secret");
        assert_eq!(got.access_id, "access-id");
        assert_eq!(got.project_id, "test-project");
        assert_eq!(got.service_account_email, "service@example.com");
        assert_eq!(got.state, HMAC_KEY_ACTIVE);
        Ok(())
    }

    #[tokio::test]
    async fn get_hmac_key() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/storage/v1/projects/test-project/hmacKeys/access%2Fid",
            ))
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"accessId":"access/id","projectId":"test-project","serviceAccountEmail":"service@example.com","state":"ACTIVE"}"#,
            )),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .get_hmac_key("test-project", "access/id")
            .send()
            .await?;

        assert_eq!(got.access_id, "access/id");
        assert_eq!(got.state, HMAC_KEY_ACTIVE);
        Ok(())
    }

    #[tokio::test]
    async fn update_hmac_key() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                "/storage/v1/projects/test-project/hmacKeys/access-id",
            ))
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"accessId":"access-id","projectId":"test-project","serviceAccountEmail":"service@example.com","state":"INACTIVE","etag":"etag-1"}"#,
            )),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .update_hmac_key(
                "test-project",
                "access-id",
                HmacKeyUpdate {
                    state: HMAC_KEY_INACTIVE.into(),
                    etag: "etag-1".into(),
                },
            )
            .send()
            .await?;

        assert_eq!(got.state, HMAC_KEY_INACTIVE);
        assert_eq!(got.etag, "etag-1");
        Ok(())
    }

    #[tokio::test]
    async fn delete_hmac_key() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "DELETE",
                "/storage/v1/projects/test-project/hmacKeys/access-id",
            ))
            .times(1)
            .respond_with(status_code(204)),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        client
            .delete_hmac_key("test-project", "access-id")
            .send()
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn list_hmac_keys() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/storage/v1/projects/test-project/hmacKeys"),
                request::query(url_decoded(contains((
                    "serviceAccountEmail",
                    "service@example.com"
                )))),
                request::query(url_decoded(contains(("showDeletedKeys", "true")))),
                request::query(url_decoded(contains(("maxResults", "10")))),
                request::query(url_decoded(contains(("pageToken", "token-1")))),
            ])
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"items":[{"accessId":"access-id","projectId":"test-project","serviceAccountEmail":"service@example.com","state":"ACTIVE"}],"nextPageToken":"token-2"}"#,
            )),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .list_hmac_keys("test-project")
            .set_service_account_email("service@example.com")
            .set_show_deleted_keys(true)
            .set_page_size(10)
            .set_page_token("token-1")
            .send()
            .await?;

        assert_eq!(got.hmac_keys.len(), 1);
        assert_eq!(got.hmac_keys[0].access_id, "access-id");
        assert_eq!(got.next_page_token, "token-2");
        Ok(())
    }
}
