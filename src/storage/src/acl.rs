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

//! Types related to Cloud Storage access control lists.

/// ACL role granting ownership permissions.
pub const ACL_ROLE_OWNER: &str = "OWNER";

/// ACL role granting read permissions.
pub const ACL_ROLE_READER: &str = "READER";

/// ACL role granting write permissions.
pub const ACL_ROLE_WRITER: &str = "WRITER";

/// ACL entity representing all users.
pub const ACL_ENTITY_ALL_USERS: &str = "allUsers";

/// ACL entity representing all authenticated users.
pub const ACL_ENTITY_ALL_AUTHENTICATED_USERS: &str = "allAuthenticatedUsers";

/// A Cloud Storage access control list rule.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AclRule {
    /// The entity granted access.
    pub entity: String,
    /// The stable ID for the entity, when returned by the service.
    pub entity_id: String,
    /// The role granted to the entity.
    pub role: String,
    /// The domain associated with the entity, when any.
    pub domain: String,
    /// The email address associated with the entity, when any.
    pub email: String,
    /// The project team associated with the entity, when any.
    pub project_team: Option<ProjectTeam>,
    /// The HTTP entity tag for the ACL rule.
    pub etag: String,
}

/// A project team associated with an ACL entity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProjectTeam {
    /// The project number.
    pub project_number: String,
    /// The team name.
    pub team: String,
}
