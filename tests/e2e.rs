use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use grammar_api::create_app;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn post_check(text: &str) -> Value {
    let app = create_app();

    let request = Request::builder()
        .method("POST")
        .uri("/v1/check")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "text": text }).to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_spelling_error_detected() {
    let result = post_check("This has a speling mistake.").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty(), "Should detect spelling error");

    let spell_error = matches.iter().find(|m| m["rule"]["category"] == "spelling");
    assert!(spell_error.is_some(), "Should have spelling category");

    let error = spell_error.unwrap();
    assert!(
        error["replacements"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r.as_str() == Some("spelling")),
        "Should suggest 'spelling' as replacement"
    );
}

#[tokio::test]
async fn test_grammar_error_indefinite_article() {
    let result = post_check("This is an test.").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty(), "Should detect grammar error");

    let grammar_error = matches.iter().find(|m| m["rule"]["category"] == "grammar");
    assert!(grammar_error.is_some(), "Should have grammar category");

    let error = grammar_error.unwrap();
    assert_eq!(error["offset"], 8, "Error should be at position 8");
    assert!(
        error["replacements"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r.as_str() == Some("a")),
        "Should suggest 'a' as replacement"
    );
}

#[tokio::test]
async fn test_multiple_errors_detected() {
    let result = post_check("This is an test with erors and mispeled words.").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(matches.len() >= 3, "Should detect at least 3 errors");

    let spelling_errors: Vec<_> = matches
        .iter()
        .filter(|m| m["rule"]["category"] == "spelling")
        .collect();
    assert!(
        spelling_errors.len() >= 2,
        "Should have at least 2 spelling errors"
    );
}

#[tokio::test]
async fn test_clean_text_no_errors() {
    let result = post_check("This is a perfectly correct sentence.").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(matches.is_empty(), "Should have no errors for correct text");
}

#[tokio::test]
async fn test_response_contains_metrics() {
    let result = post_check("Test text.").await;

    assert!(
        result["metrics"]["processingTimeMs"].is_number(),
        "Should have processing time metric"
    );
}

#[tokio::test]
async fn test_response_contains_context() {
    let result = post_check("This is an test.").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty());
    let first = &matches[0];

    assert!(
        first["context"]["text"].is_string(),
        "Should have context text"
    );
    assert!(
        first["context"]["offset"].is_number(),
        "Should have context offset"
    );
    assert!(
        first["context"]["length"].is_number(),
        "Should have context length"
    );
}

#[tokio::test]
async fn test_error_offset_and_length() {
    let result = post_check("The quik brown fox.").await;
    let matches = result["matches"].as_array().unwrap();

    let error = matches
        .iter()
        .find(|m| m["rule"]["category"] == "spelling")
        .expect("Should find spelling error");

    assert_eq!(error["offset"], 4, "Error should start at 'quik'");
    assert_eq!(error["length"], 4, "Error length should be 4");
}

#[tokio::test]
async fn test_replacements_provided() {
    let result = post_check("I recieved your message.").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty(), "Should detect error");

    let error = &matches[0];
    let replacements = error["replacements"].as_array().unwrap();

    assert!(!replacements.is_empty(), "Should provide replacements");
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = create_app();

    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"ok");
}

#[tokio::test]
async fn test_empty_text() {
    let result = post_check("").await;
    let matches = result["matches"].as_array().unwrap();

    assert!(matches.is_empty(), "Empty text should have no errors");
}

#[tokio::test]
async fn test_long_text() {
    let long_text = "This is a sentence. ".repeat(100);
    let result = post_check(&long_text).await;

    assert!(
        result["metrics"]["processingTimeMs"].as_u64().unwrap() < 1000,
        "Should process long text in under 1 second"
    );
}
