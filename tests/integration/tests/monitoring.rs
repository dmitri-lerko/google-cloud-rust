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

//! Verify Cloud Monitoring generated clients route requests through HTTP/JSON.

#[cfg(test)]
mod monitoring {
    use google_cloud_auth::credentials::anonymous::Builder as Anonymous;
    use google_cloud_monitoring_dashboard_v1::client::DashboardsService;
    use google_cloud_monitoring_dashboard_v1::model::Dashboard;
    use google_cloud_monitoring_metricsscope_v1::client::MetricsScopes;
    use google_cloud_monitoring_metricsscope_v1::model::MonitoredProject;
    use google_cloud_monitoring_v3::client::MetricService;
    use httptest::{Expectation, Server, matchers::*, responders::*};
    use serde_json::json;

    const FILTER: &str = r#"metric.type = "compute.googleapis.com/instance/cpu/utilization""#;

    #[tokio::test(flavor = "multi_thread")]
    async fn metric_service_list_time_series_uses_monitoring_v3_route() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("GET", "/monitoring/v3/projects/test-project/timeSeries"),
                request::query(url_decoded(contains(("filter", FILTER)))),
                request::query(url_decoded(contains(("pageSize", "25")))),
                request::query(url_decoded(contains(("$alt", "json;enum-encoding=int")))),
            ])
            .respond_with(json_encoded(json!({
                "timeSeries": [],
                "nextPageToken": "next-page"
            }))),
        );

        let client = MetricService::builder()
            .with_endpoint(endpoint(&server))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;

        let response = client
            .list_time_series()
            .set_name("projects/test-project")
            .set_filter(FILTER)
            .set_page_size(25)
            .send()
            .await?;

        assert_eq!(response.next_page_token, "next-page");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn dashboards_service_create_dashboard_uses_monitoring_v1_route() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path("POST", "/monitoring/v1/projects/test-project/dashboards"),
                request::query(url_decoded(contains(("validateOnly", "true")))),
                request::query(url_decoded(contains(("$alt", "json;enum-encoding=int")))),
                request::body(json_decoded(eq(json!({
                    "displayName": "CPU dashboard"
                })))),
            ])
            .respond_with(json_encoded(json!({
                "name": "projects/test-project/dashboards/dashboard-1",
                "displayName": "CPU dashboard"
            }))),
        );

        let client = DashboardsService::builder()
            .with_endpoint(endpoint(&server))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;

        let response = client
            .create_dashboard()
            .set_parent("projects/test-project")
            .set_dashboard(Dashboard::new().set_display_name("CPU dashboard"))
            .set_validate_only(true)
            .send()
            .await?;

        assert_eq!(response.display_name, "CPU dashboard");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn metrics_scopes_list_by_monitored_project_uses_monitoring_v1_route()
    -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path(
                    "GET",
                    "/monitoring/v1/locations/global/metricsScopes:listMetricsScopesByMonitoredProject",
                ),
                request::query(url_decoded(contains((
                    "monitoredResourceContainer",
                    "projects/monitored-project"
                )))),
                request::query(url_decoded(contains((
                    "$alt",
                    "json;enum-encoding=int"
                )))),
            ])
            .respond_with(json_encoded(json!({
                "metricsScopes": [
                    {"name": "locations/global/metricsScopes/scoping-project"}
                ]
            }))),
        );

        let client = MetricsScopes::builder()
            .with_endpoint(endpoint(&server))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;

        let response = client
            .list_metrics_scopes_by_monitored_project()
            .set_monitored_resource_container("projects/monitored-project")
            .send()
            .await?;

        assert_eq!(response.metrics_scopes.len(), 1);
        assert_eq!(
            response.metrics_scopes[0].name,
            "locations/global/metricsScopes/scoping-project"
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn metrics_scopes_create_monitored_project_exposes_lro_routes() -> anyhow::Result<()> {
        let server = Server::run();
        server.expect(
            Expectation::matching(all_of![
                request::method_path(
                    "POST",
                    "/monitoring/v1/locations/global/metricsScopes/scoping-project/projects",
                ),
                request::query(url_decoded(contains((
                    "$alt",
                    "json;enum-encoding=int"
                )))),
                request::body(json_decoded(eq(json!({
                    "name": "locations/global/metricsScopes/scoping-project/projects/monitored-project"
                })))),
            ])
            .respond_with(json_encoded(json!({
                "name": "operations/create-monitored-project-1"
            }))),
        );
        server.expect(
            Expectation::matching(all_of![
                request::method_path(
                    "GET",
                    "/monitoring/v1/operations/create-monitored-project-1",
                ),
                request::query(url_decoded(contains(("$alt", "json;enum-encoding=int")))),
            ])
            .respond_with(json_encoded(json!({
                "name": "operations/create-monitored-project-1",
                "done": true
            }))),
        );

        let client = MetricsScopes::builder()
            .with_endpoint(endpoint(&server))
            .with_credentials(Anonymous::new().build())
            .build()
            .await?;

        let operation = client
            .create_monitored_project()
            .set_parent("locations/global/metricsScopes/scoping-project")
            .set_monitored_project(MonitoredProject::new().set_name(
                "locations/global/metricsScopes/scoping-project/projects/monitored-project",
            ))
            .send()
            .await?;

        assert_eq!(operation.name, "operations/create-monitored-project-1");

        let operation = client
            .get_operation()
            .set_name("operations/create-monitored-project-1")
            .send()
            .await?;

        assert!(operation.done);
        Ok(())
    }

    fn endpoint(server: &Server) -> String {
        server.url_str("/monitoring")
    }
}
