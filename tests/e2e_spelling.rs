//! E2E tests for spelling detection over HTTP.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::{by_category, get_matches, has_replacement, TestServer};

#[tokio::test]
async fn detects_simple_misspelling() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This has a speling mistake.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(!spelling.is_empty(), "Should detect 'speling'");
    assert!(has_replacement(spelling[0], "spelling"));
}

#[tokio::test]
async fn detects_multiple_misspellings() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The quik brwon fox jumps ovar the lazzy dog.")
        .await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(
        spelling.len() >= 3,
        "Should find at least 3 misspellings, found {}",
        spelling.len()
    );
}

#[tokio::test]
async fn suggests_replacement_for_recieved() {
    let srv = TestServer::start().await;
    let json = srv.check_json("I recieved your message.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(!spelling.is_empty(), "Should detect 'recieved'");

    let replacements = spelling[0]["replacements"]
        .as_array()
        .expect("replacements");
    assert!(
        !replacements.is_empty(),
        "Should suggest at least one replacement"
    );
}

#[tokio::test]
async fn detects_transposed_letters() {
    let srv = TestServer::start().await;
    let json = srv.check_json("Teh cat sat on teh mat.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(
        spelling.len() >= 2,
        "Should detect at least 2 transposed-letter errors"
    );
}

#[tokio::test]
async fn detects_missing_letters() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The governmnt made an announcment.")
        .await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(
        spelling.len() >= 2,
        "Should detect at least 2 missing-letter errors"
    );
}

#[tokio::test]
async fn detects_extra_letters() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is definately wrong.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(!spelling.is_empty(), "Should detect 'definately'");
    assert!(has_replacement(spelling[0], "definitely"));
}

#[tokio::test]
async fn detects_doubled_letters() {
    let srv = TestServer::start().await;
    let json = srv.check_json("I have a beautifull garden.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(!spelling.is_empty(), "Should detect 'beautifull'");
    assert!(has_replacement(spelling[0], "beautiful"));
}

#[tokio::test]
async fn correct_spelling_returns_no_spelling_errors() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The quick brown fox jumps over the lazy dog.")
        .await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(spelling.is_empty(), "Correct text has no spelling errors");
}

#[tokio::test]
async fn proper_nouns_not_flagged() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("John went to London with Mary.")
        .await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(spelling.is_empty(), "Proper nouns should not be flagged");
}

#[tokio::test]
async fn contractions_not_flagged() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("I can't believe it's not butter.")
        .await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(spelling.is_empty(), "Contractions should not be flagged");
}

#[tokio::test]
async fn hyphenated_words_not_flagged() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is a well-known fact.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(
        spelling.is_empty(),
        "Hyphenated words should not be flagged"
    );
}

#[tokio::test]
async fn spelling_error_has_correct_offset() {
    let srv = TestServer::start().await;
    let input = "This has a speling mistake.";
    let json = srv.check_json(input).await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(!spelling.is_empty());

    let offset = usize::try_from(spelling[0]["offset"].as_u64().expect("offset")).expect("offset fits usize");
    let length = usize::try_from(spelling[0]["length"].as_u64().expect("length")).expect("length fits usize");

    let chars: Vec<char> = input.chars().collect();
    let flagged: String = chars[offset..offset + length].iter().collect();

    assert_eq!(flagged, "speling", "Should point at 'speling'");
}

#[tokio::test]
async fn spelling_error_category_is_spelling() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This has a speling mistake.").await;
    let spelling = by_category(get_matches(&json), "spelling");

    assert!(!spelling.is_empty());
    assert_eq!(
        spelling[0]["rule"]["category"].as_str(),
        Some("spelling")
    );
}
