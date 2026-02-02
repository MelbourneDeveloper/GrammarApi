use serde_json::{json, Value};
use std::process::{Child, Command};
use std::time::Duration;

struct TestServer {
    child: Child,
}

impl TestServer {
    fn start() -> Self {
        let child = Command::new("cargo")
            .args(["run", "--release"])
            .spawn()
            .expect("Failed to start server");

        // Wait for server to be ready
        std::thread::sleep(Duration::from_secs(5));

        Self { child }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn post_check(text: &str) -> Value {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:8080/v1/check")
        .json(&json!({ "text": text }))
        .send()
        .expect("Failed to send request");

    response.json().expect("Failed to parse JSON")
}

#[test]
fn test_spelling_error() {
    let _server = TestServer::start();

    let result = post_check("This has a speling mistake.");
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty(), "Should detect spelling error");

    let spell_error = matches.iter().find(|m| {
        m["rule"]["category"] == "spelling"
    });
    assert!(spell_error.is_some(), "Should have spelling category");

    let error = spell_error.unwrap();
    assert!(
        error["replacements"].as_array().unwrap().iter()
            .any(|r| r.as_str() == Some("spelling")),
        "Should suggest 'spelling' as replacement"
    );
}

#[test]
fn test_grammar_error_indefinite_article() {
    let _server = TestServer::start();

    let result = post_check("This is an test.");
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty(), "Should detect grammar error");

    let grammar_error = matches.iter().find(|m| {
        m["rule"]["category"] == "grammar"
    });
    assert!(grammar_error.is_some(), "Should have grammar category");

    let error = grammar_error.unwrap();
    assert_eq!(error["offset"], 8, "Error should be at position 8");
    assert!(
        error["replacements"].as_array().unwrap().iter()
            .any(|r| r.as_str() == Some("a")),
        "Should suggest 'a' as replacement"
    );
}

#[test]
fn test_multiple_errors() {
    let _server = TestServer::start();

    let result = post_check("This is an test with erors and mispeled words.");
    let matches = result["matches"].as_array().unwrap();

    assert!(matches.len() >= 3, "Should detect multiple errors");
}

#[test]
fn test_clean_text() {
    let _server = TestServer::start();

    let result = post_check("This is a perfectly correct sentence.");
    let matches = result["matches"].as_array().unwrap();

    assert!(matches.is_empty(), "Should have no errors for correct text");
}

#[test]
fn test_response_has_metrics() {
    let _server = TestServer::start();

    let result = post_check("Test text.");

    assert!(result["metrics"]["processingTimeMs"].is_number(), "Should have processing time");
}

#[test]
fn test_response_has_context() {
    let _server = TestServer::start();

    let result = post_check("This is an test.");
    let matches = result["matches"].as_array().unwrap();

    assert!(!matches.is_empty());
    let first = &matches[0];

    assert!(first["context"]["text"].is_string(), "Should have context text");
    assert!(first["context"]["offset"].is_number(), "Should have context offset");
    assert!(first["context"]["length"].is_number(), "Should have context length");
}

#[test]
fn test_health_endpoint() {
    let _server = TestServer::start();

    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://localhost:8080/health")
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    assert_eq!(response.text().unwrap(), "ok");
}
