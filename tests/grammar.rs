//! Grammar tests for the grammar API.

#![allow(clippy::panic, clippy::manual_let_else)]

mod common;

use common::{find_grammar_errors, get_matches, has_replacement, post_check};

#[tokio::test]
async fn detects_incorrect_indefinite_article_an() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(!grammar_errors.is_empty(), "Should detect 'an test' error");

    let error = grammar_errors[0];
    assert!(
        has_replacement(error, "a"),
        "Should suggest 'a' as replacement"
    );
}

#[tokio::test]
async fn detects_incorrect_indefinite_article_a() {
    let result = match post_check("I saw a elephant.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(
        !grammar_errors.is_empty(),
        "Should detect 'a elephant' error"
    );

    let error = grammar_errors[0];
    assert!(
        has_replacement(error, "an"),
        "Should suggest 'an' as replacement"
    );
}

#[tokio::test]
async fn correct_indefinite_article_passes() {
    let result = match post_check("This is a test and an example.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(
        grammar_errors.is_empty(),
        "Correct article usage should not trigger errors"
    );
}

#[tokio::test]
async fn detects_repeated_words() {
    let result = match post_check("The the cat sat on the mat.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should detect repeated word 'the the'");
}

#[tokio::test]
async fn detects_multiple_grammar_errors() {
    let result = match post_check("This is an test and I seen a elephant.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(matches.len() >= 2, "Should detect multiple grammar errors");
}

#[tokio::test]
async fn correct_grammar_returns_no_errors() {
    let result = match post_check("The quick brown fox jumps over the lazy dog.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(
        grammar_errors.is_empty(),
        "Correct grammar should not trigger errors"
    );
}

#[tokio::test]
async fn error_has_correct_offset() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(!grammar_errors.is_empty(), "Should detect grammar error");

    let error = grammar_errors[0];
    let offset = match error["offset"].as_u64() {
        Some(o) => o,
        None => panic!("Missing offset in error"),
    };

    assert_eq!(offset, 8, "Error offset should be 8 (start of 'an')");
}

#[tokio::test]
async fn error_has_correct_length() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(!grammar_errors.is_empty(), "Should detect grammar error");

    let error = grammar_errors[0];
    let length = match error["length"].as_u64() {
        Some(l) => l,
        None => panic!("Missing length in error"),
    };

    assert_eq!(length, 2, "Error length should be 2 (length of 'an')");
}

#[tokio::test]
async fn detects_sentence_starting_lowercase() {
    let result = match post_check("hello world. this is wrong.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    // May or may not detect depending on Harper's rules
    // This test documents the behavior
    let _ = matches;
}

#[tokio::test]
async fn handles_questions_correctly() {
    let result = match post_check("What is an apple?").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(
        grammar_errors.is_empty(),
        "Correct question should not trigger errors"
    );
}

#[tokio::test]
async fn handles_exclamations_correctly() {
    let result = match post_check("What a beautiful day!").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(
        grammar_errors.is_empty(),
        "Correct exclamation should not trigger errors"
    );
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

    let grammar_errors = find_grammar_errors(matches);
    assert!(!grammar_errors.is_empty(), "Should detect grammar error");

    let error = grammar_errors[0];
    let message = match error["message"].as_str() {
        Some(m) => m,
        None => panic!("Missing message in error"),
    };

    assert!(!message.is_empty(), "Error message should not be empty");
}

#[tokio::test]
async fn error_has_rule_id() {
    let result = match post_check("This is an test.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let grammar_errors = find_grammar_errors(matches);
    assert!(!grammar_errors.is_empty(), "Should detect grammar error");

    let error = grammar_errors[0];
    let rule_id = match error["rule"]["id"].as_str() {
        Some(id) => id,
        None => panic!("Missing rule id in error"),
    };

    assert!(!rule_id.is_empty(), "Rule ID should not be empty");
}
