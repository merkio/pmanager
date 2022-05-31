use anyhow::Result;
use api::files::router;
use application::{DefaultFileService, CONTEXT};
use axum::{Router, Server};
use log::info;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "DEBUG".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let file_service = DefaultFileService::new(
        CONTEXT.resources.clone(),
        CONTEXT.storage.clone(),
        CONTEXT.config.aws.bucket.as_str(),
    );

    let app = Router::new()
        .merge(router(Box::new(file_service)))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    info!("Starting server...");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
