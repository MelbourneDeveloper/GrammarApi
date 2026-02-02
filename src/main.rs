use grammar_api::create_app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Loading dictionary...");
    let app = create_app();
    tracing::info!("Server ready");

    let addr = "0.0.0.0:8080";
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
