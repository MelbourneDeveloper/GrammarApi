//! Spelling tests for the grammar API.

#![allow(clippy::panic, clippy::manual_let_else)]

mod common;

use common::{find_spelling_errors, get_matches, has_replacement, post_check};

#[tokio::test]
async fn detects_simple_misspelling() {
    let result = match post_check("This has a speling mistake.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(!spelling_errors.is_empty(), "Should detect spelling error");

    let error = spelling_errors[0];
    assert!(
        has_replacement(error, "spelling"),
        "Should suggest 'spelling' as replacement"
    );
}

#[tokio::test]
async fn detects_multiple_misspellings() {
    let result = match post_check("The quik brwon fox jumps ovar the lazzy dog.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        spelling_errors.len() >= 3,
        "Should detect at least 3 spelling errors, found {}",
        spelling_errors.len()
    );
}

#[tokio::test]
async fn suggests_replacement_for_common_typo() {
    let result = match post_check("I recieved your message.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        !spelling_errors.is_empty(),
        "Should detect 'recieved' error"
    );

    let error = spelling_errors[0];
    match error["replacements"].as_array() {
        Some(replacements) => {
            assert!(
                !replacements.is_empty(),
                "Should provide at least one suggestion for 'recieved'"
            );
        }
        None => panic!("Missing replacements array"),
    }
}

#[tokio::test]
async fn detects_transposed_letters() {
    let result = match post_check("Teh cat sat on teh mat.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        spelling_errors.len() >= 2,
        "Should detect transposed letters in 'teh'"
    );
}

#[tokio::test]
async fn detects_missing_letters() {
    let result = match post_check("The governmnt made an announcment.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(spelling_errors.len() >= 2, "Should detect missing letters");
}

#[tokio::test]
async fn detects_extra_letters() {
    let result = match post_check("This is definately wrong.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(!spelling_errors.is_empty(), "Should detect 'definately'");

    let error = spelling_errors[0];
    assert!(
        has_replacement(error, "definitely"),
        "Should suggest 'definitely'"
    );
}

#[tokio::test]
async fn correct_spelling_returns_no_errors() {
    let result = match post_check("The quick brown fox jumps over the lazy dog.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        spelling_errors.is_empty(),
        "Correct text should have no spelling errors"
    );
}

#[tokio::test]
async fn handles_proper_nouns() {
    let result = match post_check("John went to London with Mary.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        spelling_errors.is_empty(),
        "Should not flag proper nouns as spelling errors"
    );
}

#[tokio::test]
async fn handles_contractions() {
    let result = match post_check("I can't believe it's not butter.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        spelling_errors.is_empty(),
        "Should not flag contractions as spelling errors"
    );
}

#[tokio::test]
async fn handles_hyphenated_words() {
    let result = match post_check("This is a well-known fact.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(
        spelling_errors.is_empty(),
        "Should not flag hyphenated words as spelling errors"
    );
}

#[tokio::test]
async fn provides_multiple_suggestions() {
    let result = match post_check("I ned help.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(!spelling_errors.is_empty(), "Should detect 'ned'");

    let error = spelling_errors[0];
    if let Some(replacements) = error["replacements"].as_array() {
        assert!(
            !replacements.is_empty(),
            "Should provide at least one suggestion"
        );
    } else {
        panic!("Missing replacements array");
    }
}

#[tokio::test]
async fn detects_doubled_letters_error() {
    let result = match post_check("I have a beautifull garden.").await {
        Ok(r) => r,
        Err(e) => panic!("Request failed: {}", e),
    };

    let matches = match get_matches(&result) {
        Some(m) => m,
        None => panic!("Response missing matches array"),
    };

    let spelling_errors = find_spelling_errors(matches);
    assert!(!spelling_errors.is_empty(), "Should detect 'beautifull'");

    let error = spelling_errors[0];
    assert!(
        has_replacement(error, "beautiful"),
        "Should suggest 'beautiful'"
    );
}
