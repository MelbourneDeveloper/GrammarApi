//! Shared E2E test harness that starts a real HTTP server.

#![expect(dead_code, clippy::expect_used)]

use grammar_api::create_app_for_testing;
use reqwest::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// A running test server with its address and a client.
pub struct TestServer {
    pub addr: SocketAddr,
    pub client: Client,
}

impl TestServer {
    /// Spins up the grammar API on a random OS-assigned port.
    pub async fn start() -> Self {
        let app = create_app_for_testing();
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind to random port");
        let addr = listener.local_addr().expect("get local addr");

        tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("server should run");
        });

        let client = Client::builder()
            .no_proxy()
            .build()
            .expect("build reqwest client");

        Self { addr, client }
    }

    /// Base URL for this server instance.
    pub fn url(&self, path: &str) -> String {
        format!("http://{}{path}", self.addr)
    }

    /// POST /v1/check with a text payload.
    pub async fn check(&self, text: &str) -> reqwest::Response {
        self.client
            .post(self.url("/v1/check"))
            .json(&serde_json::json!({ "text": text }))
            .send()
            .await
            .expect("send check request")
    }

    /// POST /v1/check and parse JSON body.
    pub async fn check_json(&self, text: &str) -> serde_json::Value {
        self.check(text)
            .await
            .json()
            .await
            .expect("parse check response")
    }

    /// GET /health
    pub async fn health(&self) -> reqwest::Response {
        self.client
            .get(self.url("/health"))
            .send()
            .await
            .expect("send health request")
    }

    /// GET /metrics
    pub async fn metrics(&self) -> reqwest::Response {
        self.client
            .get(self.url("/metrics"))
            .send()
            .await
            .expect("send metrics request")
    }
}

/// Extract the matches array from a check response.
pub fn get_matches(v: &serde_json::Value) -> &Vec<serde_json::Value> {
    v["matches"]
        .as_array()
        .expect("response should have matches array")
}

/// Filter matches by category.
pub fn by_category<'a>(
    matches: &'a [serde_json::Value],
    category: &str,
) -> Vec<&'a serde_json::Value> {
    matches
        .iter()
        .filter(|m| m["rule"]["category"].as_str() == Some(category))
        .collect()
}

/// Check whether any replacement matches the expected string.
pub fn has_replacement(m: &serde_json::Value, expected: &str) -> bool {
    m["replacements"]
        .as_array()
        .is_some_and(|r| r.iter().any(|v| v.as_str() == Some(expected)))
}
