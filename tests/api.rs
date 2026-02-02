mod common;

use axum::http::StatusCode;
use common::{get_health, get_matches, post_check, post_check_with_options};

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let (status, body) = match get_health().await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert_eq!(status, StatusCode::OK, "Health should return 200");
    assert_eq!(body, "ok", "Health body should be 'ok'");
}

#[tokio::test]
async fn check_endpoint_returns_matches_array() {
    let result = match post_check("Test text.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Response should have matches array"
    );
}

#[tokio::test]
async fn check_endpoint_returns_metrics() {
    let result = match post_check("Test text.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["metrics"].is_object(),
        "Response should have metrics object"
    );
}

#[tokio::test]
async fn metrics_contains_processing_time() {
    let result = match post_check("Test text.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["metrics"]["processingTimeMs"].is_number(),
        "Metrics should have processingTimeMs"
    );
}

#[tokio::test]
async fn processing_time_is_reasonable() {
    let result = match post_check("This is a simple test sentence.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let time = match result["metrics"]["processingTimeMs"].as_u64() {
        Some(t) => t,
        None => panic!("Missing processingTimeMs"),
    };

    assert!(
        time < 1000,
        "Processing time should be under 1 second, was {}ms",
        time
    );
}

#[tokio::test]
async fn error_has_context_text() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(
        error["context"]["text"].is_string(),
        "Error should have context text"
    );
}

#[tokio::test]
async fn error_has_context_offset() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(
        error["context"]["offset"].is_number(),
        "Error should have context offset"
    );
}

#[tokio::test]
async fn error_has_context_length() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(
        error["context"]["length"].is_number(),
        "Error should have context length"
    );
}

#[tokio::test]
async fn error_has_rule_object() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(error["rule"].is_object(), "Error should have rule object");
}

#[tokio::test]
async fn rule_has_id() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(error["rule"]["id"].is_string(), "Rule should have id");
}

#[tokio::test]
async fn rule_has_category() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(
        error["rule"]["category"].is_string(),
        "Rule should have category"
    );
}

#[tokio::test]
async fn category_is_valid_value() {
    let result = match post_check("This is an test with speling erors.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    for error in matches {
        let category = match error["rule"]["category"].as_str() {
            Some(c) => c,
            None => panic!("Missing category"),
        };

        assert!(
            category == "spelling" || category == "grammar",
            "Category should be 'spelling' or 'grammar', got '{}'",
            category
        );
    }
}

#[tokio::test]
async fn error_has_replacements_array() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(
        error["replacements"].is_array(),
        "Error should have replacements array"
    );
}

#[tokio::test]
async fn accepts_language_parameter() {
    let result = match post_check_with_options("Test text.", "en-US", true, true).await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should accept language parameter"
    );
}

#[tokio::test]
async fn accepts_options_parameter() {
    let result = match post_check_with_options("Test text.", "en-US", true, true).await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should accept options parameter"
    );
}

#[tokio::test]
async fn error_has_offset() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(error["offset"].is_number(), "Error should have offset");
}

#[tokio::test]
async fn error_has_length() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(error["length"].is_number(), "Error should have length");
}

#[tokio::test]
async fn error_has_message() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should have at least one match");

    let error = &matches[0];
    assert!(error["message"].is_string(), "Error should have message");
}
