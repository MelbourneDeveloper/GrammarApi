//! Grammar tests for the grammar API using shared fixtures.

#![allow(clippy::panic, clippy::manual_let_else, clippy::expect_used)]

mod common;

use common::{find_grammar_errors, get_matches, has_replacement, post_check, TestFixtures};

#[tokio::test]
async fn detects_incorrect_indefinite_article_an() {
    let case = TestFixtures::case("grammar_article_an_before_consonant");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(!errors.is_empty(), "{}", case.description);

    let expected = &case.expected_errors[0];
    if let Some(replacements) = &expected.replacements {
        assert!(has_replacement(errors[0], &replacements[0]));
    }
}

#[tokio::test]
async fn detects_incorrect_indefinite_article_a() {
    let case = TestFixtures::case("grammar_article_a_before_vowel");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(!errors.is_empty(), "{}", case.description);

    let expected = &case.expected_errors[0];
    if let Some(replacements) = &expected.replacements {
        assert!(has_replacement(errors[0], &replacements[0]));
    }
}

#[tokio::test]
async fn correct_indefinite_article_passes() {
    let case = TestFixtures::case("correct_articles");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn detects_repeated_words() {
    let case = TestFixtures::case("grammar_repeated_word");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");

    let min_count = case.expected_errors[0].min_count.unwrap_or(1);
    assert!(matches.len() >= min_count, "{}", case.description);
}

#[tokio::test]
async fn correct_grammar_returns_no_errors() {
    let case = TestFixtures::case("correct_pangram");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn error_has_correct_offset() {
    let case = TestFixtures::case("grammar_article_an_before_consonant");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(!errors.is_empty());

    let expected = &case.expected_errors[0];
    if let Some(expected_offset) = expected.offset {
        let offset = errors[0]["offset"].as_u64().expect("Missing offset");
        assert_eq!(offset, expected_offset);
    }
}

#[tokio::test]
async fn error_has_correct_length() {
    let case = TestFixtures::case("grammar_article_an_before_consonant");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(!errors.is_empty());

    let expected = &case.expected_errors[0];
    if let Some(expected_length) = expected.length {
        let length = errors[0]["length"].as_u64().expect("Missing length");
        assert_eq!(length, expected_length);
    }
}

#[tokio::test]
async fn handles_questions_correctly() {
    let case = TestFixtures::case("correct_question");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn handles_exclamations_correctly() {
    let case = TestFixtures::case("correct_exclamation");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn error_has_message() {
    let case = TestFixtures::case("grammar_article_an_before_consonant");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(!errors.is_empty());

    let message = errors[0]["message"].as_str().expect("Missing message");
    assert!(!message.is_empty());
}

#[tokio::test]
async fn error_has_rule_id() {
    let case = TestFixtures::case("grammar_article_an_before_consonant");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_grammar_errors(matches);

    assert!(!errors.is_empty());

    let rule_id = errors[0]["rule"]["id"].as_str().expect("Missing rule id");
    assert!(!rule_id.is_empty());
}
