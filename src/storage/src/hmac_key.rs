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

//! Types related to Cloud Storage HMAC keys.

/// An active HMAC key state.
pub const HMAC_KEY_ACTIVE: &str = "ACTIVE";

/// An inactive HMAC key state.
pub const HMAC_KEY_INACTIVE: &str = "INACTIVE";

/// A deleted HMAC key state.
pub const HMAC_KEY_DELETED: &str = "DELETED";

/// A Cloud Storage HMAC key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HmacKey {
    /// The HMAC key secret.
    ///
    /// This is only populated by create operations.
    pub secret: String,
    /// The access ID of the HMAC key.
    pub access_id: String,
    /// The HTTP entity tag for the key metadata.
    pub etag: String,
    /// The resource ID, including the project ID and access ID.
    pub id: String,
    /// The project ID that owns the key.
    pub project_id: String,
    /// The service account email associated with the key.
    pub service_account_email: String,
    /// The creation time of the key.
    pub create_time: Option<wkt::Timestamp>,
    /// The last update time of the key metadata.
    pub update_time: Option<wkt::Timestamp>,
    /// The state of the key.
    pub state: String,
}

/// The mutable attributes for a Cloud Storage HMAC key.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HmacKeyUpdate {
    /// The target state.
    ///
    /// Must be either [HMAC_KEY_ACTIVE] or [HMAC_KEY_INACTIVE].
    pub state: String,
    /// The optional HTTP entity tag for optimistic concurrency control.
    pub etag: String,
}

/// A page of Cloud Storage HMAC keys.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListHmacKeysResponse {
    /// The HMAC keys in this page.
    pub hmac_keys: Vec<HmacKey>,
    /// The token for the next page, if any.
    pub next_page_token: String,
}
