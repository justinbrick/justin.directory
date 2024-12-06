use std::{sync::Arc, time::Duration};

use axum::Router;
use education::EducationRoutable;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

mod auth;
mod education;
#[tokio::main]
async fn main() {
    // key discovery route "https://login.microsoftonline.com/organizations/discovery/v2.0/keys",

    tracing_subscriber::fmt::init();

    let router = Router::new().route_education().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new().gzip(true))
            .layer(TimeoutLayer::new(Duration::from_secs(30))),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
