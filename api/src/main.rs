use std::{env, sync::Arc, time::Duration};

use auth::MicrosoftAuth;
use aws_config::{AppName, BehaviorVersion};
use axum::Router;
use education::EducationRoutable;
use jsonwebtoken::Validation;
use tower::ServiceBuilder;
use tower_http::{
    auth::AsyncRequireAuthorizationLayer, compression::CompressionLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub dynamo: Arc<aws_sdk_dynamodb::Client>,
}

pub const PORTFOLIO_TABLE: &'static str = "portfolio";

mod auth;
mod education;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = aws_config::load_defaults(BehaviorVersion::latest())
        .await
        .to_builder()
        .app_name(AppName::new("directory.justin.portfolio").expect("valid app name"))
        .build();

    let dynamo = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let auth = Arc::new(auth::AuthHandler::new(
        "https://login.microsoftonline.com/organizations/discovery/v2.0/keys",
        {
            let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
            validation.set_audience(&["11ec9395-8c5d-4ac7-9bc2-f4505e7053cf"]);
            validation
        },
    ));

    let router = Router::new()
        .route_education()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new().gzip(true))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(AsyncRequireAuthorizationLayer::new(MicrosoftAuth::new(
                    auth,
                ))),
        )
        .with_state(AppState { dynamo });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
