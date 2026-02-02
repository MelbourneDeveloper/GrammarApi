//! Grammar and spelling checking API.
//!
//! This crate provides a production-ready HTTP API for grammar and spelling
//! checking using the Harper library.

use axum::{
    extract::{Request, State},
    http::{header, Method, StatusCode},
    middleware::{self, Next},
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
use metrics::{counter, histogram};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use std::{env, fmt, sync::Arc, time::Instant};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::info_span;

/// Maximum allowed text size in bytes (100KB).
pub const MAX_TEXT_SIZE: usize = 100 * 1024;

/// Default rate limit: requests per second per IP.
const DEFAULT_RATE_LIMIT_PER_SECOND: u64 = 10;

/// Default rate limit burst size.
const DEFAULT_RATE_LIMIT_BURST: u32 = 30;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    dictionary: Arc<FstDictionary>,
    api_key: Option<String>,
    metrics_handle: PrometheusHandle,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("dictionary", &"<FstDictionary>")
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field("metrics_handle", &"<PrometheusHandle>")
            .finish()
    }
}

/// Request payload for the check endpoint.
#[derive(Debug, Deserialize)]
pub struct CheckRequest {
    /// The text to check for grammar and spelling errors.
    text: String,
    /// Language code (default: en-US).
    #[serde(default = "default_language")]
    #[allow(dead_code)]
    language: String,
    /// Optional checking options.
    #[serde(default)]
    #[allow(dead_code)]
    options: CheckOptions,
}

fn default_language() -> String {
    "en-US".to_string()
}

/// Options for controlling what checks to perform.
#[derive(Debug, Deserialize, Default)]
pub struct CheckOptions {
    /// Enable spelling checks (default: true).
    #[serde(default = "default_true")]
    #[allow(dead_code)]
    spelling: bool,
    /// Enable grammar checks (default: true).
    #[serde(default = "default_true")]
    #[allow(dead_code)]
    grammar: bool,
}

fn default_true() -> bool {
    true
}

/// Response from the check endpoint.
#[derive(Debug, Serialize)]
pub struct CheckResponse {
    /// List of detected issues.
    matches: Vec<Match>,
    /// Processing metrics.
    metrics: Metrics,
}

/// A detected grammar or spelling issue.
#[derive(Debug, Serialize)]
pub struct Match {
    /// Human-readable description of the issue.
    message: String,
    /// Character offset where the issue starts.
    offset: usize,
    /// Length of the problematic text.
    length: usize,
    /// Suggested replacements.
    replacements: Vec<String>,
    /// Rule information.
    rule: Rule,
    /// Context around the issue.
    context: Context,
}

/// Information about the rule that detected an issue.
#[derive(Debug, Serialize)]
pub struct Rule {
    /// Unique identifier for the rule.
    id: String,
    /// Category of the rule (spelling or grammar).
    category: String,
}

/// Context surrounding a detected issue.
#[derive(Debug, Serialize)]
pub struct Context {
    /// Text snippet around the issue.
    text: String,
    /// Offset within the context where the issue starts.
    offset: usize,
    /// Length of the issue within the context.
    length: usize,
}

/// Processing metrics for the request.
#[derive(Debug, Serialize)]
pub struct Metrics {
    /// Time taken to process the request in milliseconds.
    #[serde(rename = "processingTimeMs")]
    processing_time_ms: u128,
}

/// Error response structure.
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Human-readable error message.
    error: String,
    /// Machine-readable error code.
    code: String,
}

/// Application errors.
#[derive(Debug)]
pub enum AppError {
    /// Request payload exceeds maximum size.
    PayloadTooLarge,
    /// Invalid or missing API key.
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error, code) = match self {
            Self::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!("Text exceeds maximum size of {MAX_TEXT_SIZE} bytes"),
                "PAYLOAD_TOO_LARGE".to_string(),
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Invalid or missing API key".to_string(),
                "UNAUTHORIZED".to_string(),
            ),
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
    let start = Instant::now();

    // Validate input size
    if payload.text.len() > MAX_TEXT_SIZE {
        counter!("api.errors", "type" => "payload_too_large").increment(1);
        return Err(AppError::PayloadTooLarge);
    }

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

    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_millis();

    // Record metrics
    counter!("api.requests", "endpoint" => "check").increment(1);
    histogram!("api.request_duration_ms", "endpoint" => "check").record(elapsed_ms as f64);
    counter!("api.matches_found").increment(matches.len() as u64);

    let response = CheckResponse {
        matches,
        metrics: Metrics {
            processing_time_ms: elapsed_ms,
        },
    };

    Ok(Json(response))
}

async fn health() -> &'static str {
    counter!("api.requests", "endpoint" => "health").increment(1);
    "ok"
}

async fn metrics_handler(State(state): State<AppState>) -> String {
    state.metrics_handle.render()
}

async fn auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Skip auth for health and metrics endpoints
    let path = request.uri().path();
    if path == "/health" || path == "/metrics" {
        return Ok(next.run(request).await);
    }

    // If no API key is configured, allow all requests
    let Some(expected_key) = &state.api_key else {
        return Ok(next.run(request).await);
    };

    // Check for API key in header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(header_val) if header_val.starts_with("Bearer ") => {
            let provided_key = &header_val[7..];
            if provided_key == expected_key {
                Ok(next.run(request).await)
            } else {
                counter!("api.errors", "type" => "unauthorized").increment(1);
                Err(AppError::Unauthorized)
            }
        }
        _ => {
            counter!("api.errors", "type" => "unauthorized").increment(1);
            Err(AppError::Unauthorized)
        }
    }
}

fn build_cors_layer() -> CorsLayer {
    let allowed_origins = env::var("CORS_ORIGINS").unwrap_or_default();

    if allowed_origins.is_empty() || allowed_origins == "*" {
        CorsLayer::permissive()
    } else {
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

fn get_rate_limit_config() -> (u64, u32) {
    let rps: u64 = env::var("RATE_LIMIT_PER_SECOND")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_RATE_LIMIT_PER_SECOND);

    let burst: u32 = env::var("RATE_LIMIT_BURST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_RATE_LIMIT_BURST);

    (rps, burst)
}

/// Global metrics handle for sharing across app instances.
static METRICS_HANDLE: std::sync::OnceLock<PrometheusHandle> = std::sync::OnceLock::new();

fn get_or_init_metrics() -> PrometheusHandle {
    METRICS_HANDLE
        .get_or_init(|| {
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .install_recorder()
                .unwrap_or_else(|_| {
                    // Recorder already installed, just build a new handle
                    metrics_exporter_prometheus::PrometheusBuilder::new()
                        .build_recorder()
                        .handle()
                })
        })
        .clone()
}

/// Creates the application router with all middleware configured.
pub fn create_app() -> Router {
    let metrics_handle = get_or_init_metrics();

    let dictionary = FstDictionary::curated();
    let api_key = env::var("API_KEY").ok().filter(|k| !k.is_empty());

    let state = AppState {
        dictionary,
        api_key,
        metrics_handle,
    };

    let cors = build_cors_layer();
    let (rps, burst) = get_rate_limit_config();
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(rps)
        .burst_size(burst)
        .finish()
        .unwrap_or_else(|| {
            GovernorConfigBuilder::default()
                .per_second(DEFAULT_RATE_LIMIT_PER_SECOND)
                .burst_size(DEFAULT_RATE_LIMIT_BURST)
                .finish()
                .unwrap_or_else(|| unreachable!())
        });
    let rate_limiter = GovernorLayer::new(governor_conf);

    let x_request_id = http::HeaderName::from_static("x-request-id");

    Router::new()
        .route("/v1/check", post(check_text))
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .layer(rate_limiter)
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let request_id = request
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");
            info_span!(
                "request",
                method = %request.method(),
                uri = %request.uri(),
                request_id = %request_id,
            )
        }))
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

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }
}
