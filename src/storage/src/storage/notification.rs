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

use crate::notification::Notification;
use crate::{Error, Result};
use std::collections::BTreeMap;

/// The request builder for [Storage::list_notifications][crate::client::Storage::list_notifications] calls.
///
/// # Example
/// ```
/// # use google_cloud_storage::client::Storage;
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// let notifications = client
///     .list_notifications("projects/_/buckets/my-bucket")
///     .send()
///     .await?;
/// println!("notifications={notifications:?}");
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ListNotifications<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for ListNotifications<S>
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

impl<S> ListNotifications<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B>(
        stub: std::sync::Arc<S>,
        bucket: B,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
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
    pub async fn send(self) -> Result<BTreeMap<String, Notification>> {
        self.stub
            .list_notifications(self.bucket, self.options)
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

/// The request builder for [Storage::create_notification][crate::client::Storage::create_notification] calls.
///
/// # Example
/// ```
/// # use google_cloud_storage::client::Storage;
/// # use google_cloud_storage::notification::{JSON_PAYLOAD, Notification};
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// let notification = Notification {
///     topic_project_id: "my-project".into(),
///     topic_id: "my-topic".into(),
///     payload_format: JSON_PAYLOAD.into(),
///     ..Default::default()
/// };
/// let created = client
///     .create_notification("projects/_/buckets/my-bucket", notification)
///     .send()
///     .await?;
/// println!("notification={created:?}");
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct CreateNotification<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    notification: Notification,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for CreateNotification<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            notification: self.notification.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> CreateNotification<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B>(
        stub: std::sync::Arc<S>,
        bucket: B,
        notification: Notification,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        B: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            notification,
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<Notification> {
        validate_notification_for_create(&self.notification)?;
        self.stub
            .create_notification(self.bucket, self.notification, self.options)
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

/// The request builder for [Storage::delete_notification][crate::client::Storage::delete_notification] calls.
///
/// # Example
/// ```
/// # use google_cloud_storage::client::Storage;
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// client
///     .delete_notification("projects/_/buckets/my-bucket", "notification-id")
///     .send()
///     .await?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct DeleteNotification<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    bucket: String,
    notification: String,
    options: crate::storage::request_options::RequestOptions,
}

impl<S> Clone for DeleteNotification<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            bucket: self.bucket.clone(),
            notification: self.notification.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> DeleteNotification<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<B, N>(
        stub: std::sync::Arc<S>,
        bucket: B,
        notification: N,
        options: crate::storage::request_options::RequestOptions,
    ) -> Self
    where
        B: Into<String>,
        N: Into<String>,
    {
        Self {
            stub,
            bucket: bucket.into(),
            notification: notification.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<()> {
        self.stub
            .delete_notification(self.bucket, self.notification, self.options)
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

fn validate_notification_for_create(notification: &Notification) -> Result<()> {
    if !notification.id.is_empty() {
        return Err(Error::binding("notification id must not be set"));
    }
    if notification.topic_project_id.is_empty() {
        return Err(Error::binding("missing topic project id"));
    }
    if notification.topic_id.is_empty() {
        return Err(Error::binding("missing topic id"));
    }
    Ok(())
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RawNotification {
    pub id: String,
    pub topic: String,
    pub event_types: Vec<String>,
    pub object_name_prefix: String,
    pub custom_attributes: BTreeMap<String, String>,
    pub payload_format: String,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct ListNotificationsResponse {
    pub items: Vec<RawNotification>,
}

pub(crate) fn notifications_to_map(
    notifications: impl IntoIterator<Item = RawNotification>,
) -> BTreeMap<String, Notification> {
    notifications
        .into_iter()
        .map(|notification| {
            let notification = Notification::from(notification);
            (notification.id.clone(), notification)
        })
        .collect()
}

impl From<RawNotification> for Notification {
    fn from(value: RawNotification) -> Self {
        let (topic_project_id, topic_id) = parse_notification_topic(&value.topic);
        Self {
            id: value.id,
            topic_id,
            topic_project_id,
            event_types: value.event_types,
            object_name_prefix: value.object_name_prefix,
            custom_attributes: value.custom_attributes,
            payload_format: value.payload_format,
        }
    }
}

impl From<Notification> for RawNotification {
    fn from(value: Notification) -> Self {
        Self {
            id: value.id,
            topic: format!(
                "//pubsub.googleapis.com/projects/{}/topics/{}",
                value.topic_project_id, value.topic_id
            ),
            event_types: value.event_types,
            object_name_prefix: value.object_name_prefix,
            custom_attributes: value.custom_attributes,
            payload_format: value.payload_format,
        }
    }
}

fn parse_notification_topic(topic: &str) -> (String, String) {
    let Some(rest) = topic.strip_prefix("//pubsub.googleapis.com/projects/") else {
        return ("?".into(), "?".into());
    };
    let Some((project, topic)) = rest.split_once("/topics/") else {
        return ("?".into(), "?".into());
    };
    if project.is_empty() || topic.is_empty() || topic.contains('/') {
        return ("?".into(), "?".into());
    }
    (project.into(), topic.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Storage;
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use httptest::{Expectation, Server, matchers::*, responders::status_code};

    type TestResult = anyhow::Result<()>;

    #[test]
    fn parse_topic() {
        assert_eq!(
            parse_notification_topic("//pubsub.googleapis.com/projects/p/topics/t"),
            ("p".into(), "t".into())
        );
        assert_eq!(parse_notification_topic("bad"), ("?".into(), "?".into()));
    }

    #[test]
    fn convert_notification() -> TestResult {
        let notification = Notification {
            id: "id-1".into(),
            topic_project_id: "project-1".into(),
            topic_id: "topic-1".into(),
            event_types: vec![crate::notification::OBJECT_FINALIZE_EVENT.into()],
            object_name_prefix: "prefix/".into(),
            custom_attributes: BTreeMap::from([("k".into(), "v".into())]),
            payload_format: crate::notification::JSON_PAYLOAD.into(),
        };
        let raw = RawNotification::from(notification.clone());
        assert_eq!(
            raw.topic,
            "//pubsub.googleapis.com/projects/project-1/topics/topic-1"
        );
        assert_eq!(Notification::from(raw), notification);
        Ok(())
    }

    #[test]
    fn notifications_map() {
        let got = notifications_to_map([RawNotification {
            id: "id-1".into(),
            topic: "//pubsub.googleapis.com/projects/project-1/topics/topic-1".into(),
            ..Default::default()
        }]);
        assert_eq!(got.keys().collect::<Vec<_>>(), vec!["id-1"]);
        assert_eq!(got["id-1"].topic_project_id, "project-1");
        assert_eq!(got["id-1"].topic_id, "topic-1");
    }

    #[tokio::test]
    async fn create_validation() {
        let notification = Notification {
            id: "id-1".into(),
            topic_project_id: "project-1".into(),
            topic_id: "topic-1".into(),
            ..Default::default()
        };
        let err = validate_notification_for_create(&notification)
            .expect_err("id must not be accepted on create");
        assert!(err.is_binding(), "{err:?}");

        let notification = Notification {
            topic_id: "topic-1".into(),
            ..Default::default()
        };
        let err = validate_notification_for_create(&notification)
            .expect_err("topic project id is required");
        assert!(err.is_binding(), "{err:?}");

        let notification = Notification {
            topic_project_id: "project-1".into(),
            ..Default::default()
        };
        let err =
            validate_notification_for_create(&notification).expect_err("topic id is required");
        assert!(err.is_binding(), "{err:?}");
    }

    #[tokio::test]
    async fn list_notifications() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/storage/v1/b/test-bucket/notificationConfigs"),
                request::query(url_decoded(contains(("userProject", "billing-project")))),
            ])
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"items":[{"id":"n1","topic":"//pubsub.googleapis.com/projects/p/topics/t","payloadFormat":"JSON_API_V1"}]}"#,
            )),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .list_notifications("projects/_/buckets/test-bucket")
            .with_user_project("billing-project")
            .send()
            .await?;

        assert_eq!(got.len(), 1);
        assert_eq!(got["n1"].topic_project_id, "p");
        assert_eq!(got["n1"].topic_id, "t");
        assert_eq!(got["n1"].payload_format, crate::notification::JSON_PAYLOAD);
        Ok(())
    }

    #[tokio::test]
    async fn create_notification() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("POST", "/storage/v1/b/test-bucket/notificationConfigs"),
                request::query(url_decoded(contains(("userProject", "billing-project")))),
            ])
            .times(1)
            .respond_with(status_code(200).body(
                r#"{"id":"n1","topic":"//pubsub.googleapis.com/projects/p/topics/t","payloadFormat":"JSON_API_V1"}"#,
            )),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .create_notification(
                "projects/_/buckets/test-bucket",
                Notification {
                    topic_project_id: "p".into(),
                    topic_id: "t".into(),
                    payload_format: crate::notification::JSON_PAYLOAD.into(),
                    ..Default::default()
                },
            )
            .with_user_project("billing-project")
            .send()
            .await?;

        assert_eq!(got.id, "n1");
        assert_eq!(got.topic_project_id, "p");
        assert_eq!(got.topic_id, "t");
        Ok(())
    }

    #[tokio::test]
    async fn delete_notification() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path(
                    "DELETE",
                    "/storage/v1/b/test-bucket/notificationConfigs/notification%2Fid",
                ),
                request::query(url_decoded(contains(("userProject", "billing-project")))),
            ])
            .times(1)
            .respond_with(status_code(204)),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        client
            .delete_notification("projects/_/buckets/test-bucket", "notification/id")
            .with_user_project("billing-project")
            .send()
            .await?;

        Ok(())
    }
}
