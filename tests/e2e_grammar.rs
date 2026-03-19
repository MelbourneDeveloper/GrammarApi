//! E2E tests for grammar detection over HTTP.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::{by_category, get_matches, has_replacement, TestServer};

#[tokio::test]
async fn detects_an_before_consonant() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty(), "Should detect 'an' before consonant");
    assert!(has_replacement(grammar[0], "a"));
}

#[tokio::test]
async fn detects_a_before_vowel() {
    let srv = TestServer::start().await;
    let json = srv.check_json("I saw a elephant.").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty(), "Should detect 'a' before vowel");
    assert!(has_replacement(grammar[0], "an"));
}

#[tokio::test]
async fn correct_articles_pass() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("This is a test and an example.")
        .await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(grammar.is_empty(), "Correct articles should pass");
}

#[tokio::test]
async fn detects_repeated_words() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The the cat sat on the mat.")
        .await;
    let matches = get_matches(&json);

    assert!(
        !matches.is_empty(),
        "Should detect 'the the' repeated word"
    );
}

#[tokio::test]
async fn correct_grammar_returns_no_grammar_errors() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("The quick brown fox jumps over the lazy dog.")
        .await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(grammar.is_empty(), "Correct text has no grammar errors");
}

#[tokio::test]
async fn questions_do_not_trigger_false_positives() {
    let srv = TestServer::start().await;
    let json = srv.check_json("What is an apple?").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(grammar.is_empty(), "Question should have no grammar error");
}

#[tokio::test]
async fn exclamations_do_not_trigger_false_positives() {
    let srv = TestServer::start().await;
    let json = srv.check_json("What a beautiful day!").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(
        grammar.is_empty(),
        "Exclamation should have no grammar error"
    );
}

#[tokio::test]
async fn grammar_error_offset_points_at_error() {
    let srv = TestServer::start().await;
    let input = "This is an test.";
    let json = srv.check_json(input).await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty());

    let offset = grammar[0]["offset"].as_u64().expect("offset");
    // "an" starts at index 8 in "This is an test."
    assert_eq!(offset, 8, "Offset should point at 'an'");
}

#[tokio::test]
async fn grammar_error_length_matches_error_text() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty());

    let length = grammar[0]["length"].as_u64().expect("length");
    // "an" is 2 characters
    assert_eq!(length, 2, "Length should be 2 for 'an'");
}

#[tokio::test]
async fn grammar_error_has_non_empty_message() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty());

    let msg = grammar[0]["message"].as_str().expect("message");
    assert!(!msg.is_empty(), "Grammar error message must not be empty");
}

#[tokio::test]
async fn grammar_error_has_rule_id() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty());

    let rule_id = grammar[0]["rule"]["id"].as_str().expect("rule.id");
    assert!(!rule_id.is_empty(), "Grammar rule.id must not be empty");
}

#[tokio::test]
async fn grammar_error_category_is_grammar() {
    let srv = TestServer::start().await;
    let json = srv.check_json("This is an test.").await;
    let grammar = by_category(get_matches(&json), "grammar");

    assert!(!grammar.is_empty());
    assert_eq!(
        grammar[0]["rule"]["category"].as_str(),
        Some("grammar")
    );
}

// --- Mixed grammar + spelling ---

#[tokio::test]
async fn detects_both_grammar_and_spelling() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("This is an test with speling errors.")
        .await;
    let matches = get_matches(&json);
    let grammar = by_category(matches, "grammar");
    let spelling = by_category(matches, "spelling");

    assert!(
        !grammar.is_empty(),
        "Should detect grammar error ('an test')"
    );
    assert!(
        !spelling.is_empty(),
        "Should detect spelling error ('speling')"
    );
}

#[tokio::test]
async fn mixed_errors_each_have_correct_category() {
    let srv = TestServer::start().await;
    let json = srv
        .check_json("I saw a elephant with a beautifull coat.")
        .await;
    let matches = get_matches(&json);
    let grammar = by_category(matches, "grammar");
    let spelling = by_category(matches, "spelling");

    assert!(
        !grammar.is_empty(),
        "Should detect 'a elephant' grammar error"
    );
    assert!(
        !spelling.is_empty(),
        "Should detect 'beautifull' spelling error"
    );
}
