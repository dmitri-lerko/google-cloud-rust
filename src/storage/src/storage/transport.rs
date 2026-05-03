// Copyright 2025 Google LLC
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

use super::tracing::{TracingObjectDescriptor, TracingResponse};
use crate::acl::AclRule;
use crate::hmac_key::{HmacKey, HmacKeyUpdate, ListHmacKeysResponse};
use crate::model::{Object, ReadObjectRequest};
use crate::model_ext::WriteObjectRequest;
use crate::notification::Notification;
use crate::read_object::ReadObjectResponse;
use crate::storage::acl::{RawAclList, RawAclRule};
use crate::storage::client::{StorageInner, enc};
use crate::storage::hmac_key::{RawHmacKey, RawHmacKeyMetadata, RawListHmacKeysResponse};
use crate::storage::info::{INSTRUMENTATION, X_GOOG_API_CLIENT_HEADER};
use crate::storage::notification::{
    ListNotificationsResponse, RawNotification, notifications_to_map,
};
use crate::storage::perform_upload::PerformUpload;
use crate::storage::read_object::Reader;
use crate::storage::request_options::RequestOptions;
use crate::storage::service_account::ServiceAccountResponse;
use crate::storage::streaming_source::{Seek, StreamingSource};
use crate::{Error, Result};
use crate::{
    model_ext::OpenObjectRequest, object_descriptor::ObjectDescriptor,
    storage::bidi::connector::Connector, storage::bidi::transport::ObjectDescriptorTransport,
};
use gaxi::attempt_info::AttemptInfo;
use gaxi::http::reqwest::{HeaderValue, Method};
use gaxi::observability::{ClientRequestAttributes, DurationMetric, RequestRecorder};
use google_cloud_gax::options::internal::{PathTemplate, RequestOptionsExt, ResourceName};
use std::collections::BTreeMap;
use std::sync::Arc;

/// An implementation of [`stub::Storage`][crate::storage::stub::Storage] that
/// interacts with the Cloud Storage service.
///
/// This is the default implementation of a
/// [`client::Storage<T>`][crate::storage::client::Storage].
///
/// ## Example
///
/// ```
/// # async fn sample() -> anyhow::Result<()> {
/// use google_cloud_storage::client::Storage;
/// use google_cloud_storage::stub::DefaultStorage;
/// let client: Storage<DefaultStorage> = Storage::builder().build().await?;
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct Storage {
    inner: Arc<StorageInner>,
    tracing: bool,
    metric: DurationMetric,
}

impl Storage {
    pub(crate) fn new(inner: Arc<StorageInner>, tracing: bool) -> Arc<Self> {
        let metric = DurationMetric::new(&INSTRUMENTATION);
        Arc::new(Self {
            inner,
            tracing,
            metric,
        })
    }

    async fn read_object_plain(
        &self,
        request: ReadObjectRequest,
        options: RequestOptions,
    ) -> Result<ReadObjectResponse> {
        let reader = Reader {
            inner: self.inner.clone(),
            request,
            options,
        };
        reader.response().await
    }

    #[tracing::instrument(name = "read_object", level = tracing::Level::DEBUG, ret, err(Debug))]
    async fn read_object_tracing(
        &self,
        request: ReadObjectRequest,
        options: RequestOptions,
    ) -> Result<ReadObjectResponse> {
        let resource_name = format!("//storage.googleapis.com/{}", request.bucket);
        let (span, pending) = gaxi::client_request_signals!(
        metric: self.metric.clone(),
        info: *INSTRUMENTATION,
        method: "client::Storage::read_object",
        async {
            if let Some(recorder) = RequestRecorder::current() {
                recorder.on_client_request(
                    ClientRequestAttributes::default()
                        .set_url_template("/storage/v1/b/{bucket}/o/{object}")
                        .set_resource_name(resource_name),
                );
            }
            self.read_object_plain(request, options).await
        });

        let response = pending.await?;
        let inner = TracingResponse::new(response.into_parts(), span);
        Ok(ReadObjectResponse::new(Box::new(inner)))
    }

    async fn write_object_buffered_plain<P>(
        &self,
        payload: P,
        request: WriteObjectRequest,
        options: RequestOptions,
    ) -> Result<Object>
    where
        P: StreamingSource + Send + Sync + 'static,
    {
        PerformUpload::new(
            payload,
            self.inner.clone(),
            request.spec,
            request.params,
            options,
        )
        .send()
        .await
    }

    #[tracing::instrument(name = "write_object_buffered", level = tracing::Level::DEBUG, ret, err(Debug), skip(payload))]
    async fn write_object_buffered_tracing<P>(
        &self,
        payload: P,
        request: WriteObjectRequest,
        options: RequestOptions,
    ) -> Result<Object>
    where
        P: StreamingSource + Send + Sync + 'static,
    {
        let resource_name = format!(
            "//storage.googleapis.com/{}",
            request
                .spec
                .resource
                .as_ref()
                .map(|r| r.bucket.as_str())
                .unwrap_or_default()
        );
        let (_span, pending) = gaxi::client_request_signals!(
            metric: self.metric.clone(),
            info: *INSTRUMENTATION,
            method: "client::Storage::write_object",
            async {
                if let Some(recorder) = RequestRecorder::current() {
                    recorder.on_client_request(
                        ClientRequestAttributes::default()
                            .set_url_template("/upload/storage/v1/b/{bucket}/o")
                            .set_resource_name(resource_name),
                    );
                }
                self.write_object_buffered_plain(payload, request, options).await
            }
        );
        pending.await
    }

    async fn write_object_unbuffered_plain<P>(
        &self,
        payload: P,
        request: WriteObjectRequest,
        options: RequestOptions,
    ) -> Result<Object>
    where
        P: StreamingSource + Seek + Send + Sync + 'static,
    {
        PerformUpload::new(
            payload,
            self.inner.clone(),
            request.spec,
            request.params,
            options,
        )
        .send_unbuffered()
        .await
    }

    #[tracing::instrument(name = "write_object_unbuffered", level = tracing::Level::DEBUG, ret, err(Debug), skip(payload))]
    async fn write_object_unbuffered_tracing<P>(
        &self,
        payload: P,
        request: WriteObjectRequest,
        options: RequestOptions,
    ) -> Result<Object>
    where
        P: StreamingSource + Seek + Send + Sync + 'static,
    {
        let resource_name = format!(
            "//storage.googleapis.com/{}",
            request
                .spec
                .resource
                .as_ref()
                .map(|r| r.bucket.as_str())
                .unwrap_or_default()
        );
        let (_span, pending) = gaxi::client_request_signals!(
            metric: self.metric.clone(),
            info: *INSTRUMENTATION,
            method: "client::Storage::write_object",
            async {
                if let Some(recorder) = RequestRecorder::current() {
                    recorder.on_client_request(
                        ClientRequestAttributes::default()
                            .set_url_template("/upload/storage/v1/b/{bucket}/o")
                            .set_resource_name(resource_name),
                    );
                }
                self.write_object_unbuffered_plain(payload, request, options).await
            }
        );
        pending.await
    }

    async fn open_object_plain(
        &self,
        request: OpenObjectRequest,
        options: RequestOptions,
    ) -> Result<(ObjectDescriptor, Vec<ReadObjectResponse>)> {
        let (spec, ranges) = request.into_parts();
        let connector = Connector::new(spec, options, self.inner.grpc.clone());
        let (transport, readers) = ObjectDescriptorTransport::new(connector, ranges).await?;
        Ok((ObjectDescriptor::new(transport), readers))
    }

    #[tracing::instrument(name = "open_object", level = tracing::Level::DEBUG, ret, err(Debug))]
    async fn open_object_tracing(
        &self,
        request: OpenObjectRequest,
        options: RequestOptions,
    ) -> Result<(ObjectDescriptor, Vec<ReadObjectResponse>)> {
        let resource_name = format!("//storage.googleapis.com/{}", request.bucket);
        let (span, pending) = gaxi::client_request_signals!(
            metric: self.metric.clone(),
            info: *INSTRUMENTATION,
            method: "client::Storage::open_object",
            async {
                if let Some(recorder) = RequestRecorder::current() {
                    recorder.on_client_request(
                        ClientRequestAttributes::default()
                            .set_rpc_method("google.storage.v2.Storage/BidiStreamingRead")
                            .set_url_template("/upload/storage/v1/b/{bucket}/o")
                            .set_resource_name(resource_name),
                    );
                }
                self.open_object_plain(request, options).await
            }
        );
        let (descriptor, readers) = pending.await?;
        let descriptor =
            ObjectDescriptor::new(TracingObjectDescriptor::new(descriptor.into_parts()));
        let readers = readers
            .into_iter()
            .map(|r| {
                let inner = r.into_parts();
                ReadObjectResponse::new(Box::new(TracingResponse::new(inner, span.clone())))
            })
            .collect::<Vec<_>>();
        Ok((descriptor, readers))
    }

    async fn service_account_plain(
        &self,
        project: String,
        options: RequestOptions,
    ) -> Result<String> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::service_account_attempt(inner.clone(), &project, &options, current).await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn service_account_attempt(
        inner: Arc<StorageInner>,
        project: &str,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<String> {
        let builder = inner
            .client
            .http_builder(
                Method::GET,
                &format!("/storage/v1/projects/{}/serviceAccount", enc(project)),
            )
            .header(
                "x-goog-api-client",
                HeaderValue::from_static(&X_GOOG_API_CLIENT_HEADER),
            );
        let options = options
            .gax()
            .insert_extension(PathTemplate(
                "/storage/v1/projects/{project}/serviceAccount",
            ))
            .insert_extension(ResourceName(format!(
                "//storage.googleapis.com/projects/{project}"
            )));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response =
            serde_json::from_slice::<ServiceAccountResponse>(&body).map_err(Error::deser)?;
        Ok(response.email_address)
    }

    #[tracing::instrument(name = "service_account", level = tracing::Level::DEBUG, ret, err(Debug))]
    async fn service_account_tracing(
        &self,
        project: String,
        options: RequestOptions,
    ) -> Result<String> {
        let resource_name = format!("//storage.googleapis.com/projects/{project}");
        let (_span, pending) = gaxi::client_request_signals!(
            metric: self.metric.clone(),
            info: *INSTRUMENTATION,
            method: "client::Storage::service_account",
            async {
                if let Some(recorder) = RequestRecorder::current() {
                    recorder.on_client_request(
                        ClientRequestAttributes::default()
                            .set_url_template("/storage/v1/projects/{project}/serviceAccount")
                            .set_resource_name(resource_name),
                    );
                }
                self.service_account_plain(project, options).await
            }
        );
        pending.await
    }

    async fn list_bucket_acls_plain(
        &self,
        bucket: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.list_acls_plain(
            format!("/storage/v1/b/{}/acl", enc(&bucket_id)),
            "/storage/v1/b/{bucket}/acl",
            format!("//storage.googleapis.com/{bucket}"),
            options,
        )
        .await
    }

    async fn update_bucket_acl_plain(
        &self,
        bucket: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.update_acl_plain(
            format!("/storage/v1/b/{}/acl/{}", enc(&bucket_id), enc(&entity)),
            "/storage/v1/b/{bucket}/acl/{entity}",
            format!("//storage.googleapis.com/{bucket}"),
            entity,
            role,
            options,
        )
        .await
    }

    async fn delete_bucket_acl_plain(
        &self,
        bucket: String,
        entity: String,
        options: RequestOptions,
    ) -> Result<()> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.delete_acl_plain(
            format!("/storage/v1/b/{}/acl/{}", enc(&bucket_id), enc(&entity)),
            "/storage/v1/b/{bucket}/acl/{entity}",
            format!("//storage.googleapis.com/{bucket}"),
            options,
        )
        .await
    }

    async fn list_default_object_acls_plain(
        &self,
        bucket: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.list_acls_plain(
            format!("/storage/v1/b/{}/defaultObjectAcl", enc(&bucket_id)),
            "/storage/v1/b/{bucket}/defaultObjectAcl",
            format!("//storage.googleapis.com/{bucket}"),
            options,
        )
        .await
    }

    async fn update_default_object_acl_plain(
        &self,
        bucket: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.update_acl_plain(
            format!(
                "/storage/v1/b/{}/defaultObjectAcl/{}",
                enc(&bucket_id),
                enc(&entity)
            ),
            "/storage/v1/b/{bucket}/defaultObjectAcl/{entity}",
            format!("//storage.googleapis.com/{bucket}"),
            entity,
            role,
            options,
        )
        .await
    }

    async fn delete_default_object_acl_plain(
        &self,
        bucket: String,
        entity: String,
        options: RequestOptions,
    ) -> Result<()> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.delete_acl_plain(
            format!(
                "/storage/v1/b/{}/defaultObjectAcl/{}",
                enc(&bucket_id),
                enc(&entity)
            ),
            "/storage/v1/b/{bucket}/defaultObjectAcl/{entity}",
            format!("//storage.googleapis.com/{bucket}"),
            options,
        )
        .await
    }

    async fn list_object_acls_plain(
        &self,
        bucket: String,
        object: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.list_acls_plain(
            format!("/storage/v1/b/{}/o/{}/acl", enc(&bucket_id), enc(&object)),
            "/storage/v1/b/{bucket}/o/{object}/acl",
            format!("//storage.googleapis.com/{bucket}"),
            options,
        )
        .await
    }

    async fn update_object_acl_plain(
        &self,
        bucket: String,
        object: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.update_acl_plain(
            format!(
                "/storage/v1/b/{}/o/{}/acl/{}",
                enc(&bucket_id),
                enc(&object),
                enc(&entity)
            ),
            "/storage/v1/b/{bucket}/o/{object}/acl/{entity}",
            format!("//storage.googleapis.com/{bucket}"),
            entity,
            role,
            options,
        )
        .await
    }

    async fn delete_object_acl_plain(
        &self,
        bucket: String,
        object: String,
        entity: String,
        options: RequestOptions,
    ) -> Result<()> {
        let bucket_id = bucket_id(&bucket)?.to_string();
        self.delete_acl_plain(
            format!(
                "/storage/v1/b/{}/o/{}/acl/{}",
                enc(&bucket_id),
                enc(&object),
                enc(&entity)
            ),
            "/storage/v1/b/{bucket}/o/{object}/acl/{entity}",
            format!("//storage.googleapis.com/{bucket}"),
            options,
        )
        .await
    }

    async fn list_acls_plain(
        &self,
        path: String,
        path_template: &'static str,
        resource_name: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        let operation = AclOperation {
            path,
            path_template,
            resource_name,
        };
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::list_acls_attempt(inner.clone(), operation.clone(), options.clone(), current)
                .await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn list_acls_attempt(
        inner: Arc<StorageInner>,
        operation: AclOperation,
        options: RequestOptions,
        attempt_count: u32,
    ) -> Result<Vec<AclRule>> {
        let builder = acl_builder(inner, Method::GET, &operation.path, &options);
        let options = options
            .gax()
            .insert_extension(PathTemplate(operation.path_template))
            .insert_extension(ResourceName(operation.resource_name));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response = serde_json::from_slice::<RawAclList>(&body).map_err(Error::deser)?;
        Ok(response.items.into_iter().map(AclRule::from).collect())
    }

    async fn update_acl_plain(
        &self,
        path: String,
        path_template: &'static str,
        resource_name: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        let operation = AclUpdateOperation {
            acl: AclOperation {
                path,
                path_template,
                resource_name,
            },
            entity,
            role,
        };
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::update_acl_attempt(inner.clone(), operation.clone(), options.clone(), current)
                .await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            false,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn update_acl_attempt(
        inner: Arc<StorageInner>,
        operation: AclUpdateOperation,
        options: RequestOptions,
        attempt_count: u32,
    ) -> Result<()> {
        let body = serde_json::to_vec(&RawAclRule::for_update(operation.entity, operation.role))
            .map_err(Error::ser)?;
        let builder = acl_builder(inner, Method::PUT, &operation.acl.path, &options)
            .header("content-type", HeaderValue::from_static("application/json"))
            .body(body);
        let options = options
            .gax()
            .insert_extension(PathTemplate(operation.acl.path_template))
            .insert_extension(ResourceName(operation.acl.resource_name));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        Ok(())
    }

    async fn delete_acl_plain(
        &self,
        path: String,
        path_template: &'static str,
        resource_name: String,
        options: RequestOptions,
    ) -> Result<()> {
        let operation = AclOperation {
            path,
            path_template,
            resource_name,
        };
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::delete_acl_attempt(inner.clone(), operation.clone(), options.clone(), current)
                .await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            false,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn delete_acl_attempt(
        inner: Arc<StorageInner>,
        operation: AclOperation,
        options: RequestOptions,
        attempt_count: u32,
    ) -> Result<()> {
        let builder = acl_builder(inner, Method::DELETE, &operation.path, &options);
        let options = options
            .gax()
            .insert_extension(PathTemplate(operation.path_template))
            .insert_extension(ResourceName(operation.resource_name));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        Ok(())
    }

    async fn list_notifications_plain(
        &self,
        bucket: String,
        options: RequestOptions,
    ) -> Result<BTreeMap<String, Notification>> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::list_notifications_attempt(inner.clone(), &bucket, &options, current).await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn list_notifications_attempt(
        inner: Arc<StorageInner>,
        bucket: &str,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<BTreeMap<String, Notification>> {
        let bucket_id = bucket_id(bucket)?;
        let builder = notification_builder(
            inner,
            Method::GET,
            &format!("/storage/v1/b/{}/notificationConfigs", enc(bucket_id)),
            options,
        );
        let options = options
            .gax()
            .insert_extension(PathTemplate("/storage/v1/b/{bucket}/notificationConfigs"))
            .insert_extension(ResourceName(format!("//storage.googleapis.com/{bucket}")));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response =
            serde_json::from_slice::<ListNotificationsResponse>(&body).map_err(Error::deser)?;
        Ok(notifications_to_map(response.items))
    }

    async fn create_notification_plain(
        &self,
        bucket: String,
        notification: Notification,
        options: RequestOptions,
    ) -> Result<Notification> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::create_notification_attempt(
                inner.clone(),
                &bucket,
                notification.clone(),
                &options,
                current,
            )
            .await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            false,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn create_notification_attempt(
        inner: Arc<StorageInner>,
        bucket: &str,
        notification: Notification,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<Notification> {
        let bucket_id = bucket_id(bucket)?;
        let raw = RawNotification::from(notification);
        let body = serde_json::to_vec(&raw).map_err(Error::ser)?;
        let builder = notification_builder(
            inner,
            Method::POST,
            &format!("/storage/v1/b/{}/notificationConfigs", enc(bucket_id)),
            options,
        )
        .header("content-type", HeaderValue::from_static("application/json"))
        .body(body);
        let options = options
            .gax()
            .insert_extension(PathTemplate("/storage/v1/b/{bucket}/notificationConfigs"))
            .insert_extension(ResourceName(format!("//storage.googleapis.com/{bucket}")));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response = serde_json::from_slice::<RawNotification>(&body).map_err(Error::deser)?;
        Ok(Notification::from(response))
    }

    async fn delete_notification_plain(
        &self,
        bucket: String,
        notification: String,
        options: RequestOptions,
    ) -> Result<()> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::delete_notification_attempt(
                inner.clone(),
                &bucket,
                &notification,
                &options,
                current,
            )
            .await
        };

        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn delete_notification_attempt(
        inner: Arc<StorageInner>,
        bucket: &str,
        notification: &str,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<()> {
        let bucket_id = bucket_id(bucket)?;
        let builder = notification_builder(
            inner,
            Method::DELETE,
            &format!(
                "/storage/v1/b/{}/notificationConfigs/{}",
                enc(bucket_id),
                enc(notification)
            ),
            options,
        );
        let options = options
            .gax()
            .insert_extension(PathTemplate(
                "/storage/v1/b/{bucket}/notificationConfigs/{notification}",
            ))
            .insert_extension(ResourceName(format!("//storage.googleapis.com/{bucket}")));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        Ok(())
    }

    async fn create_hmac_key_plain(
        &self,
        project: String,
        service_account_email: String,
        options: RequestOptions,
    ) -> Result<HmacKey> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::create_hmac_key_attempt(
                inner.clone(),
                &project,
                &service_account_email,
                &options,
                current,
            )
            .await
        };
        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            false,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn create_hmac_key_attempt(
        inner: Arc<StorageInner>,
        project: &str,
        service_account_email: &str,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<HmacKey> {
        let builder = hmac_builder(
            inner,
            Method::POST,
            &format!("/storage/v1/projects/{}/hmacKeys", enc(project)),
            options,
        )
        .query("serviceAccountEmail", service_account_email);
        let options = options
            .gax()
            .insert_extension(PathTemplate("/storage/v1/projects/{project}/hmacKeys"))
            .insert_extension(ResourceName(format!(
                "//storage.googleapis.com/projects/{project}"
            )));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response = serde_json::from_slice::<RawHmacKey>(&body).map_err(Error::deser)?;
        Ok(HmacKey::from(response))
    }

    async fn get_hmac_key_plain(
        &self,
        project: String,
        access_id: String,
        options: RequestOptions,
    ) -> Result<HmacKey> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::get_hmac_key_attempt(inner.clone(), &project, &access_id, &options, current).await
        };
        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn get_hmac_key_attempt(
        inner: Arc<StorageInner>,
        project: &str,
        access_id: &str,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<HmacKey> {
        let builder = hmac_builder(
            inner,
            Method::GET,
            &format!(
                "/storage/v1/projects/{}/hmacKeys/{}",
                enc(project),
                enc(access_id)
            ),
            options,
        );
        let options = options
            .gax()
            .insert_extension(PathTemplate(
                "/storage/v1/projects/{project}/hmacKeys/{access_id}",
            ))
            .insert_extension(ResourceName(format!(
                "//storage.googleapis.com/projects/{project}"
            )));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response = serde_json::from_slice::<RawHmacKeyMetadata>(&body).map_err(Error::deser)?;
        Ok(HmacKey::from(response))
    }

    async fn update_hmac_key_plain(
        &self,
        project: String,
        access_id: String,
        update: HmacKeyUpdate,
        options: RequestOptions,
    ) -> Result<HmacKey> {
        let idempotent = !update.etag.is_empty();
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::update_hmac_key_attempt(
                inner.clone(),
                &project,
                &access_id,
                update.clone(),
                &options,
                current,
            )
            .await
        };
        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            idempotent,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn update_hmac_key_attempt(
        inner: Arc<StorageInner>,
        project: &str,
        access_id: &str,
        update: HmacKeyUpdate,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<HmacKey> {
        let body = serde_json::to_vec(&RawHmacKeyMetadata::from(update)).map_err(Error::ser)?;
        let builder = hmac_builder(
            inner,
            Method::PUT,
            &format!(
                "/storage/v1/projects/{}/hmacKeys/{}",
                enc(project),
                enc(access_id)
            ),
            options,
        )
        .header("content-type", HeaderValue::from_static("application/json"))
        .body(body);
        let options = options
            .gax()
            .insert_extension(PathTemplate(
                "/storage/v1/projects/{project}/hmacKeys/{access_id}",
            ))
            .insert_extension(ResourceName(format!(
                "//storage.googleapis.com/projects/{project}"
            )));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response = serde_json::from_slice::<RawHmacKeyMetadata>(&body).map_err(Error::deser)?;
        Ok(HmacKey::from(response))
    }

    async fn delete_hmac_key_plain(
        &self,
        project: String,
        access_id: String,
        options: RequestOptions,
    ) -> Result<()> {
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::delete_hmac_key_attempt(inner.clone(), &project, &access_id, &options, current)
                .await
        };
        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn delete_hmac_key_attempt(
        inner: Arc<StorageInner>,
        project: &str,
        access_id: &str,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<()> {
        let builder = hmac_builder(
            inner,
            Method::DELETE,
            &format!(
                "/storage/v1/projects/{}/hmacKeys/{}",
                enc(project),
                enc(access_id)
            ),
            options,
        );
        let options = options
            .gax()
            .insert_extension(PathTemplate(
                "/storage/v1/projects/{project}/hmacKeys/{access_id}",
            ))
            .insert_extension(ResourceName(format!(
                "//storage.googleapis.com/projects/{project}"
            )));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        Ok(())
    }

    async fn list_hmac_keys_plain(
        &self,
        project: String,
        service_account_email: Option<String>,
        show_deleted_keys: bool,
        page_size: Option<i64>,
        page_token: Option<String>,
        options: RequestOptions,
    ) -> Result<ListHmacKeysResponse> {
        let params = ListHmacKeysParams {
            project,
            service_account_email,
            show_deleted_keys,
            page_size,
            page_token,
        };
        let inner = self.inner.clone();
        let throttler = options.retry_throttler.clone();
        let retry = options.retry_policy.clone();
        let backoff = options.backoff_policy.clone();
        let mut count = 0;
        let attempt = async move |_| {
            let current = count;
            count += 1;
            Self::list_hmac_keys_attempt(inner.clone(), &params, &options, current).await
        };
        google_cloud_gax::retry_loop_internal::retry_loop(
            attempt,
            async |duration| tokio::time::sleep(duration).await,
            true,
            throttler,
            retry,
            backoff,
        )
        .await
    }

    async fn list_hmac_keys_attempt(
        inner: Arc<StorageInner>,
        params: &ListHmacKeysParams,
        options: &RequestOptions,
        attempt_count: u32,
    ) -> Result<ListHmacKeysResponse> {
        let builder = hmac_builder(
            inner,
            Method::GET,
            &format!("/storage/v1/projects/{}/hmacKeys", enc(&params.project)),
            options,
        );
        let builder = params
            .service_account_email
            .as_deref()
            .into_iter()
            .fold(builder, |b, v| b.query("serviceAccountEmail", v));
        let builder = if params.show_deleted_keys {
            builder.query("showDeletedKeys", true)
        } else {
            builder
        };
        let builder = params
            .page_size
            .into_iter()
            .fold(builder, |b, v| b.query("maxResults", v));
        let builder = params
            .page_token
            .as_deref()
            .into_iter()
            .fold(builder, |b, v| b.query("pageToken", v));
        let options = options
            .gax()
            .insert_extension(PathTemplate("/storage/v1/projects/{project}/hmacKeys"))
            .insert_extension(ResourceName(format!(
                "//storage.googleapis.com/projects/{}",
                params.project
            )));
        let response = builder
            .send(options, AttemptInfo::new(attempt_count))
            .await?;
        if !response.status().is_success() {
            return gaxi::http::to_http_error(response).await;
        }
        let body = response.bytes().await.map_err(Error::io)?;
        let response =
            serde_json::from_slice::<RawListHmacKeysResponse>(&body).map_err(Error::deser)?;
        Ok(ListHmacKeysResponse::from(response))
    }
}

fn bucket_id(bucket: &str) -> Result<&str> {
    bucket.strip_prefix("projects/_/buckets/").ok_or_else(|| {
        Error::binding(format!(
            "malformed bucket name, it must start with `projects/_/buckets/`: {bucket}"
        ))
    })
}

fn acl_builder(
    inner: Arc<StorageInner>,
    method: Method,
    path: &str,
    options: &RequestOptions,
) -> gaxi::http::HttpRequestBuilder {
    let builder = inner.client.http_builder(method, path).header(
        "x-goog-api-client",
        HeaderValue::from_static(&X_GOOG_API_CLIENT_HEADER),
    );
    options
        .user_project()
        .into_iter()
        .fold(builder, |builder, user_project| {
            builder.query("userProject", user_project)
        })
}

fn notification_builder(
    inner: Arc<StorageInner>,
    method: Method,
    path: &str,
    options: &RequestOptions,
) -> gaxi::http::HttpRequestBuilder {
    let builder = inner.client.http_builder(method, path).header(
        "x-goog-api-client",
        HeaderValue::from_static(&X_GOOG_API_CLIENT_HEADER),
    );
    options
        .user_project()
        .into_iter()
        .fold(builder, |builder, user_project| {
            builder.query("userProject", user_project)
        })
}

#[derive(Clone)]
struct AclOperation {
    path: String,
    path_template: &'static str,
    resource_name: String,
}

#[derive(Clone)]
struct AclUpdateOperation {
    acl: AclOperation,
    entity: String,
    role: String,
}

struct ListHmacKeysParams {
    project: String,
    service_account_email: Option<String>,
    show_deleted_keys: bool,
    page_size: Option<i64>,
    page_token: Option<String>,
}

fn hmac_builder(
    inner: Arc<StorageInner>,
    method: Method,
    path: &str,
    options: &RequestOptions,
) -> gaxi::http::HttpRequestBuilder {
    let builder = inner.client.http_builder(method, path).header(
        "x-goog-api-client",
        HeaderValue::from_static(&X_GOOG_API_CLIENT_HEADER),
    );
    options
        .user_project()
        .into_iter()
        .fold(builder, |builder, user_project| {
            builder.query("userProject", user_project)
        })
}

impl super::stub::Storage for Storage {
    /// Implements [crate::client::Storage::read_object].
    async fn read_object(
        &self,
        req: ReadObjectRequest,
        options: RequestOptions,
    ) -> Result<ReadObjectResponse> {
        if self.tracing {
            return self.read_object_tracing(req, options).await;
        }
        self.read_object_plain(req, options).await
    }

    /// Implements [crate::client::Storage::write_object].
    async fn write_object_buffered<P>(
        &self,
        payload: P,
        req: WriteObjectRequest,
        options: RequestOptions,
    ) -> Result<Object>
    where
        P: StreamingSource + Send + Sync + 'static,
    {
        if self.tracing {
            return self
                .write_object_buffered_tracing(payload, req, options)
                .await;
        }
        self.write_object_buffered_plain(payload, req, options)
            .await
    }

    /// Implements [crate::client::Storage::write_object].
    async fn write_object_unbuffered<P>(
        &self,
        payload: P,
        req: WriteObjectRequest,
        options: RequestOptions,
    ) -> Result<Object>
    where
        P: StreamingSource + Seek + Send + Sync + 'static,
    {
        if self.tracing {
            return self
                .write_object_unbuffered_tracing(payload, req, options)
                .await;
        }
        self.write_object_unbuffered_plain(payload, req, options)
            .await
    }

    async fn open_object(
        &self,
        request: OpenObjectRequest,
        options: RequestOptions,
    ) -> Result<(ObjectDescriptor, Vec<ReadObjectResponse>)> {
        if self.tracing {
            return self.open_object_tracing(request, options).await;
        }
        self.open_object_plain(request, options).await
    }

    async fn service_account(&self, project: String, options: RequestOptions) -> Result<String> {
        if self.tracing {
            return self.service_account_tracing(project, options).await;
        }
        self.service_account_plain(project, options).await
    }

    async fn list_bucket_acls(
        &self,
        bucket: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        self.list_bucket_acls_plain(bucket, options).await
    }

    async fn update_bucket_acl(
        &self,
        bucket: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.update_bucket_acl_plain(bucket, entity, role, options)
            .await
    }

    async fn delete_bucket_acl(
        &self,
        bucket: String,
        entity: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.delete_bucket_acl_plain(bucket, entity, options).await
    }

    async fn list_default_object_acls(
        &self,
        bucket: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        self.list_default_object_acls_plain(bucket, options).await
    }

    async fn update_default_object_acl(
        &self,
        bucket: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.update_default_object_acl_plain(bucket, entity, role, options)
            .await
    }

    async fn delete_default_object_acl(
        &self,
        bucket: String,
        entity: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.delete_default_object_acl_plain(bucket, entity, options)
            .await
    }

    async fn list_object_acls(
        &self,
        bucket: String,
        object: String,
        options: RequestOptions,
    ) -> Result<Vec<AclRule>> {
        self.list_object_acls_plain(bucket, object, options).await
    }

    async fn update_object_acl(
        &self,
        bucket: String,
        object: String,
        entity: String,
        role: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.update_object_acl_plain(bucket, object, entity, role, options)
            .await
    }

    async fn delete_object_acl(
        &self,
        bucket: String,
        object: String,
        entity: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.delete_object_acl_plain(bucket, object, entity, options)
            .await
    }

    async fn list_notifications(
        &self,
        bucket: String,
        options: RequestOptions,
    ) -> Result<BTreeMap<String, Notification>> {
        self.list_notifications_plain(bucket, options).await
    }

    async fn create_notification(
        &self,
        bucket: String,
        notification: Notification,
        options: RequestOptions,
    ) -> Result<Notification> {
        self.create_notification_plain(bucket, notification, options)
            .await
    }

    async fn delete_notification(
        &self,
        bucket: String,
        notification: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.delete_notification_plain(bucket, notification, options)
            .await
    }

    async fn create_hmac_key(
        &self,
        project: String,
        service_account_email: String,
        options: RequestOptions,
    ) -> Result<HmacKey> {
        self.create_hmac_key_plain(project, service_account_email, options)
            .await
    }

    async fn get_hmac_key(
        &self,
        project: String,
        access_id: String,
        options: RequestOptions,
    ) -> Result<HmacKey> {
        self.get_hmac_key_plain(project, access_id, options).await
    }

    async fn update_hmac_key(
        &self,
        project: String,
        access_id: String,
        update: HmacKeyUpdate,
        options: RequestOptions,
    ) -> Result<HmacKey> {
        self.update_hmac_key_plain(project, access_id, update, options)
            .await
    }

    async fn delete_hmac_key(
        &self,
        project: String,
        access_id: String,
        options: RequestOptions,
    ) -> Result<()> {
        self.delete_hmac_key_plain(project, access_id, options)
            .await
    }

    async fn list_hmac_keys(
        &self,
        project: String,
        service_account_email: Option<String>,
        show_deleted_keys: bool,
        page_size: Option<i64>,
        page_token: Option<String>,
        options: RequestOptions,
    ) -> Result<ListHmacKeysResponse> {
        self.list_hmac_keys_plain(
            project,
            service_account_email,
            show_deleted_keys,
            page_size,
            page_token,
            options,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::{Storage, StorageInner};
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use google_cloud_test_utils::test_layer::AttributeValue;
    use google_cloud_test_utils::test_layer::{CapturedSpan, TestLayer};
    use httptest::{Expectation, Server, matchers::*, responders::status_code};
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;
    use std::sync::Arc;

    impl Storage {
        pub(crate) fn new_test(inner: Arc<StorageInner>) -> Arc<Self> {
            Self::new(inner, false)
        }
    }

    #[tokio::test]
    async fn read_object() -> anyhow::Result<()> {
        let guard = TestLayer::initialize();

        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/storage/v1/b/test-bucket/o/test-object"),
                request::query(url_decoded(contains(("alt", "media")))),
            ])
            .respond_with(status_code(404)),
        );

        let client = crate::client::Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .with_tracing()
            .build()
            .await?;
        let response = client
            .read_object("projects/_/buckets/test-bucket", "test-object")
            .send()
            .await;
        assert!(
            matches!(response, Err(ref e) if e.is_transport()),
            "{response:?}"
        );

        let captured = TestLayer::capture(&guard);
        check_debug_log(&captured, "read_object");

        client_request_span(&captured, "read_object", "404", "http");

        Ok(())
    }

    #[tokio::test]
    async fn read_object_success() -> anyhow::Result<()> {
        let guard = TestLayer::initialize();

        let body = (0..100_000)
            .map(|i| format!("{i:08} {:1000}", ""))
            .collect::<Vec<_>>()
            .join("\n");
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/storage/v1/b/test-bucket/o/test-object"),
                request::query(url_decoded(contains(("alt", "media")))),
            ])
            .respond_with(
                status_code(200)
                    .body(body.clone())
                    .append_header("x-goog-generation", 123456),
            ),
        );

        let client = crate::client::Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .with_tracing()
            .build()
            .await?;
        let mut got = Vec::new();
        let mut response = client
            .read_object("projects/_/buckets/test-bucket", "test-object")
            .send()
            .await?;
        let object = response.object();
        assert_eq!(object.generation, 123456, "{object:?}");
        while let Some(b) = response.next().await.transpose()? {
            got.push(b);
        }

        let captured = TestLayer::capture(&guard);
        let span = captured
            .iter()
            .find(|s| s.name == "client_request")
            .unwrap_or_else(|| panic!("missing `client_request` span in capture: {captured:#?}"));
        // The span counts one more event: the EOF
        assert_eq!(span.events, got.len() + 1, "{span:?}");

        Ok(())
    }

    #[tokio::test]
    async fn write_object_buffered() -> anyhow::Result<()> {
        let guard = TestLayer::initialize();

        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("POST", "/upload/storage/v1/b/test-bucket/o"),
                request::query(url_decoded(contains(("uploadType", "multipart")))),
            ])
            .respond_with(status_code(404)),
        );

        let client = crate::client::Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .with_tracing()
            .build()
            .await?;
        let response = client
            .write_object("projects/_/buckets/test-bucket", "test-object", "payload")
            .send_buffered()
            .await;
        assert!(
            matches!(response, Err(ref e) if e.is_transport()),
            "{response:?}"
        );

        let captured = TestLayer::capture(&guard);
        check_debug_log(&captured, "write_object_buffered");

        client_request_span(&captured, "write_object", "404", "http");

        Ok(())
    }

    #[tokio::test]
    async fn write_object_unbuffered() -> anyhow::Result<()> {
        let guard = TestLayer::initialize();

        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("POST", "/upload/storage/v1/b/test-bucket/o"),
                request::query(url_decoded(contains(("uploadType", "multipart")))),
            ])
            .respond_with(status_code(404)),
        );

        let client = crate::client::Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .with_tracing()
            .build()
            .await?;
        let response = client
            .write_object("projects/_/buckets/test-bucket", "test-object", "payload")
            .send_unbuffered()
            .await;
        assert!(
            matches!(response, Err(ref e) if e.is_transport()),
            "{response:?}"
        );

        let captured = TestLayer::capture(&guard);
        check_debug_log(&captured, "write_object_unbuffered");

        client_request_span(&captured, "write_object", "404", "http");

        Ok(())
    }

    #[tokio::test]
    async fn open_object() -> anyhow::Result<()> {
        use gaxi::grpc::tonic::Status as TonicStatus;
        use google_cloud_gax::error::rpc::Code;
        use storage_grpc_mock::{MockStorage, start};

        let guard = TestLayer::initialize();

        let mut mock = MockStorage::new();
        mock.expect_bidi_read_object()
            .return_once(|_| Err(TonicStatus::not_found("not here")));
        let (endpoint, _server) = start("0.0.0.0:0", mock).await?;

        let client = crate::client::Storage::builder()
            .with_credentials(Anonymous::new().build())
            .with_endpoint(endpoint.clone())
            .with_tracing()
            .build()
            .await?;
        let response = client
            .open_object("projects/_/buckets/test-bucket", "test-object")
            .send()
            .await;
        assert!(
            matches!(response, Err(ref e) if e.status().is_some_and(|s| s.code == Code::NotFound)),
            "{response:?}"
        );

        let captured = TestLayer::capture(&guard);
        check_debug_log(&captured, "open_object");

        client_request_span(&captured, "open_object", "NOT_FOUND", "grpc");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "flaky test, see #5290"]
    async fn open_object_success() -> anyhow::Result<()> {
        // TODO(#4772) - Move these `use` declarations and constants once the tracing APIs are stable.
        use crate::model_ext::ReadRange;
        use gaxi::grpc::tonic::{Response as TonicResponse, Result as TonicResult};
        use storage_grpc_mock::google::storage::v2::{
            BidiReadObjectResponse, ChecksummedData, Object as ProtoObject, ObjectRangeData,
            ReadRange as ProtoRange,
        };
        use storage_grpc_mock::{MockStorage, start};
        const BUCKET_NAME: &str = "projects/_/buckets/test-bucket";
        const OBJECT_NAME: &str = "test-object";
        const BIND_ADDRESS: &str = "0.0.0.0:0";
        const PAYLOAD: &str = "the quick brown fox jumps over the lazy dog";

        let guard = TestLayer::initialize();

        let (tx, rx) = tokio::sync::mpsc::channel::<TonicResult<BidiReadObjectResponse>>(10);
        let response = BidiReadObjectResponse {
            metadata: Some(ProtoObject {
                bucket: BUCKET_NAME.to_string(),
                name: OBJECT_NAME.to_string(),
                generation: 123456,
                ..ProtoObject::default()
            }),
            object_data_ranges: vec![ObjectRangeData {
                read_range: Some(ProtoRange {
                    read_id: 0_i64,
                    ..ProtoRange::default()
                }),
                range_end: true,
                checksummed_data: Some(ChecksummedData {
                    content: PAYLOAD.as_bytes().to_vec(),
                    crc32c: None,
                }),
            }],
            ..BidiReadObjectResponse::default()
        };
        // This is the initial response.
        tx.send(Ok(response.clone())).await?;
        // These simulate the calls to ObjectDescriptor::read_range(). The data is wrong, but this
        // test is about the spans.
        tx.send(Ok(response.clone())).await?;
        tx.send(Ok(response.clone())).await?;

        let mut mock = MockStorage::new();
        mock.expect_bidi_read_object()
            .return_once(|_| Ok(TonicResponse::from(rx)));
        let (endpoint, _server) = start(BIND_ADDRESS, mock).await?;

        let client = crate::client::Storage::builder()
            .with_credentials(Anonymous::new().build())
            .with_endpoint(endpoint.clone())
            .with_tracing()
            .build()
            .await?;
        let (descriptor, _reader0) = client
            .open_object(BUCKET_NAME, OBJECT_NAME)
            .send_and_read(ReadRange::all())
            .await?;
        let _reader1 = descriptor.read_range(ReadRange::offset(5)).await;
        let _reader2 = descriptor.read_range(ReadRange::segment(10, 10)).await;
        let _reader3 = descriptor.read_range(ReadRange::tail(15)).await;

        let captured = TestLayer::capture(&guard);
        let _span = captured
            .iter()
            .find(|s| s.name == "client_request")
            .unwrap_or_else(|| panic!("missing `client_request` span in capture: {captured:#?}"));

        let range_spans = captured
            .iter()
            .filter(|s| s.name == "read_range")
            .collect::<Vec<_>>();

        let _span_reader1 = range_spans
            .clone()
            .into_iter()
            .find(|s| {
                s.attributes
                    .get("read_range.start")
                    .and_then(|v| v.as_i64())
                    == Some(5)
            })
            .unwrap_or_else(|| {
                panic!("missing `read_range` span for ReadRange::offset(5): {range_spans:#?}")
            });

        let _span_reader2 = range_spans
            .clone()
            .into_iter()
            .find(|s| {
                s.attributes
                    .get("read_range.start")
                    .and_then(|v| v.as_i64())
                    == Some(10)
                    && s.attributes
                        .get("read_range.limit")
                        .and_then(|v| v.as_i64())
                        == Some(10)
            })
            .unwrap_or_else(|| {
                panic!("missing `read_range` span for ReadRange::segment(10, 10): {range_spans:#?}")
            });

        let _span_reader3 = range_spans
            .clone()
            .into_iter()
            .find(|s| {
                s.attributes
                    .get("read_range.start")
                    .and_then(|v| v.as_i64())
                    == Some(-15)
            })
            .unwrap_or_else(|| {
                panic!("missing `read_range` span for ReadRange::tail(15): {range_spans:#?}")
            });
        Ok(())
    }

    #[track_caller]
    fn check_debug_log(captured: &Vec<CapturedSpan>, method: &'static str) {
        let span = captured
            .iter()
            .find(|s| s.name == method)
            .unwrap_or_else(|| panic!("missing `{method}` span in capture: {captured:#?}"));

        let got = BTreeMap::from_iter(span.attributes.clone());
        let want = ["self", "options", "request"];
        let missing = want
            .iter()
            .filter(|k| !got.contains_key(**k))
            .collect::<Vec<_>>();
        assert!(
            missing.is_empty(),
            "missing = {missing:?}\ngot  = {:?}\nwant = {want:?}\nfull = {got:#?}",
            got.keys().collect::<Vec<_>>(),
        );
    }

    #[track_caller]
    fn client_request_span(
        captured: &Vec<CapturedSpan>,
        method: &'static str,
        error_type: &'static str,
        rpc_system: &'static str,
    ) {
        let expected_attributes: [(&str, &str); 6] = [
            ("otel.kind", "Internal"),
            ("rpc.system.name", rpc_system),
            ("otel.status_code", "ERROR"),
            ("gcp.client.service", "storage"),
            ("gcp.client.repo", "googleapis/google-cloud-rust"),
            ("gcp.client.artifact", "google-cloud-storage"),
        ];
        let span = captured
            .iter()
            .find(|s| s.name == "client_request")
            .unwrap_or_else(|| panic!("missing `client_request` span in capture: {captured:#?}"));
        let got = BTreeMap::from_iter(span.attributes.clone());
        // This is a subset of the fields, but good enough to catch most
        // mistakes. Recall that we use a macro, which is already tested.
        let want = BTreeMap::<String, AttributeValue>::from_iter(
            expected_attributes
                .iter()
                .map(|(k, v)| (k.to_string(), AttributeValue::from(*v)))
                .chain(
                    [
                        (
                            "otel.name",
                            format!("google_cloud_storage::client::Storage::{method}").into(),
                        ),
                        ("error.type", error_type.into()),
                    ]
                    .map(|(k, v)| (k.to_string(), v)),
                ),
        );
        let mismatch = want
            .iter()
            .filter(|(k, v)| !got.get(k.as_str()).is_some_and(|g| g == *v))
            .collect::<Vec<_>>();
        assert!(
            mismatch.is_empty(),
            "mismatch = {mismatch:?}\ngot      = {got:?}\nwant     = {want:?}"
        );
    }
}
