use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use harper_core::{
    linting::{LintGroup, Linter},
    parsers::PlainEnglish,
    spell::FstDictionary,
    Dialect, Document, Span,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Instant};
use tower_http::cors::{Any, CorsLayer};

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
) -> Result<Json<CheckResponse>, StatusCode> {
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

pub fn create_app() -> Router {
    let dictionary = FstDictionary::curated();
    let state = AppState { dictionary };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/v1/check", post(check_text))
        .route("/health", axum::routing::get(health))
        .layer(cors)
        .with_state(state)
}
