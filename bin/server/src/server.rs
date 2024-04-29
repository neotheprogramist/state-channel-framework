use axum::{routing::get, Router};

use std::{
    net::{AddrParseError, SocketAddr},
    time::Duration,
};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::shutdown::shutdown_signal;
use serde_json::json;
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{request, Args};

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("server error")]
    Server(#[from] std::io::Error),

    #[error("failed to parse address")]
    AddressParse(#[from] AddrParseError),

    #[error("Request to Binance API failed")]
    BTCRequestFailure(String), // Implement From trait for reqwest::Error
    #[error("Failed to parse JSON response")]
    JsonParsingFailed(#[from] serde_json::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            ServerError::Server(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ServerError::AddressParse(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ServerError::BTCRequestFailure(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ServerError::JsonParsingFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),

        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

pub async fn start(args: &Args) -> Result<(), ServerError> {

    // Enable tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a regular axum app.
    let app = Router::new()
        .nest("/server", request::router())
        .route("/slow", get(|| sleep(Duration::from_secs(5))))
        .route("/forever", get(std::future::pending::<()>))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(60)),
        ));

    let address: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
    tracing::trace!("start listening on {}", address);

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(address).await?;

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
