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

//! Types related to Cloud Storage Pub/Sub notification configurations.

use std::collections::BTreeMap;

/// Sends no payload with notification messages.
pub const NO_PAYLOAD: &str = "NONE";

/// Sends object metadata as JSON with notification messages.
pub const JSON_PAYLOAD: &str = "JSON_API_V1";

/// Event that occurs when an object is successfully created.
pub const OBJECT_FINALIZE_EVENT: &str = "OBJECT_FINALIZE";

/// Event that occurs when the metadata of an existing object changes.
pub const OBJECT_METADATA_UPDATE_EVENT: &str = "OBJECT_METADATA_UPDATE";

/// Event that occurs when an object is permanently deleted.
pub const OBJECT_DELETE_EVENT: &str = "OBJECT_DELETE";

/// Event that occurs when the live version of an object becomes archived.
pub const OBJECT_ARCHIVE_EVENT: &str = "OBJECT_ARCHIVE";

/// A Cloud Storage Pub/Sub notification configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Notification {
    /// The ID of the notification configuration.
    pub id: String,
    /// The ID of the Pub/Sub topic to which this configuration publishes.
    pub topic_id: String,
    /// The ID of the project that owns the Pub/Sub topic.
    pub topic_project_id: String,
    /// Only send notifications about these event types.
    ///
    /// If empty, Cloud Storage sends notifications for all event types.
    pub event_types: Vec<String>,
    /// Only apply this notification configuration to object names with this prefix.
    pub object_name_prefix: String,
    /// Additional attributes attached to each Pub/Sub message.
    pub custom_attributes: BTreeMap<String, String>,
    /// The payload format for published messages.
    pub payload_format: String,
}
