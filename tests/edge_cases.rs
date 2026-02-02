//! Edge case tests for the grammar API.

#![allow(clippy::panic, clippy::manual_let_else)]

mod common;

use common::{get_matches, post_check};

#[tokio::test]
async fn handles_empty_text() {
    let result = match post_check("").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(matches.is_empty(), "Empty text should have no errors");
}

#[tokio::test]
async fn handles_whitespace_only() {
    let result = match post_check("   \t\n  ").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    // Just verify API handles whitespace without crashing
    assert!(
        result["matches"].is_array(),
        "Should return valid response for whitespace"
    );
}

#[tokio::test]
async fn handles_single_word() {
    let result = match post_check("Hello").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(
        matches.is_empty(),
        "Single correct word should have no errors"
    );
}

#[tokio::test]
async fn handles_single_misspelled_word() {
    let result = match post_check("Helo").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(
        !matches.is_empty(),
        "Single misspelled word should be detected"
    );
}

#[tokio::test]
async fn handles_long_text() {
    let long_text = "This is a sentence. ".repeat(100);
    let result = match post_check(&long_text).await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let time = match result["metrics"]["processingTimeMs"].as_u64() {
        Some(t) => t,
        None => panic!("Missing processingTimeMs"),
    };

    assert!(
        time < 2000,
        "Long text should process in under 2 seconds, took {}ms",
        time
    );
}

#[tokio::test]
async fn handles_very_long_word() {
    let long_word = "a".repeat(100);
    let text = format!("This is a {} word.", long_word);
    let result = match post_check(&text).await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should handle very long words"
    );
}

#[tokio::test]
async fn handles_numbers() {
    let result = match post_check("I have 123 apples and 456 oranges.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(matches.is_empty(), "Numbers should not trigger errors");
}

#[tokio::test]
async fn handles_special_characters() {
    let result = match post_check("Email me at test@example.com!").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should handle special characters"
    );
}

#[tokio::test]
async fn handles_urls() {
    let result = match post_check("Visit https://example.com for more info.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle URLs");
}

#[tokio::test]
async fn handles_unicode() {
    let result = match post_check("The café serves naïve customers.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should handle unicode characters"
    );
}

#[tokio::test]
async fn handles_emoji() {
    let result = match post_check("I love this! 😀").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle emoji");
}

#[tokio::test]
async fn handles_multiple_sentences() {
    let result = match post_check("First sentence. Second sentence. Third sentence.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(
        matches.is_empty(),
        "Correct multiple sentences should have no errors"
    );
}

#[tokio::test]
async fn handles_newlines() {
    let result = match post_check("First line.\nSecond line.\nThird line.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle newlines");
}

#[tokio::test]
async fn handles_tabs() {
    let result = match post_check("Column1\tColumn2\tColumn3").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle tabs");
}

#[tokio::test]
async fn handles_mixed_case() {
    let result = match post_check("ThIs Is MiXeD cAsE.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle mixed case");
}

#[tokio::test]
async fn handles_all_caps() {
    let result = match post_check("THIS IS ALL CAPS.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle all caps");
}

#[tokio::test]
async fn handles_all_lowercase() {
    let result = match post_check("this is all lowercase.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle all lowercase");
}

#[tokio::test]
async fn handles_punctuation_only() {
    let result = match post_check("...!!!???").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should handle punctuation-only text"
    );
}

#[tokio::test]
async fn handles_quoted_text() {
    let result = match post_check("She said \"Hello, world!\"").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle quoted text");
}

#[tokio::test]
async fn handles_parentheses() {
    let result = match post_check("This (with parentheses) is fine.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(
        matches.is_empty(),
        "Text with parentheses should not trigger errors"
    );
}

#[tokio::test]
async fn handles_brackets() {
    let result = match post_check("Array elements [1, 2, 3] are listed.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle brackets");
}

#[tokio::test]
async fn handles_currency_symbols() {
    let result = match post_check("The price is $100 or €85.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(
        result["matches"].is_array(),
        "Should handle currency symbols"
    );
}

#[tokio::test]
async fn handles_percentages() {
    let result = match post_check("The rate increased by 50%.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(
        matches.is_empty(),
        "Text with percentages should not trigger false errors"
    );
}

#[tokio::test]
async fn handles_abbreviations() {
    let result = match post_check("Dr. Smith works at NASA.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(
        matches.is_empty(),
        "Common abbreviations should not trigger errors"
    );
}

#[tokio::test]
async fn handles_possessives() {
    let result = match post_check("John's book is on Mary's desk.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(matches.is_empty(), "Possessives should not trigger errors");
}

#[tokio::test]
async fn handles_ordinals() {
    let result = match post_check("This is the 1st, 2nd, and 3rd time.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    assert!(result["matches"].is_array(), "Should handle ordinals");
}

#[tokio::test]
async fn handles_dates() {
    let result = match post_check("The meeting is on January 15, 2024.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(matches.is_empty(), "Dates should not trigger errors");
}

#[tokio::test]
async fn handles_times() {
    let result = match post_check("The event starts at 3:30 PM.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(matches.is_empty(), "Times should not trigger errors");
}

#[tokio::test]
async fn offset_calculation_with_unicode() {
    let result = match post_check("Café has an speling error.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    assert!(!matches.is_empty(), "Should detect error after unicode");
}
