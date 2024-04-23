use axum::{routing::get, Router};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::shutdown::shutdown_signal;

use crate::prove;

pub async fn start() -> Result<(), std::io::Error> {
    // Enable tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "example_graceful_shutdown=debug,tower_http=debug,axum=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    // Create a regular axum app.
    let app = Router::new()
        .nest("/prove", prove::router())
        .route("/slow", get(|| sleep(Duration::from_secs(5))))
        .route("/forever", get(std::future::pending::<()>))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(60)),
        ));

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
