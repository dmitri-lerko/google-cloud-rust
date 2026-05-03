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

use crate::Result;
use crate::acl::{AclRule, ProjectTeam};
use crate::storage::request_options::RequestOptions;

/// The request builder for [Storage::list_bucket_acls][crate::client::Storage::list_bucket_acls] calls.
#[derive(Debug)]
pub struct ListBucketAcls<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    options: RequestOptions,
}

impl<S> Clone for ListBucketAcls<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> ListBucketAcls<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B>(stub: std::sync::Arc<S>, bucket: B, options: RequestOptions) -> Self
    where
        B: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<Vec<AclRule>> {
        self.stub.list_bucket_acls(self.bucket, self.options).await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::update_bucket_acl][crate::client::Storage::update_bucket_acl] calls.
#[derive(Debug)]
pub struct UpdateBucketAcl<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    entity: String,
    role: String,
    options: RequestOptions,
}

impl<S> Clone for UpdateBucketAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            entity: self.entity.clone(),
            role: self.role.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> UpdateBucketAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, E, R>(
        stub: std::sync::Arc<S>,
        bucket: B,
        entity: E,
        role: R,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        E: Into<String>,
        R: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            entity: entity.into(),
            role: role.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .update_bucket_acl(self.bucket, self.entity, self.role, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::delete_bucket_acl][crate::client::Storage::delete_bucket_acl] calls.
#[derive(Debug)]
pub struct DeleteBucketAcl<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    entity: String,
    options: RequestOptions,
}

impl<S> Clone for DeleteBucketAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            entity: self.entity.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> DeleteBucketAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, E>(
        stub: std::sync::Arc<S>,
        bucket: B,
        entity: E,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        E: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            entity: entity.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .delete_bucket_acl(self.bucket, self.entity, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::list_default_object_acls][crate::client::Storage::list_default_object_acls] calls.
#[derive(Debug)]
pub struct ListDefaultObjectAcls<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    options: RequestOptions,
}

impl<S> Clone for ListDefaultObjectAcls<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> ListDefaultObjectAcls<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B>(stub: std::sync::Arc<S>, bucket: B, options: RequestOptions) -> Self
    where
        B: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<Vec<AclRule>> {
        self.stub
            .list_default_object_acls(self.bucket, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::update_default_object_acl][crate::client::Storage::update_default_object_acl] calls.
#[derive(Debug)]
pub struct UpdateDefaultObjectAcl<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    entity: String,
    role: String,
    options: RequestOptions,
}

impl<S> Clone for UpdateDefaultObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            entity: self.entity.clone(),
            role: self.role.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> UpdateDefaultObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, E, R>(
        stub: std::sync::Arc<S>,
        bucket: B,
        entity: E,
        role: R,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        E: Into<String>,
        R: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            entity: entity.into(),
            role: role.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .update_default_object_acl(self.bucket, self.entity, self.role, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::delete_default_object_acl][crate::client::Storage::delete_default_object_acl] calls.
#[derive(Debug)]
pub struct DeleteDefaultObjectAcl<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    entity: String,
    options: RequestOptions,
}

impl<S> Clone for DeleteDefaultObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            entity: self.entity.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> DeleteDefaultObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, E>(
        stub: std::sync::Arc<S>,
        bucket: B,
        entity: E,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        E: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            entity: entity.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .delete_default_object_acl(self.bucket, self.entity, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::list_object_acls][crate::client::Storage::list_object_acls] calls.
#[derive(Debug)]
pub struct ListObjectAcls<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    object: String,
    options: RequestOptions,
}

impl<S> Clone for ListObjectAcls<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            object: self.object.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> ListObjectAcls<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, O>(
        stub: std::sync::Arc<S>,
        bucket: B,
        object: O,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        O: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            object: object.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<Vec<AclRule>> {
        self.stub
            .list_object_acls(self.bucket, self.object, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::update_object_acl][crate::client::Storage::update_object_acl] calls.
#[derive(Debug)]
pub struct UpdateObjectAcl<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    object: String,
    entity: String,
    role: String,
    options: RequestOptions,
}

impl<S> Clone for UpdateObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            object: self.object.clone(),
            entity: self.entity.clone(),
            role: self.role.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> UpdateObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, O, E, R>(
        stub: std::sync::Arc<S>,
        bucket: B,
        object: O,
        entity: E,
        role: R,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        O: Into<String>,
        E: Into<String>,
        R: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            object: object.into(),
            entity: entity.into(),
            role: role.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .update_object_acl(
                self.bucket,
                self.object,
                self.entity,
                self.role,
                self.options,
            )
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

/// The request builder for [Storage::delete_object_acl][crate::client::Storage::delete_object_acl] calls.
#[derive(Debug)]
pub struct DeleteObjectAcl<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    object: String,
    entity: String,
    options: RequestOptions,
}

impl<S> Clone for DeleteObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            object: self.object.clone(),
            entity: self.entity.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> DeleteObjectAcl<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, O, E>(
        stub: std::sync::Arc<S>,
        bucket: B,
        object: O,
        entity: E,
        options: RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        O: Into<String>,
        E: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            object: object.into(),
            entity: entity.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .delete_object_acl(self.bucket, self.object, self.entity, self.options)
            .await
    }

    /// Appends a user agent string to the request.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }

    /// Sets the user project for Requester Pays billing.
    pub fn with_user_project(mut self, user_project: impl Into<String>) -> Self {
        self.options.set_user_project(user_project);
        self
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawAclRule {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bucket: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub object: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub entity: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub role: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub domain: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub entity_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub etag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_team: Option<RawProjectTeam>,
}

impl RawAclRule {
    pub(crate) fn for_update(entity: String, role: String) -> Self {
        Self {
            entity,
            role,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawProjectTeam {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub project_number: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub team: String,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawAclList {
    pub items: Vec<RawAclRule>,
}

impl From<RawAclRule> for AclRule {
    fn from(value: RawAclRule) -> Self {
        Self {
            entity: value.entity,
            entity_id: value.entity_id,
            role: value.role,
            domain: value.domain,
            email: value.email,
            project_team: value.project_team.map(ProjectTeam::from),
            etag: value.etag,
        }
    }
}

impl From<RawProjectTeam> for ProjectTeam {
    fn from(value: RawProjectTeam) -> Self {
        Self {
            project_number: value.project_number,
            team: value.team,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acl::{ACL_ENTITY_ALL_USERS, ACL_ROLE_READER, ACL_ROLE_WRITER};
    use crate::client::Storage;
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use httptest::{Expectation, Server, matchers::*, responders::status_code};
    use pretty_assertions::assert_eq;

    type TestResult<T = ()> = anyhow::Result<T>;

    async fn client(server: &Server) -> TestResult<Storage> {
        Ok(Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?)
    }

    #[test]
    fn raw_acl_rule_to_public() {
        let got = AclRule::from(RawAclRule {
            entity: ACL_ENTITY_ALL_USERS.into(),
            entity_id: "entity-id".into(),
            role: ACL_ROLE_READER.into(),
            domain: "example.com".into(),
            email: "reader@example.com".into(),
            etag: "etag-1".into(),
            project_team: Some(RawProjectTeam {
                project_number: "123456".into(),
                team: "viewers".into(),
            }),
            ..Default::default()
        });
        assert_eq!(got.entity, ACL_ENTITY_ALL_USERS);
        assert_eq!(got.entity_id, "entity-id");
        assert_eq!(got.role, ACL_ROLE_READER);
        assert_eq!(got.domain, "example.com");
        assert_eq!(got.email, "reader@example.com");
        assert_eq!(
            got.project_team,
            Some(ProjectTeam {
                project_number: "123456".into(),
                team: "viewers".into()
            })
        );
        assert_eq!(got.etag, "etag-1");
    }

    #[test]
    fn raw_acl_update_body_only_contains_settable_fields() -> TestResult {
        let got = serde_json::to_value(RawAclRule::for_update(
            ACL_ENTITY_ALL_USERS.into(),
            ACL_ROLE_READER.into(),
        ))?;
        assert_eq!(
            got,
            serde_json::json!({"entity": ACL_ENTITY_ALL_USERS, "role": ACL_ROLE_READER})
        );
        Ok(())
    }

    #[tokio::test]
    async fn list_bucket_acls() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/storage/v1/b/test-bucket/acl"),
                request::query(url_decoded(contains(("userProject", "billing-project")))),
            ])
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"items":[{"entity":"allUsers","role":"READER","projectTeam":{"projectNumber":"123","team":"owners"}}]}"#,
            )),
        );

        let got = client(&server)
            .await?
            .list_bucket_acls("projects/_/buckets/test-bucket")
            .with_user_project("billing-project")
            .send()
            .await?;

        assert_eq!(got.len(), 1);
        assert_eq!(got[0].entity, ACL_ENTITY_ALL_USERS);
        assert_eq!(got[0].role, ACL_ROLE_READER);
        assert_eq!(got[0].project_team.as_ref().unwrap().project_number, "123");
        Ok(())
    }

    #[tokio::test]
    async fn update_bucket_acl() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                "/storage/v1/b/test-bucket/acl/allUsers",
            ))
            .times(1)
            .respond_with(status_code(200).body("{}")),
        );

        client(&server)
            .await?
            .update_bucket_acl(
                "projects/_/buckets/test-bucket",
                ACL_ENTITY_ALL_USERS,
                ACL_ROLE_READER,
            )
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn delete_bucket_acl() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "DELETE",
                "/storage/v1/b/test-bucket/acl/allUsers",
            ))
            .times(1)
            .respond_with(status_code(204)),
        );

        client(&server)
            .await?
            .delete_bucket_acl("projects/_/buckets/test-bucket", ACL_ENTITY_ALL_USERS)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn list_default_object_acls() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/storage/v1/b/test-bucket/defaultObjectAcl",
            ))
            .times(1)
            .respond_with(
                status_code(200).body(r#"{"items":[{"entity":"allUsers","role":"READER"}]}"#),
            ),
        );

        let got = client(&server)
            .await?
            .list_default_object_acls("projects/_/buckets/test-bucket")
            .send()
            .await?;

        assert_eq!(got[0].role, ACL_ROLE_READER);
        Ok(())
    }

    #[tokio::test]
    async fn update_default_object_acl() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                "/storage/v1/b/test-bucket/defaultObjectAcl/allUsers",
            ))
            .times(1)
            .respond_with(status_code(200).body("{}")),
        );

        client(&server)
            .await?
            .update_default_object_acl(
                "projects/_/buckets/test-bucket",
                ACL_ENTITY_ALL_USERS,
                ACL_ROLE_READER,
            )
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn delete_default_object_acl() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "DELETE",
                "/storage/v1/b/test-bucket/defaultObjectAcl/allUsers",
            ))
            .times(1)
            .respond_with(status_code(204)),
        );

        client(&server)
            .await?
            .delete_default_object_acl("projects/_/buckets/test-bucket", ACL_ENTITY_ALL_USERS)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn list_object_acls() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/storage/v1/b/test-bucket/o/folder%2Fobject%20name/acl",
            ))
            .times(1)
            .respond_with(
                status_code(200).body(r#"{"items":[{"entity":"allUsers","role":"READER"}]}"#),
            ),
        );

        let got = client(&server)
            .await?
            .list_object_acls("projects/_/buckets/test-bucket", "folder/object name")
            .send()
            .await?;

        assert_eq!(got[0].entity, ACL_ENTITY_ALL_USERS);
        Ok(())
    }

    #[tokio::test]
    async fn update_object_acl() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "PUT",
                "/storage/v1/b/test-bucket/o/object/acl/allUsers",
            ))
            .times(1)
            .respond_with(status_code(200).body("{}")),
        );

        client(&server)
            .await?
            .update_object_acl(
                "projects/_/buckets/test-bucket",
                "object",
                ACL_ENTITY_ALL_USERS,
                ACL_ROLE_WRITER,
            )
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn delete_object_acl() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "DELETE",
                "/storage/v1/b/test-bucket/o/object/acl/allUsers",
            ))
            .times(1)
            .respond_with(status_code(204)),
        );

        client(&server)
            .await?
            .delete_object_acl(
                "projects/_/buckets/test-bucket",
                "object",
                ACL_ENTITY_ALL_USERS,
            )
            .send()
            .await?;

        Ok(())
    }
}
