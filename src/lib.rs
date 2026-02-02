//! Grammar and spelling checking API using Harper.
//!
//! This crate provides a REST API for checking text for grammar and spelling errors
//! using the Harper library.

use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use harper_core::{
    linting::{LintGroup, Linter},
    parsers::PlainEnglish,
    spell::FstDictionary,
    Dialect, Document, Span,
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, time::Instant};
use tower_http::cors::CorsLayer;

/// Maximum allowed text size in bytes (100KB)
const MAX_TEXT_SIZE: usize = 100 * 1024;

#[derive(Clone)]
struct AppState {
    dictionary: Arc<FstDictionary>,
}

#[derive(Debug, Deserialize)]
struct CheckRequest {
    text: String,
    #[serde(default = "default_language")]
    #[allow(dead_code)]
    language: String,
    #[serde(default)]
    #[allow(dead_code)]
    options: CheckOptions,
}

fn default_language() -> String {
    "en-US".to_string()
}

#[derive(Debug, Deserialize, Default)]
struct CheckOptions {
    #[serde(default = "default_true")]
    #[allow(dead_code)]
    spelling: bool,
    #[serde(default = "default_true")]
    #[allow(dead_code)]
    grammar: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize)]
struct CheckResponse {
    matches: Vec<Match>,
    metrics: Metrics,
}

#[derive(Debug, Serialize)]
struct Match {
    message: String,
    offset: usize,
    length: usize,
    replacements: Vec<String>,
    rule: Rule,
    context: Context,
}

#[derive(Debug, Serialize)]
struct Rule {
    id: String,
    category: String,
}

#[derive(Debug, Serialize)]
struct Context {
    text: String,
    offset: usize,
    length: usize,
}

#[derive(Debug, Serialize)]
struct Metrics {
    #[serde(rename = "processingTimeMs")]
    processing_time_ms: u128,
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
    code: String,
}

enum AppError {
    PayloadTooLarge,
    #[allow(dead_code)]
    InvalidRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error, code) = match self {
            Self::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!("Text exceeds maximum size of {} bytes", MAX_TEXT_SIZE),
                "PAYLOAD_TOO_LARGE".to_string(),
            ),
            Self::InvalidRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg, "INVALID_REQUEST".to_string())
            }
        };

        (status, Json(ApiError { error, code })).into_response()
    }
}

fn get_context(text: &str, span: Span) -> Context {
    let start = span.start.saturating_sub(20);
    let end = (span.end + 20).min(text.len());

    let context_text: String = text.chars().skip(start).take(end - start).collect();

    Context {
        text: context_text,
        offset: span.start - start,
        length: span.len(),
    }
}

async fn check_text(
    State(state): State<AppState>,
    Json(payload): Json<CheckRequest>,
) -> Result<Json<CheckResponse>, AppError> {
    // Validate input size
    if payload.text.len() > MAX_TEXT_SIZE {
        return Err(AppError::PayloadTooLarge);
    }

    let start = Instant::now();

    let parser = PlainEnglish;
    let document = Document::new_curated(&payload.text, &parser);

    let mut linter = LintGroup::new_curated(state.dictionary.clone(), Dialect::American);

    let lints = linter.lint(&document);

    let matches: Vec<Match> = lints
        .into_iter()
        .map(|lint| {
            let span = lint.span;
            let suggestions: Vec<String> = lint
                .suggestions
                .into_iter()
                .filter_map(|s| {
                    if let harper_core::linting::Suggestion::ReplaceWith(chars) = s {
                        Some(chars.into_iter().collect())
                    } else {
                        None
                    }
                })
                .collect();

            let category = if lint.lint_kind.to_string().to_lowercase().contains("spell") {
                "spelling"
            } else {
                "grammar"
            };

            Match {
                message: lint.message,
                offset: span.start,
                length: span.len(),
                replacements: suggestions,
                rule: Rule {
                    id: lint.lint_kind.to_string(),
                    category: category.to_string(),
                },
                context: get_context(&payload.text, span),
            }
        })
        .collect();

    let response = CheckResponse {
        matches,
        metrics: Metrics {
            processing_time_ms: start.elapsed().as_millis(),
        },
    };

    Ok(Json(response))
}

async fn health() -> &'static str {
    "ok"
}

fn build_cors_layer() -> CorsLayer {
    let allowed_origins = env::var("CORS_ORIGINS").unwrap_or_default();

    if allowed_origins.is_empty() || allowed_origins == "*" {
        // Development mode: allow all origins
        CorsLayer::permissive()
    } else {
        // Production mode: restrict to specific origins
        let origins: Vec<_> = allowed_origins
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([header::CONTENT_TYPE, header::ACCEPT, header::AUTHORIZATION])
    }
}

/// Creates the application router with all routes configured.
pub fn create_app() -> Router {
    let dictionary = FstDictionary::curated();
    let state = AppState { dictionary };

    let cors = build_cors_layer();

    Router::new()
        .route("/v1/check", post(check_text))
        .route("/health", get(health))
        .layer(cors)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_text_size_constant() {
        assert_eq!(MAX_TEXT_SIZE, 100 * 1024);
    }

    #[test]
    fn test_default_language() {
        assert_eq!(default_language(), "en-US");
    }
}
