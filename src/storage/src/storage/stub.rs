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

use crate::Result;
use crate::acl::AclRule;
use crate::hmac_key::{HmacKey, HmacKeyUpdate, ListHmacKeysResponse};
use crate::model::{Object, ReadObjectRequest};
use crate::model_ext::WriteObjectRequest;
use crate::notification::Notification;
use crate::read_object::ReadObjectResponse;
use crate::storage::request_options::RequestOptions;
use crate::streaming_source::{Seek, StreamingSource};
use crate::{
    http::HeaderMap,
    model_ext::{OpenObjectRequest, ReadRange},
    object_descriptor::ObjectDescriptor as Descriptor,
};
use gaxi::unimplemented::UNIMPLEMENTED;

/// Defines the trait used to implement [crate::client::Storage].
///
/// Application developers may need to implement this trait to mock
/// `client::Storage`. In other use-cases, application developers only
/// use `client::Storage` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait Storage: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::Storage::read_object].
    fn read_object(
        &self,
        _req: ReadObjectRequest,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<ReadObjectResponse>> + Send {
        unimplemented_stub::<ReadObjectResponse>()
    }

    /// Implements [crate::client::Storage::write_object].
    fn write_object_buffered<P>(
        &self,
        _payload: P,
        _req: WriteObjectRequest,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<Object>> + Send
    where
        P: StreamingSource + Send + Sync + 'static,
    {
        unimplemented_stub::<Object>()
    }

    /// Implements [crate::client::Storage::write_object].
    fn write_object_unbuffered<P>(
        &self,
        _payload: P,
        _req: WriteObjectRequest,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<Object>> + Send
    where
        P: StreamingSource + Seek + Send + Sync + 'static,
    {
        unimplemented_stub::<Object>()
    }

    /// Implements [crate::client::Storage::open_object].
    fn open_object(
        &self,
        _request: OpenObjectRequest,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<(Descriptor, Vec<ReadObjectResponse>)>> + Send
    {
        unimplemented_stub::<(Descriptor, Vec<ReadObjectResponse>)>()
    }

    /// Implements [crate::client::Storage::service_account].
    fn service_account(
        &self,
        _project: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        unimplemented_stub::<String>()
    }

    /// Implements [crate::client::Storage::list_bucket_acls].
    fn list_bucket_acls(
        &self,
        _bucket: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<Vec<AclRule>>> + Send {
        unimplemented_stub::<Vec<AclRule>>()
    }

    /// Implements [crate::client::Storage::update_bucket_acl].
    fn update_bucket_acl(
        &self,
        _bucket: String,
        _entity: String,
        _role: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::delete_bucket_acl].
    fn delete_bucket_acl(
        &self,
        _bucket: String,
        _entity: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::list_default_object_acls].
    fn list_default_object_acls(
        &self,
        _bucket: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<Vec<AclRule>>> + Send {
        unimplemented_stub::<Vec<AclRule>>()
    }

    /// Implements [crate::client::Storage::update_default_object_acl].
    fn update_default_object_acl(
        &self,
        _bucket: String,
        _entity: String,
        _role: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::delete_default_object_acl].
    fn delete_default_object_acl(
        &self,
        _bucket: String,
        _entity: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::list_object_acls].
    fn list_object_acls(
        &self,
        _bucket: String,
        _object: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<Vec<AclRule>>> + Send {
        unimplemented_stub::<Vec<AclRule>>()
    }

    /// Implements [crate::client::Storage::update_object_acl].
    fn update_object_acl(
        &self,
        _bucket: String,
        _object: String,
        _entity: String,
        _role: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::delete_object_acl].
    fn delete_object_acl(
        &self,
        _bucket: String,
        _object: String,
        _entity: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::list_notifications].
    fn list_notifications(
        &self,
        _bucket: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<std::collections::BTreeMap<String, Notification>>> + Send
    {
        unimplemented_stub::<std::collections::BTreeMap<String, Notification>>()
    }

    /// Implements [crate::client::Storage::create_notification].
    fn create_notification(
        &self,
        _bucket: String,
        _notification: Notification,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<Notification>> + Send {
        unimplemented_stub::<Notification>()
    }

    /// Implements [crate::client::Storage::delete_notification].
    fn delete_notification(
        &self,
        _bucket: String,
        _notification: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::create_hmac_key].
    fn create_hmac_key(
        &self,
        _project: String,
        _service_account_email: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<HmacKey>> + Send {
        unimplemented_stub::<HmacKey>()
    }

    /// Implements [crate::client::Storage::get_hmac_key].
    fn get_hmac_key(
        &self,
        _project: String,
        _access_id: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<HmacKey>> + Send {
        unimplemented_stub::<HmacKey>()
    }

    /// Implements [crate::client::Storage::update_hmac_key].
    fn update_hmac_key(
        &self,
        _project: String,
        _access_id: String,
        _update: HmacKeyUpdate,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<HmacKey>> + Send {
        unimplemented_stub::<HmacKey>()
    }

    /// Implements [crate::client::Storage::delete_hmac_key].
    fn delete_hmac_key(
        &self,
        _project: String,
        _access_id: String,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        unimplemented_stub::<()>()
    }

    /// Implements [crate::client::Storage::list_hmac_keys].
    fn list_hmac_keys(
        &self,
        _project: String,
        _service_account_email: Option<String>,
        _show_deleted_keys: bool,
        _page_size: Option<i64>,
        _page_token: Option<String>,
        _options: RequestOptions,
    ) -> impl std::future::Future<Output = Result<ListHmacKeysResponse>> + Send {
        unimplemented_stub::<ListHmacKeysResponse>()
    }
}

/// Defines the trait used to implement [crate::object_descriptor::ObjectDescriptor].
///
/// Application developers may need to implement this trait to mock
/// `ObjectDescriptor`. In other use-cases, application developers
/// should use `ObjectDescriptor` directly, and need not be concerned
/// with this trait or its implementations.
pub trait ObjectDescriptor: std::fmt::Debug + Send + Sync {
    /// The implementation for [ObjectDescriptor::object][Descriptor::object].
    fn object(&self) -> Object;

    /// The implementation for [ObjectDescriptor::read_range][Descriptor::read_range].
    fn read_range(&self, range: ReadRange) -> impl Future<Output = ReadObjectResponse> + Send;

    /// The implementation for [ObjectDescriptor::headers][Descriptor::headers].
    fn headers(&self) -> HeaderMap;
}

async fn unimplemented_stub<T>() -> google_cloud_gax::Result<T> {
    unimplemented!("{UNIMPLEMENTED}");
}
