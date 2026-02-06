#![allow(dead_code)]

pub mod fixtures;

#[allow(unused_imports)]
pub use fixtures::TestFixtures;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use grammar_api::create_app_for_testing;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn create_test_app() -> Router {
    create_app_for_testing()
}

pub async fn post_check(text: &str) -> Result<Value, String> {
    let app = create_test_app();

    let request = match Request::builder()
        .method("POST")
        .uri("/v1/check")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "text": text }).to_string()))
    {
        Ok(req) => req,
        Err(e) => return Err(format!("Failed to build request: {}", e)),
    };

    let response = match app.oneshot(request).await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Request failed: {}", e)),
    };

    let body = match response.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => return Err(format!("Failed to read body: {}", e)),
    };

    match serde_json::from_slice(&body) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Failed to parse JSON: {}", e)),
    }
}

pub async fn get_health() -> Result<(StatusCode, String), String> {
    let app = create_test_app();

    let request = match Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
    {
        Ok(req) => req,
        Err(e) => return Err(format!("Failed to build request: {}", e)),
    };

    let response = match app.oneshot(request).await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Request failed: {}", e)),
    };

    let status = response.status();

    let body = match response.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => return Err(format!("Failed to read body: {}", e)),
    };

    let text = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(e) => return Err(format!("Invalid UTF-8: {}", e)),
    };

    Ok((status, text))
}

pub fn get_matches(result: &Value) -> Option<&Vec<Value>> {
    result["matches"].as_array()
}

pub fn find_spelling_errors(matches: &[Value]) -> Vec<&Value> {
    matches
        .iter()
        .filter(|m| m["rule"]["category"] == "spelling")
        .collect()
}

pub fn find_grammar_errors(matches: &[Value]) -> Vec<&Value> {
    matches
        .iter()
        .filter(|m| m["rule"]["category"] == "grammar")
        .collect()
}

pub fn has_replacement(error: &Value, expected: &str) -> bool {
    if let Some(replacements) = error["replacements"].as_array() {
        replacements.iter().any(|r| r.as_str() == Some(expected))
    } else {
        false
    }
}
