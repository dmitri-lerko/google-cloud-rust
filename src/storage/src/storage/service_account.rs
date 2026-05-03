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
use crate::storage::request_options::RequestOptions;

/// The request builder for [Storage::service_account][crate::client::Storage::service_account] calls.
///
/// # Example
/// ```
/// # use google_cloud_storage::client::Storage;
/// # async fn sample(client: &Storage) -> anyhow::Result<()> {
/// let email = client.service_account("my-project").send().await?;
/// println!("service account={email}");
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ServiceAccount<S = crate::storage::transport::Storage>
where
    S: crate::storage::stub::Storage + 'static,
{
    stub: std::sync::Arc<S>,
    project: String,
    options: RequestOptions,
}

impl<S> Clone for ServiceAccount<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stub: self.stub.clone(),
            project: self.project.clone(),
            options: self.options.clone(),
        }
    }
}

impl<S> ServiceAccount<S>
where
    S: crate::storage::stub::Storage + 'static,
{
    pub(crate) fn new<P>(stub: std::sync::Arc<S>, project: P, options: RequestOptions) -> Self
    where
        P: Into<String>,
    {
        Self {
            stub,
            project: project.into(),
            options,
        }
    }

    /// Sends the request.
    pub async fn send(self) -> Result<String> {
        self.stub.service_account(self.project, self.options).await
    }

    /// Appends a user agent string to the request.
    ///
    /// # Example
    /// ```
    /// # use google_cloud_storage::client::Storage;
    /// # async fn sample(client: &Storage) -> anyhow::Result<()> {
    /// let email = client
    ///     .service_account("my-project")
    ///     .with_user_agent("my-app/1.0.0")
    ///     .send()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.with_user_agent(user_agent);
        self
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub(crate) struct ServiceAccountResponse {
    #[serde(rename = "email_address")]
    pub email_address: String,
}

#[cfg(test)]
mod tests {
    use crate::client::Storage;
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use httptest::{Expectation, Server, matchers::*, responders::status_code};
    use pretty_assertions::assert_eq;

    type TestResult = anyhow::Result<()>;

    #[tokio::test]
    async fn service_account_success() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/storage/v1/projects/test-project/serviceAccount",
            ))
            .times(1)
            .respond_with(status_code(200).body(r#"{"email_address":"service-test@example.com"}"#)),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client.service_account("test-project").send().await?;

        assert_eq!(got, "service-test@example.com");

        Ok(())
    }

    #[tokio::test]
    async fn service_account_with_user_agent() -> TestResult {
        use http::header::USER_AGENT;

        let user_agent = "quick_foxes_lazy_dogs/1.2.3";
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/storage/v1/projects/test-project/serviceAccount"),
                request::headers(contains((USER_AGENT.as_str(), user_agent))),
            ])
            .times(1)
            .respond_with(status_code(200).body(r#"{"email_address":"service-test@example.com"}"#)),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .service_account("test-project")
            .with_user_agent(user_agent)
            .send()
            .await?;

        assert_eq!(got, "service-test@example.com");

        Ok(())
    }

    #[tokio::test]
    async fn service_account_project_is_url_encoded() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/storage/v1/projects/billing%20project%2Ftenant%3Aalpha/serviceAccount",
            ))
            .times(1)
            .respond_with(status_code(200).body(r#"{"email_address":"service-test@example.com"}"#)),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let got = client
            .service_account("billing project/tenant:alpha")
            .send()
            .await?;

        assert_eq!(got, "service-test@example.com");

        Ok(())
    }

    #[tokio::test]
    async fn service_account_error() -> TestResult {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/storage/v1/projects/test-project/serviceAccount",
            ))
            .times(1)
            .respond_with(status_code(404).body("not found")),
        );

        let client = Storage::builder()
            .with_endpoint(format!("http://{}", server.addr()))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;
        let err = client
            .service_account("test-project")
            .send()
            .await
            .expect_err("404 should return an error");

        assert!(err.is_transport(), "{err:?}");

        Ok(())
    }
}
