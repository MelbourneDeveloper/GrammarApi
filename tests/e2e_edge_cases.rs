//! E2E tests for edge cases and unusual inputs over HTTP.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::{get_matches, TestServer};
use reqwest::StatusCode;

// --- Empty / whitespace inputs ---

#[tokio::test]
async fn empty_text_returns_200() {
    let srv = TestServer::start().await;
    let resp = srv.check("").await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn empty_text_returns_no_matches() {
    let srv = TestServer::start().await;
    let json = srv.check_json("").await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Empty text should have no matches");
}

#[tokio::test]
async fn whitespace_only_returns_200() {
    let srv = TestServer::start().await;
    let resp = srv.check("   \t\n  ").await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn whitespace_only_returns_valid_json() {
    let srv = TestServer::start().await;
    let json = srv.check_json("   \t\n  ").await;

    assert!(json["matches"].is_array());
    assert!(json["metrics"].is_object());
}

// --- Single word ---

#[tokio::test]
async fn single_correct_word() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Hello").await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Single correct word = no errors");
}

#[tokio::test]
async fn single_misspelled_word() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Helo").await;
    let matches = get_matches(&json);

    assert!(
        !matches.is_empty(),
        "Single misspelled word should be detected"
    );
}

// --- Long text ---

#[tokio::test]
async fn long_text_processes_within_timeout() {
    let srv = TestServer::start().await;
    let long_text = "This is a sentence. ".repeat(100);
    let json = srv.check_json(&long_text).await;

    let ms = json["metrics"]["processingTimeMs"]
        .as_u64()
        .expect("processingTimeMs");
    assert!(ms < 3000, "Long text took {ms}ms, expected < 3000");
}

#[tokio::test]
async fn very_long_word_does_not_crash() {
    let srv = TestServer::start().await;
    let long_word = "a".repeat(200);
    let text = format!("This is a {long_word} word.");
    let json = srv.check_json(&text).await;

    assert!(json["matches"].is_array());
}

// --- Special character inputs ---

#[tokio::test]
async fn numbers_only() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("I have 123 apples and 456 oranges.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Numbers should not trigger errors");
}

#[tokio::test]
async fn email_addresses() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("Email me at test@example.com today.")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn urls_in_text() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("Visit https://example.com for more info.")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn unicode_accented_characters() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The café serves naïve customers.")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn emoji_in_text() {
    let srv = TestServer::start().await;
    let resp = srv.check("I love this! 😀🎉").await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn punctuation_only() {
    let srv = TestServer::start().await;
    let json = srv.check_json("...!!!???").await;

    assert!(json["matches"].is_array());
}

// --- Formatting variations ---

#[tokio::test]
async fn multiple_sentences() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("First sentence. Second sentence. Third sentence.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Correct sentences = no errors");
}

#[tokio::test]
async fn newlines_between_sentences() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("First line.\nSecond line.\nThird line.")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn tabs_in_text() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Column1\tColumn2\tColumn3").await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn all_caps() {
    let srv = TestServer::start().await;
    let json = srv.check_json("THIS IS ALL CAPS.").await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn all_lowercase() {
    let srv = TestServer::start().await;
    let json = srv.check_json("this is all lowercase.").await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn mixed_case() {
    let srv = TestServer::start().await;
    let json = srv.check_json("ThIs Is MiXeD cAsE.").await;

    assert!(json["matches"].is_array());
}

// --- Punctuation and formatting ---

#[tokio::test]
async fn quoted_text() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("She said \"Hello, world!\"")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn parentheses() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("This (with parentheses) is fine.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Parentheses should not cause errors");
}

#[tokio::test]
async fn brackets() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("Array elements [1, 2, 3] are listed.")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn currency_symbols() {
    let srv = TestServer::start().await;
    let json = srv.check_json("The price is $100 or €85.").await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn percentages() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The rate increased by 50%.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Percentages should not cause errors");
}

// --- Proper text types ---

#[tokio::test]
async fn abbreviations() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Dr. Smith works at NASA.").await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Abbreviations should not cause errors");
}

#[tokio::test]
async fn possessives() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("John's book is on Mary's desk.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Possessives should not cause errors");
}

#[tokio::test]
async fn ordinals() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("This is the 1st, 2nd, and 3rd time.")
        .await;

    assert!(json["matches"].is_array());
}

#[tokio::test]
async fn dates() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The meeting is on January 15, 2024.")
        .await;
    let matches = get_matches(&json);

    assert!(matches.is_empty(), "Dates should not trigger errors");
}

#[tokio::test]
async fn times_return_valid_response() {
    let srv = TestServer::start().await;
    let resp = srv.check("The event starts at 3:30 PM.").await;

    assert_eq!(resp.status(), StatusCode::OK);

    let json: serde_json::Value = resp.json().await.expect("parse");
    assert!(json["matches"].is_array());
}

// --- Unicode offset handling ---

#[tokio::test]
async fn unicode_offset_is_correct_after_accents() {
    let srv = TestServer::start().await;
    let input = "Café has an speling error.";
    let json = srv.check_json(input).await;
    let matches = get_matches(&json);

    assert!(!matches.is_empty(), "Should detect errors after unicode");

    // Every match should have valid offset within text length
    let char_count = input.chars().count();
    for m in matches {
        let offset = usize::try_from(m["offset"].as_u64().expect("offset")).expect("offset fits usize");
        let length = usize::try_from(m["length"].as_u64().expect("length")).expect("length fits usize");
        assert!(
            offset + length <= char_count,
            "offset {offset} + length {length} exceeds text char count {char_count}"
        );
    }
}

// --- Repeated calls return consistent results ---

#[tokio::test]
async fn repeated_calls_are_deterministic() {
    let srv = TestServer::start().await;

    let json1 = srv.check_json("This is an test.").await;
    let json2 = srv.check_json("This is an test.").await;

    let m1 = get_matches(&json1);
    let m2 = get_matches(&json2);

    assert_eq!(m1.len(), m2.len(), "Same input should give same count");

    for (a, b) in m1.iter().zip(m2.iter()) {
        assert_eq!(a["offset"], b["offset"]);
        assert_eq!(a["length"], b["length"]);
        assert_eq!(a["rule"]["id"], b["rule"]["id"]);
    }
}
