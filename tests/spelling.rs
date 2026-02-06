//! Spelling tests for the grammar API using shared fixtures.

#![allow(clippy::panic, clippy::manual_let_else, clippy::expect_used)]

mod common;

use common::{find_spelling_errors, get_matches, has_replacement, post_check, TestFixtures};

#[tokio::test]
async fn detects_simple_misspelling() {
    let case = TestFixtures::case("spelling_simple_misspelling");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(!errors.is_empty(), "{}", case.description);

    let expected = &case.expected_errors[0];
    if let Some(replacements) = &expected.replacements {
        assert!(has_replacement(errors[0], &replacements[0]));
    }
}

#[tokio::test]
async fn detects_multiple_misspellings() {
    let case = TestFixtures::case("spelling_multiple_errors");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    let min_count = case.expected_errors[0].min_count.unwrap_or(1);
    assert!(errors.len() >= min_count, "{}", case.description);
}

#[tokio::test]
async fn suggests_replacement_for_common_typo() {
    let case = TestFixtures::case("spelling_common_typo_receive");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(!errors.is_empty(), "{}", case.description);

    let replacements = errors[0]["replacements"]
        .as_array()
        .expect("Missing replacements");
    assert!(!replacements.is_empty());
}

#[tokio::test]
async fn detects_transposed_letters() {
    let case = TestFixtures::case("spelling_transposed_letters");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    let min_count = case.expected_errors[0].min_count.unwrap_or(1);
    assert!(errors.len() >= min_count, "{}", case.description);
}

#[tokio::test]
async fn detects_missing_letters() {
    let case = TestFixtures::case("spelling_missing_letters");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    let min_count = case.expected_errors[0].min_count.unwrap_or(1);
    assert!(errors.len() >= min_count, "{}", case.description);
}

#[tokio::test]
async fn detects_extra_letters() {
    let case = TestFixtures::case("spelling_extra_letters");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(!errors.is_empty(), "{}", case.description);

    let expected = &case.expected_errors[0];
    if let Some(replacements) = &expected.replacements {
        assert!(has_replacement(errors[0], &replacements[0]));
    }
}

#[tokio::test]
async fn correct_spelling_returns_no_errors() {
    let case = TestFixtures::case("correct_pangram");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn handles_proper_nouns() {
    let case = TestFixtures::case("correct_proper_nouns");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn handles_contractions() {
    let case = TestFixtures::case("correct_contractions");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn handles_hyphenated_words() {
    let case = TestFixtures::case("correct_hyphenated");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(errors.is_empty(), "{}", case.description);
}

#[tokio::test]
async fn detects_doubled_letters_error() {
    let case = TestFixtures::case("spelling_doubled_letters");

    let result = post_check(&case.input).await.expect("Request failed");
    let matches = get_matches(&result).expect("Missing matches");
    let errors = find_spelling_errors(matches);

    assert!(!errors.is_empty(), "{}", case.description);

    let expected = &case.expected_errors[0];
    if let Some(replacements) = &expected.replacements {
        assert!(has_replacement(errors[0], &replacements[0]));
    }
}
