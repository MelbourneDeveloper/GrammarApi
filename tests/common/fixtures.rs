//! Shared test fixtures loader.

#![allow(clippy::expect_used)]

use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct TestFixtures {
    pub version: String,
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub description: String,
    pub input: String,
    #[serde(rename = "expectedErrors")]
    pub expected_errors: Vec<ExpectedError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedError {
    pub category: String,
    pub offset: Option<u64>,
    pub length: Option<u64>,
    #[serde(rename = "errorText")]
    pub error_text: Option<String>,
    pub replacements: Option<Vec<String>>,
    #[serde(rename = "minCount")]
    pub min_count: Option<usize>,
}

impl TestFixtures {
    pub fn load() -> Self {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_fixtures.json");
        let content = fs::read_to_string(path).expect("Failed to read test_fixtures.json");
        serde_json::from_str(&content).expect("Failed to parse test_fixtures.json")
    }

    pub fn case(id: &str) -> TestCase {
        let fixtures = Self::load();
        fixtures
            .cases
            .into_iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("Missing fixture: {}", id))
    }

    pub fn get_case(&self, id: &str) -> Option<&TestCase> {
        self.cases.iter().find(|c| c.id == id)
    }

    pub fn grammar_cases(&self) -> Vec<&TestCase> {
        self.cases
            .iter()
            .filter(|c| c.id.starts_with("grammar_"))
            .collect()
    }

    pub fn spelling_cases(&self) -> Vec<&TestCase> {
        self.cases
            .iter()
            .filter(|c| c.id.starts_with("spelling_"))
            .collect()
    }

    pub fn correct_cases(&self) -> Vec<&TestCase> {
        self.cases
            .iter()
            .filter(|c| c.id.starts_with("correct_"))
            .collect()
    }

    pub fn error_cases(&self) -> Vec<&TestCase> {
        self.cases
            .iter()
            .filter(|c| !c.expected_errors.is_empty())
            .collect()
    }
}

impl TestCase {
    pub fn expects_grammar_error(&self) -> bool {
        self.expected_errors.iter().any(|e| e.category == "grammar")
    }

    pub fn expects_spelling_error(&self) -> bool {
        self.expected_errors.iter().any(|e| e.category == "spelling")
    }

    pub fn expected_error_count(&self) -> usize {
        self.expected_errors
            .iter()
            .map(|e| e.min_count.unwrap_or(1))
            .sum()
    }
}
