use std::sync::Arc;

use auth::{authenticate, AuthHandler};
use axum::Router;
use education::EducationRoutable;

mod auth;
mod education;
#[tokio::main]
async fn main() {
    // key discovery route "https://login.microsoftonline.com/organizations/discovery/v2.0/keys",

    tracing_subscriber::fmt::init();

    let router = Router::new().route_education();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
