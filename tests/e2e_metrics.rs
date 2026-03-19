//! E2E tests for the /metrics endpoint.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::TestServer;
use reqwest::StatusCode;

#[tokio::test]
async fn metrics_returns_200() {
    let srv = TestServer::start().await;
    let resp = srv.metrics().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn metrics_body_is_not_empty_after_request() {
    let srv = TestServer::start().await;

    // Make a request first so counters get recorded
    drop(srv.health().await);

    let body = srv.metrics().await.text().await.expect("read body");
    assert!(!body.is_empty(), "Metrics body should not be empty");
}

#[tokio::test]
async fn metrics_does_not_require_auth() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .get(srv.url("/metrics"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn metrics_post_returns_method_not_allowed() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/metrics"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn metrics_reflect_check_requests() {
    let srv = TestServer::start().await;

    // Fire a check request first
    drop(srv.check("Hello world.").await);

    let body = srv.metrics().await.text().await.expect("read body");

    // Prometheus output should mention api_requests or similar counter
    assert!(
        body.contains("api"),
        "Metrics should contain api-related counters after a request"
    );
}
