//! E2E tests for the /v1/check response structure and happy paths.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::{get_matches, TestServer};
use reqwest::StatusCode;

// --- Response structure ---

#[tokio::test]
async fn check_returns_200() {
    let srv = TestServer::start().await;
    let resp = srv.check("Hello world.").await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn check_response_is_json() {
    let srv = TestServer::start().await;
    let resp = srv.check("Hello world.").await;
    let ct = resp
        .headers()
        .get("content-type")
        .expect("content-type header")
        .to_str()
        .expect("header to str");

    assert!(
        ct.contains("application/json"),
        "Expected application/json, got {ct}"
    );
}

#[tokio::test]
async fn check_response_has_matches_array() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Hello world.").await;

    assert!(json["matches"].is_array(), "Response must have matches[]");
}

#[tokio::test]
async fn check_response_has_metrics_object() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Hello world.").await;

    assert!(
        json["metrics"].is_object(),
        "Response must have metrics object"
    );
}

#[tokio::test]
async fn check_metrics_has_processing_time() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Hello world.").await;

    assert!(
        json["metrics"]["processingTimeMs"].is_number(),
        "metrics.processingTimeMs must be a number"
    );
}

#[tokio::test]
async fn processing_time_is_non_negative() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Hello world.").await;
    let ms = json["metrics"]["processingTimeMs"]
        .as_u64()
        .expect("processingTimeMs");

    // Should be a reasonable value, not some huge garbage
    assert!(ms < 5000, "Processing time {ms} is too high");
}

// --- Match object structure ---

#[tokio::test]
async fn match_has_message() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty(), "Should detect an error");
    assert!(
        matches[0]["message"].is_string(),
        "match.message must be a string"
    );
}

#[tokio::test]
async fn match_message_is_not_empty() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    let msg = matches[0]["message"].as_str().expect("message");
    assert!(!msg.is_empty(), "match.message must not be empty");
}

#[tokio::test]
async fn match_has_offset() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["offset"].is_number(),
        "match.offset must be a number"
    );
}

#[tokio::test]
async fn match_has_length() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["length"].is_number(),
        "match.length must be a number"
    );
}

#[tokio::test]
async fn match_length_is_positive() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    let len = matches[0]["length"].as_u64().expect("length");
    assert!(len > 0, "match.length must be positive");
}

#[tokio::test]
async fn match_has_replacements_array() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["replacements"].is_array(),
        "match.replacements must be an array"
    );
}

#[tokio::test]
async fn match_has_rule_object() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["rule"].is_object(),
        "match.rule must be an object"
    );
}

#[tokio::test]
async fn match_rule_has_id() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["rule"]["id"].is_string(),
        "rule.id must be a string"
    );
}

#[tokio::test]
async fn match_rule_id_not_empty() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    let id = matches[0]["rule"]["id"].as_str().expect("rule.id");
    assert!(!id.is_empty(), "rule.id must not be empty");
}

#[tokio::test]
async fn match_rule_has_category() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["rule"]["category"].is_string(),
        "rule.category must be a string"
    );
}

#[tokio::test]
async fn match_rule_category_is_valid() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("This is an test with speling erors.")
        .await;
    let matches = get_matches(&json);

    for m in matches {
        let cat = m["rule"]["category"].as_str().expect("category");
        assert!(
            cat == "spelling" || cat == "grammar",
            "Invalid category: {cat}"
        );
    }
}

#[tokio::test]
async fn match_has_context_object() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["context"].is_object(),
        "match.context must be an object"
    );
}

#[tokio::test]
async fn match_context_has_text() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["context"]["text"].is_string(),
        "context.text must be a string"
    );
}

#[tokio::test]
async fn match_context_has_offset() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["context"]["offset"].is_number(),
        "context.offset must be a number"
    );
}

#[tokio::test]
async fn match_context_has_length() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());
    assert!(
        matches[0]["context"]["length"].is_number(),
        "context.length must be a number"
    );
}

// --- Correct text returns empty matches ---

#[tokio::test]
async fn correct_sentence_returns_no_matches() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is a correct sentence.").await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Correct text should have no matches");
}

#[tokio::test]
async fn correct_pangram_returns_no_matches() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The quick brown fox jumps over the lazy dog.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Pangram should have no matches");
}

// --- Offset + length point at the right text ---

#[tokio::test]
async fn offset_and_length_identify_error_text() {
    let srv = TestServer::start().await;
    let input = "This is an test.";
    let json = srv.check_json(input).await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty());

    let offset = usize::try_from(matches[0]["offset"].as_u64().expect("offset")).expect("offset fits usize");
    let length = usize::try_from(matches[0]["length"].as_u64().expect("length")).expect("length fits usize");

    let chars: Vec<char> = input.chars().collect();
    let error_text: String = chars[offset..offset + length].iter().collect();

    // "an" is the error in "This is an test."
    assert_eq!(error_text, "an", "Offset+length should point at 'an'");
}
